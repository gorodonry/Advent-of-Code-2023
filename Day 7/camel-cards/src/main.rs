use std::fs;
use std::cmp::Ordering;

const CARD_HIERARCHY: &'static [char] = &['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2', '1'];

fn main() {
    let file = fs::read_to_string("src/hands.txt").unwrap();
    let lines: Vec<&str> = file.split_terminator("\n").collect();

    let mut hands: Vec<Hand> = Vec::new();
    for line in lines.into_iter() {
        let hand: String = String::from(line.split(" ").collect::<Vec<&str>>()[0]);
        let bet = line.split(" ")
            .collect::<Vec<&str>>()[1]
            .parse::<u16>()
            .unwrap();

        // Determine hand type and add to collection.

        // Check for 3/4/5 of a kind and full house.
        match hand.chars().filter(|&c| c == sort_hand(&hand).chars().nth(2).unwrap()).collect::<Vec<char>>().len() {
            5 => {
                hands.push(Hand { hand, hand_type: HandType::FiveOfAKind, bet } );
                continue;
            },
            4 => {
                hands.push(Hand { hand, hand_type: HandType::FourOfAKind, bet } );
                continue;
            },
            3 => {
                let remaining = hand.chars()
                    .filter(|&c| c != sort_hand(&hand).chars().nth(2).unwrap())
                    .collect::<Vec<char>>();
                 
                if remaining[0] == remaining[1] {
                    hands.push(Hand { hand, hand_type: HandType::FullHouse, bet } );
                } else {
                    hands.push(Hand { hand, hand_type: HandType::ThreeOfAKind, bet } );
                }

                continue;
            },
            _ => ()
        }

        // Check for pairs.
        let mut pairs: Vec<char> = Vec::new();

        for c in hand.chars() {
            if hand.chars().filter(|&e| e == c).collect::<Vec<char>>().len() == 2 && !pairs.contains(&c) {
                pairs.push(c);
            }
        }

        match pairs.len() {
            2 => hands.push(Hand { hand, hand_type: HandType::TwoPair, bet } ),
            1 => hands.push(Hand { hand, hand_type: HandType::OnePair, bet } ),
            _ => hands.push(Hand { hand, hand_type: HandType::HighCard, bet } )
        }
    }

    hands.sort();

    let mut total_winnings: u32 = 0;
    for h in 0..hands.len() {
        total_winnings += hands.get(h).unwrap().bet as u32 * (hands.len() - h) as u32;
    }

    println!("{}", total_winnings);
}

fn sort_hand(hand: &String) -> String {
    let mut to_sort: Vec<char> = hand.chars().collect();

    to_sort.sort_by(|a, b| CARD_HIERARCHY.iter()
        .position(|&c| c == *a)
        .unwrap()
        .partial_cmp(&CARD_HIERARCHY.iter()
            .position(|&c| c == *b)
            .unwrap())
        .unwrap());

    to_sort.iter().collect::<String>()
}

#[derive(Debug)]
struct Hand {
    hand: String,
    hand_type: HandType,
    bet: u16
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        (&self.hand, &self.hand_type) == (&other.hand, &other.hand_type)
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type == other.hand_type {
            for c in 0..5 {
                if self.hand.chars().nth(c).unwrap() != other.hand.chars().nth(c).unwrap() {
                    return CARD_HIERARCHY.iter()
                        .position(|&e| e == self.hand.chars()
                            .nth(c)
                            .unwrap())
                        .cmp(&CARD_HIERARCHY.iter()
                            .position(|&e| e == other.hand.chars()
                                .nth(c)
                                .unwrap()));
                }
            }
        }

        self.hand_type.cmp(&other.hand_type)
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

