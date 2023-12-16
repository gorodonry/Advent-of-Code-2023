use std::collections::HashMap;
use std::fs;

fn main() {
    let file = fs::read_to_string("src/sequence.txt").unwrap();
    let sequence = file.split(",").collect::<Vec<&str>>();

    let mut part_one_total: u32 = 0;
    let mut boxes: HashMap<u16, Vec<Lens>> = HashMap::new();

    for part in sequence.into_iter() {
        part_one_total += hash(&part) as u32;

        let action = part
            .chars()
            .filter(|&c| vec!['-', '='].contains(&c))
            .collect::<Vec<char>>()[0];
        let label: String = part.chars().collect::<Vec<char>>()
            [..part.chars().position(|c| c == action).unwrap()]
            .iter()
            .collect::<String>();
        let focal_length: Option<u8>;

        if action == '=' {
            focal_length = Some(
                String::from(
                    part.chars().collect::<Vec<char>>()
                        [part.chars().position(|c| c == action).unwrap() + 1],
                )
                .parse::<u8>()
                .unwrap(),
            );
        } else {
            focal_length = None;
        }

        let hash = hash(&label);
        let index: Option<usize>;

        if boxes.contains_key(&hash) {
            index = boxes[&hash].iter().position(|lens| lens.label == label);
        } else if action == '=' {
            boxes.insert(hash, Vec::new());
            index = None;
        } else {
            index = None;
        }

        match action {
            '=' => {
                let new_lens = Lens {
                    label,
                    focal_length: focal_length.unwrap(),
                };
                if index.is_some() {
                    boxes.get_mut(&hash).unwrap()[index.unwrap()] = new_lens;
                } else {
                    boxes.get_mut(&hash).unwrap().push(new_lens);
                }
            }
            '-' => {
                if index.is_some() {
                    boxes.get_mut(&hash).unwrap().remove(index.unwrap());
                }
            }
            _ => panic!("Shit is so broken"),
        }
    }

    let mut part_two_total: u32 = 0;

    for box_number in boxes.keys() {
        for slot in 0..boxes[box_number].len() {
            part_two_total += u32::from(
                (box_number + 1)
                    * (slot as u16 + 1)
                    * (boxes[box_number][slot].focal_length as u16),
            );
        }
    }

    println!("Part 1: {}", part_one_total);
    println!("Part 2: {}", part_two_total);
}

fn hash(string: &str) -> u16 {
    let string = string.replace("\n", "");
    let mut result: u16 = 0;

    for code in string.as_bytes().into_iter() {
        result += *code as u16;
        result *= 17;
        result %= 256;
    }

    result
}

#[derive(Debug)]
struct Lens {
    label: String,
    focal_length: u8,
}
