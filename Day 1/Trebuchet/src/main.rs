use std::fs;

fn main() {
    let file: String = fs::read_to_string("src/calibration_codes.txt").unwrap();
    let input: Vec<&str> = file.split_terminator("\n").collect();

    let mut codes: Vec<u32> = Vec::new();
    for string in input.iter() {
        let mut code: String = String::new();

        for c in string.chars() {
            if c.is_numeric() {
                code.push(c);
                break;
            }
        }

        for c in string.chars().rev() {
            if c.is_numeric() {
                code.push(c);
                break;
            }
        }
        
        codes.push(code.parse::<u32>().unwrap());
    }

    let sum: u32 = codes.iter().sum();

    println!("{}", sum);
}
