use std::fs;
use std::collections::HashMap;

fn main() {
    let part_two_time: u64;
    let part_two_record: u64;
    let mut times_and_records: HashMap<u16, u16> = HashMap::new();

    {
        let file = fs::read_to_string("src/races.txt").unwrap();
        let lines: Vec<&str> = file.split_terminator("\n").collect();

        let raw_time_data: &str = lines[0].split(":").collect::<Vec<&str>>()[1];
        let raw_record_data: &str = lines[1].split(":").collect::<Vec<&str>>()[1];

        let times: Vec<&str> = raw_time_data.split(" ").filter(|&t| !t.is_empty()).collect();
        let distances: Vec<&str> = raw_record_data.split(" ").filter(|&d| !d.is_empty()).collect();
        
        for i in 0..times.len() {
            times_and_records.insert(times[i].parse::<u16>().unwrap(), distances[i].parse::<u16>().unwrap());
        }

        part_two_time = raw_time_data.split(" ")
            .filter(|&ts| !ts.is_empty())
            .collect::<Vec<&str>>()
            .join("")
            .parse::<u64>()
            .unwrap();
        part_two_record = raw_record_data.split(" ")
            .filter(|&rs| !rs.is_empty())
            .collect::<Vec<&str>>()
            .join("")
            .parse::<u64>()
            .unwrap();
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

    let mut ways_of_winning: u32 = 0;
    for hold in 0..(part_two_time + 1) {
        let distance = hold * (part_two_time - hold);
        if distance > part_two_record {
            ways_of_winning += 1;
        }
    }

    println!("Part 1: {}", margin_of_error);
    println!("Part 2: {}", ways_of_winning);
}

