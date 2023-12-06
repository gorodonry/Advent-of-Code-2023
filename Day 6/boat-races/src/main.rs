use std::fs;
use std::collections::HashMap;

fn main() {
    let mut times_and_records: HashMap<u16, u16> = HashMap::new();

    {
        let file = fs::read_to_string("src/races.txt").unwrap();
        let lines: Vec<&str> = file.split_terminator("\n").collect();

        let times: Vec<&str> = lines[0].split(":").collect::<Vec<&str>>()[1].split(" ").filter(|&t| !t.is_empty()).collect();
        let distances: Vec<&str> = lines[1].split(":").collect::<Vec<&str>>()[1].split(" ").filter(|&d| !d.is_empty()).collect();
        
        for i in 0..times.len() {
            times_and_records.insert(times[i].parse::<u16>().unwrap(), distances[i].parse::<u16>().unwrap());
        }
    }

    let mut margin_of_error: u32 = 0;
    for (time, record) in times_and_records.iter() {
        let mut ways_of_winning: u16 = 0;
        for hold in 0..(time + 1) {
            let distance = hold * (time - hold);
            if distance > *record {
                ways_of_winning += 1;
            }
        }

        if margin_of_error == 0 {
            margin_of_error += ways_of_winning as u32;
        } else {
            margin_of_error *= ways_of_winning as u32;
        }
    }

    println!("{}", margin_of_error);
}

