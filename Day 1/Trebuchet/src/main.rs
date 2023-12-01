use std::fs;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref NUMBERS: HashMap<&'static str, u32> = {
        let mut map = HashMap::new();
        map.insert("one", 1);
        map.insert("two", 2);
        map.insert("three", 3);
        map.insert("four", 4);
        map.insert("five", 5);
        map.insert("six", 6);
        map.insert("seven", 7);
        map.insert("eight", 8);
        map.insert("nine", 9);
        map
    };
}

fn main() {
    let file: String = fs::read_to_string("src/calibration_codes.txt").unwrap();
    let input: Vec<&str> = file.split_terminator("\n").collect();

    let mut codes: Vec<u32> = Vec::new();
    for string in input.iter() {
        let mut code: String = String::new();

        let mut word_form_present: bool;
        let non_numeric_segments: Vec<&str> = string.split(char::is_numeric).filter(|&s| !s.is_empty()).collect();

        word_form_present = false;

        if non_numeric_segments.len() != 0 && !string.chars().nth(0).unwrap().is_numeric() {
            'outer: for c in 0..non_numeric_segments[0].len() {
                let mut potential_number = String::from(non_numeric_segments[0].chars().nth(c).unwrap());
                
                for remaining in (c + 1)..non_numeric_segments[0].len() {
                    potential_number.push(non_numeric_segments[0].chars().nth(remaining).unwrap());
                    if NUMBERS.contains_key(&potential_number as &str) {
                        code.push(NUMBERS.get(&potential_number as &str).unwrap().to_string().chars().nth(0).unwrap());
                        word_form_present = true;
                        break 'outer;
                    }
                }
            }
        }

        if !word_form_present {
            for c in string.chars() {
                if c.is_numeric() {
                    code.push(c);
                    break;
                }
            }
        }

        word_form_present = false;

        if non_numeric_segments.len() != 0 && !string.chars().nth(string.len() - 1).unwrap().is_numeric() {
            'outer: for c in (0..non_numeric_segments[non_numeric_segments.len() - 1].len()).rev() {
                let mut potential_number = String::from(non_numeric_segments[non_numeric_segments.len() - 1].chars().nth(c).unwrap());

                for remaining in (c + 1)..non_numeric_segments[non_numeric_segments.len() - 1].len() {
                    potential_number.push(non_numeric_segments[non_numeric_segments.len() - 1].chars().nth(remaining).unwrap());
                    if NUMBERS.contains_key(&potential_number as &str) {
                        code.push(NUMBERS.get(&potential_number as &str).unwrap().to_string().chars().nth(0).unwrap());
                        word_form_present = true;
                        break 'outer;
                    }
                }
            }
        }

        if !word_form_present {
            for c in string.chars().rev() {
                if c.is_numeric() {
                    code.push(c);
                    break;
                }
            }
        }
        
        codes.push(code.parse::<u32>().unwrap()); 
    }

    let sum: u32 = codes.iter().sum();

    println!("{}", sum);
}
