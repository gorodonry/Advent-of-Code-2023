use std::fs;
use std::collections::HashMap;

fn main() {
    let file = fs::read_to_string("src/seed_map.txt").unwrap();
    let lines: Vec<&str> = file.split_terminator("\n").collect();

    let mut seed_locations: HashMap<u64, u64> = HashMap::new();
    let mut info_modified_for_current_map: HashMap<u64, bool> = HashMap::new();

    for line in lines.iter() {
        if line.is_empty() {
            continue;
        }

        // Record initial seeds.
        if line.contains("seeds") {
            let raw_data: Vec<&str> = line.split(":").collect::<Vec<&str>>()[1].split(" ").filter(|&s| !s.is_empty()).collect();
            for i in (0..(raw_data.len() / 2)).step_by(2) {
                let start = raw_data.get(i).unwrap().parse::<u64>().unwrap();
                let end = raw_data.get(i + 1).unwrap().parse::<u64>().unwrap();
                for j in 0..end {
                    seed_locations.insert(start + j, start + j);
                    info_modified_for_current_map.insert(start + j, false);
                }
            }

            continue;
        }

        // Set up for new conversion type.
        if line.contains("map") {
            for seed in info_modified_for_current_map.clone().keys() {
                info_modified_for_current_map.insert(*seed, false);
            }

            continue;
        }

        let raw_data: Vec<&str> = line.split(" ").collect();
        let destination_range_start = raw_data[0].parse::<u64>().unwrap();
        let source_range_start = raw_data[1].parse::<u64>().unwrap();
        let range_length = raw_data[2].parse::<u64>().unwrap();
        for (seed, factor) in seed_locations.clone().iter() {
            if *info_modified_for_current_map.get(seed).unwrap() {
                continue;
            }

            if *factor >= source_range_start && *factor < source_range_start + range_length {
                seed_locations.insert(*seed, destination_range_start + factor - source_range_start);
                info_modified_for_current_map.insert(*seed, true);
            }
        }
    }

    // Determine seed with closest planting location.
    let mut closest_seed: u64 = *seed_locations.keys().next().unwrap();
    for (seed, distance) in seed_locations.iter() {
        if distance < seed_locations.get(&closest_seed).unwrap() {
            closest_seed = *seed;
        }
    }

    println!("{}", seed_locations.get(&closest_seed).unwrap());
}

