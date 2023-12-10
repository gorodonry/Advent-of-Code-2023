use std::collections::HashMap;
use std::fs;

fn main() {
    let file = fs::read_to_string("src/scratchcards.txt").unwrap();
    let cards: Vec<&str> = file.split("\n").filter(|&c| !c.is_empty()).collect();

    let mut total_cards: u64 = 0;
    let mut total_score: u32 = 0;
    let mut copies: HashMap<u32, u32> = HashMap::new();

    for c in 0..cards.len() {
        copies.insert(c as u32, 1);
    }

    for (card_index, card) in cards.iter().enumerate() {
        let mut winning_numbers: Vec<u8> = Vec::new();
        let mut elf_numbers: Vec<u8> = Vec::new();

        {
            let raw_winning_data: Vec<&str> = card.split("|").collect::<Vec<&str>>()[0]
                .split(":")
                .collect::<Vec<&str>>()[1]
                .split(" ")
                .filter(|&n| !n.is_empty())
                .collect();
            for number in raw_winning_data.iter() {
                winning_numbers.push(number.parse::<u8>().unwrap());
            }

            let raw_elf_data: Vec<&str> = card.split("|").collect::<Vec<&str>>()[1]
                .split(" ")
                .filter(|&n| !n.is_empty())
                .collect();
            for number in raw_elf_data.iter() {
                elf_numbers.push(number.parse::<u8>().unwrap());
            }
        }

        let mut score: u16 = 0;
        let mut matches: u8 = 0;
        for entry in elf_numbers.iter() {
            if winning_numbers.contains(entry) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }

                matches += 1;
            }
        }

        total_score += score as u32;

        for _ in 0..(*copies.get(&(card_index as u32)).unwrap() as usize) {
            for m in 0..matches {
                copies.insert(
                    (card_index + m as usize + 1) as u32,
                    (copies.get(&((card_index + m as usize + 1) as u32)).unwrap() + 1) as u32,
                );
            }
        }

        total_cards += *copies.get(&(card_index as u32)).unwrap() as u64;
    }

    println!("Part 1: {}", total_score);
    println!("Part 2: {}", total_cards);
}
