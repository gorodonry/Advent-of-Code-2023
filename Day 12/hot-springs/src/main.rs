use std::collections::HashMap;
use std::fs;

fn main() {
    let file = fs::read_to_string("src/data.txt").unwrap();
    let rows: Vec<&str> = file.split_terminator("\n").collect();

    let mut total: u32 = 0;

    for row in rows.into_iter() {
        let springs: Vec<char> = row.split(" ").collect::<Vec<&str>>()[0].chars().collect();
        let broken_amounts: Vec<u8> = row.split(" ").collect::<Vec<&str>>()[1]
            .split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|&e| e.parse::<u8>().unwrap())
            .collect();

        total += count_arrangements(&springs, &broken_amounts, true);
    }

    println!("{}", total);
}

fn count_arrangements(
    springs: &Vec<char>,
    broken_sections: &Vec<u8>,
    next_spring_can_be_broken: bool,
) -> u32 {
    let mut result: u32 = 0;

    if springs.len() == 0 {
        if broken_sections.len() == 0 {
            return 1;
        } else {
            return 0;
        }
    }

    if broken_sections.len() == 0 {
        if springs.contains(&'#') {
            return 0;
        } else {
            return 1;
        }
    }

    if broken_sections.len() == 1 && springs.len() == broken_sections[0] as usize {
        if springs.contains(&'.') || !next_spring_can_be_broken {
            return 0;
        } else {
            return 1;
        }
    }

    // First entry is a working spring.
    if vec!['.', '?'].contains(&springs[0]) {
        result += count_arrangements(&springs[1..].to_vec(), broken_sections, true);
    }

    // First entry is a broken spring.
    if next_spring_can_be_broken
        && vec!['#', '?'].contains(&springs[0])
        && springs.len() > broken_sections[0] as usize
    {
        // If a . is in the arrangement in violation of the number of consecutive broken springs, we can safely ignore it.
        let broken_section_length = broken_sections[0] as usize;
        if !springs[..broken_section_length].contains(&'.') {
            result += count_arrangements(
                &springs[broken_section_length..].to_vec(),
                &broken_sections[1..].to_vec(),
                false,
            );
        }
    }

    result
}

fn multiply_input(input: &Vec<char>, amount: usize) -> Vec<char> {
    let mut output: Vec<char> = Vec::new();

    if amount == 0 {
        return output;
    }

    for _ in 0..(amount - 1) {
        output.extend(input.clone());
        output.push('?');
    }

    output.extend(input.clone());

    output
}
