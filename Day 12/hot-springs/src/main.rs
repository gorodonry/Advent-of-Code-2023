use std::collections::HashMap;
use std::fs;

fn main() {
    let file = fs::read_to_string("src/test.txt").unwrap();
    let rows: Vec<&str> = file.split_terminator("\n").collect();

    let mut part_one_total: u32 = 0;
    let mut part_two_total: u32 = 0;

    for row in rows.into_iter() {
        let mut cache: HashMap<String, u32>;

        let part_one_springs: Vec<char> = row.split(" ").collect::<Vec<&str>>()[0]
            .chars()
            .collect::<Vec<char>>();
        let part_one_broken_amounts: Vec<u8> = row.split(" ").collect::<Vec<&str>>()[1]
            .split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|&e| e.parse::<u8>().unwrap())
            .collect();

        println!("{}", row);
        println!("{}", part_one_springs.iter().collect::<String>());

        cache = HashMap::new();
        part_one_total += count_arrangements(&part_one_springs, &part_one_broken_amounts, true, &mut cache);

        let part_two_springs = multiply_vector_with_separator(&part_one_springs, '?', 5);
        let part_two_broken_amounts = multiply_vector(&part_one_broken_amounts, 5);

        cache = HashMap::new();
        part_two_total += count_arrangements(&part_two_springs, &part_two_broken_amounts, true, &mut cache);
    }

    println!("Part 1: {}", part_one_total);
    println!("Part 2: {}", part_two_total);
}

fn count_arrangements(
    springs: &Vec<char>,
    broken_sections: &Vec<u8>,
    next_spring_can_be_broken: bool,
    cache: &mut HashMap<String, u32>
) -> u32 {
    let mut result: u32 = 0;

    let cache_representation = get_cache_representation(springs, broken_sections);

    if cache.contains_key(&cache_representation) {
        //return *cache.get(&cache_representation).unwrap();
    }

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
        result += count_arrangements(&springs[1..].to_vec(), broken_sections, true, cache);
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
                false, cache
            );
        }
    }

    cache.insert(cache_representation, result);

    result
}

fn get_cache_representation(springs: &Vec<char>, broken_sections: &Vec<u8>) -> String {
    springs.iter().collect::<String>() + &broken_sections.iter().map(|&s| s.to_string()).collect::<Vec<String>>().join(",")
}

fn multiply_vector<T: std::clone::Clone>(input: &Vec<T>, amount: usize) -> Vec<T> {
    let mut output: Vec<T> = Vec::new();

    for _ in 0..amount {
        output.extend(input.clone());
    }

    output
}

fn multiply_vector_with_separator<T: std::clone::Clone>(
    input: &Vec<T>,
    separator: T,
    amount: usize,
) -> Vec<T> {
    let mut output: Vec<T> = Vec::new();

    if amount == 0 {
        return output;
    }

    for _ in 0..(amount - 1) {
        output.extend(input.clone());
        output.push(separator.clone());
    }

    output.extend(input.clone());

    output
}
