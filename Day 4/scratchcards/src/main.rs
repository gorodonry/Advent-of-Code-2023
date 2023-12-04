use std::fs;

fn main() {
    let file = fs::read_to_string("src/scratchcards.txt").unwrap();
    let cards: Vec<&str> = file.split("\n").filter(|&c| !c.is_empty()).collect();

    let mut total: u32 = 0;
    for card in cards.iter() {
        let mut winning_numbers: Vec<u8> = Vec::new();
        let mut elf_numbers: Vec<u8> = Vec::new();

        {
            let raw_winning_data: Vec<&str> = card.split("|")
                .collect::<Vec<&str>>()[0]
                .split(":")
                .collect::<Vec<&str>>()[1]
                .split(" ")
                .filter(|&n| !n.is_empty())
                .collect();
            for number in raw_winning_data.iter() {
                winning_numbers.push(number.parse::<u8>().unwrap());
            }

            let raw_elf_data: Vec<&str> = card.split("|")
                .collect::<Vec<&str>>()[1]
                .split(" ")
                .filter(|&n| !n.is_empty())
                .collect();
            for number in raw_elf_data.iter() {
                elf_numbers.push(number.parse::<u8>().unwrap());
            }
        }

        let mut score: u16 = 0;
        for entry in elf_numbers.iter() {
            if winning_numbers.contains(entry) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }

        total += score as u32;
    }

    println!("{}", total);
}

