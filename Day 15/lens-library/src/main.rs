use std::fs;

fn main() {
    let file = fs::read_to_string("src/sequence.txt").unwrap();
    let sequence = file.split(",").collect::<Vec<&str>>();

    let mut total: u32 = 0;

    for part in sequence.into_iter() {
        let part = part.replace("\n", "");
        let mut result: u16 = 0;

        for code in part.as_bytes().into_iter() {
            result += *code as u16;
            result *= 17;
            result %= 256;
        }

        total += result as u32;
    }

    println!("{}", total);
}
