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
        let mut valid_arrangements: u16 = 0;

        for arrangement in get_arrangements(&springs).iter() {
            let mut broken_amounts_found: Vec<u8> = Vec::new();
            let mut current_broken_sequence_length: u8 = 0;

            for c in arrangement.iter() {
                if *c == '#' {
                    current_broken_sequence_length += 1;
                } else if current_broken_sequence_length != 0 {
                    broken_amounts_found.push(current_broken_sequence_length);
                    current_broken_sequence_length = 0;
                }
            }

            if current_broken_sequence_length != 0 {
                broken_amounts_found.push(current_broken_sequence_length);
            }

            if broken_amounts[..] == broken_amounts_found[..] {
                valid_arrangements += 1;
            }
        }

        total += valid_arrangements as u32;
    }

    println!("{}", total);
}

fn get_arrangements(springs: &Vec<char>) -> Vec<Vec<char>> {
    let mut arrangements: Vec<Vec<char>> = Vec::new();
    let mut unknown_index: i16 = -1;

    for i in 0..springs.len() {
        if springs[i] == '?' {
            unknown_index = i as i16;
            break;
        }
    }

    if unknown_index == -1 {
        arrangements.push(springs.clone());
    } else {
        let mut prefix: Vec<char> = springs[..=unknown_index as usize].to_vec();
        for spring_type in vec!['.', '#'].iter() {
            prefix[unknown_index as usize] = *spring_type;
            for suffix in get_arrangements(&springs[(unknown_index as usize + 1)..].to_vec()).iter()
            {
                let mut arrangement = prefix.clone();
                arrangement.extend(suffix);
                arrangements.push(arrangement);
            }
        }
    }

    arrangements
}
