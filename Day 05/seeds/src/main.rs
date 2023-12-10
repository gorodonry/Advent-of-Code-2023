use std::collections::HashMap;
use std::fs;

fn main() {
    let file = fs::read_to_string("src/seed_map.txt").unwrap();
    let lines: Vec<&str> = file.split_terminator("\n").collect();

    let mut seed_locations: HashMap<SeedRange, bool> = HashMap::new();

    for line in lines.iter() {
        if line.is_empty() {
            continue;
        }

        // Record initial seeds.
        if line.contains("seeds") {
            let raw_data: Vec<&str> = line.split(":").collect::<Vec<&str>>()[1]
                .split(" ")
                .filter(|&s| !s.is_empty())
                .collect();
            for i in (0..((raw_data.len() / 2) + 1)).step_by(2) {
                let start = raw_data.get(i).unwrap().parse::<u64>().unwrap();
                let range = raw_data.get(i + 1).unwrap().parse::<u64>().unwrap();

                let seed_range = SeedRange {
                    start,
                    end: start + range,
                };

                seed_locations.insert(seed_range, false);
            }

            continue;
        }

        // Set up for new conversion type.
        if line.contains("map") {
            for seed_range in seed_locations.clone().keys() {
                seed_locations.insert(seed_range.clone(), false);
            }

            continue;
        }

        let raw_data: Vec<&str> = line.split(" ").collect();
        let destination_range_start = raw_data[0].parse::<u64>().unwrap();
        let source_range_start = raw_data[1].parse::<u64>().unwrap();
        let range_length = raw_data[2].parse::<u64>().unwrap();
        for (seed_info, already_processed) in seed_locations.clone().iter() {
            if *already_processed {
                continue;
            }

            if source_range_start + range_length < seed_info.start
                || source_range_start > seed_info.end
            {
                // Source start is greater than seed end or source end is less than seed start.
                continue;
            } else if source_range_start < seed_info.start
                && source_range_start + range_length >= seed_info.end
            {
                // Source start is less than seed start and source end is greater than seed end.
                let offset = seed_info.start - source_range_start;

                let new_range = SeedRange {
                    start: destination_range_start + offset,
                    end: destination_range_start + offset + seed_info.end - seed_info.start,
                };

                seed_locations.insert(new_range, true);
            } else if source_range_start < seed_info.start {
                // Source start is less than seed start and source end is between seed start and end.
                let offset = seed_info.start - source_range_start;
                let difference = source_range_start + range_length - seed_info.start;

                let lower_subrange = SeedRange {
                    start: destination_range_start + offset,
                    end: destination_range_start + offset + difference,
                };

                let upper_subrange = SeedRange {
                    start: destination_range_start + offset + difference,
                    end: destination_range_start + offset + seed_info.end - seed_info.start,
                };

                seed_locations.insert(lower_subrange, true);
                seed_locations.insert(upper_subrange, true);
            } else if source_range_start + range_length >= seed_info.end {
                // Source start is greater than seed start and source end is greater than seed end.
                let difference = source_range_start - seed_info.start;

                let lower_subrange = SeedRange {
                    start: seed_info.start,
                    end: seed_info.start + difference,
                };

                let upper_subrange = SeedRange {
                    start: destination_range_start,
                    end: destination_range_start + seed_info.end - seed_info.start - difference,
                };

                seed_locations.insert(lower_subrange, true);
                seed_locations.insert(upper_subrange, true);
            } else {
                // Both source start and end are between seed start and end.
                let lower_difference = source_range_start - seed_info.start;
                let upper_difference = source_range_start + range_length - seed_info.start;

                let lower_subrange = SeedRange {
                    start: seed_info.start,
                    end: seed_info.start + lower_difference,
                };

                let middle_subrange = SeedRange {
                    start: destination_range_start,
                    end: destination_range_start + upper_difference - lower_difference,
                };

                let upper_subrange = SeedRange {
                    start: seed_info.start + upper_difference,
                    end: seed_info.end,
                };

                seed_locations.insert(lower_subrange, true);
                seed_locations.insert(middle_subrange, true);
                seed_locations.insert(upper_subrange, true);
            }

            seed_locations.remove(seed_info);
        }
    }

    let mut closest_range: SeedRange = seed_locations.keys().next().unwrap().clone();
    for seed_range in seed_locations.keys() {
        if seed_range.start < closest_range.start {
            closest_range = seed_range.clone();
        }
    }

    println!("{}", closest_range.start);
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct SeedRange {
    start: u64,
    end: u64,
}
