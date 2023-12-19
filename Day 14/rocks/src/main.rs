use std::fs;

fn main() {
    let file = fs::read_to_string("src/platform.txt").unwrap();
    let mut platform: Vec<Vec<char>> = file
        .split_terminator("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&row| row.chars().collect::<Vec<char>>())
        .collect();

    let mut part_one_platform: Vec<Vec<char>> = Vec::new();

    for i in 0..platform[0].len() {
        part_one_platform.push(
            platform
                .iter()
                .map(|row| row[i].clone())
                .collect::<Vec<char>>(),
        );
    }

    let mut part_one_load: u32 = 0;

    for mut column in part_one_platform.into_iter() {
        while !all_round_rocks_north(&column) {
            roll_rocks(&mut column);
        }

        for i in 0..column.len() {
            if column[i] == 'O' {
                part_one_load += (column.len() - i) as u32;
            }
        }
    }

    let report = spin_cycle(&mut platform, 1000000000, true);
    spin_cycle(
        &mut platform,
        (1000000000 - report.cycles_processed) % report.repetition_cycle,
        false,
    );

    print_platform(&platform);

    let mut part_two_load: u32 = 0;

    for col in 0..platform[0].len() {
        let column = platform
            .iter()
            .map(|row| row[col].clone())
            .collect::<Vec<char>>();

        for i in 0..column.len() {
            if column[i] == 'O' {
                part_two_load += (column.len() - i) as u32;
            }
        }
    }

    println!("Part 1: {}", part_one_load);
    println!("Part 2: {}", part_two_load);
}

fn spin_cycle(
    platform: &mut Vec<Vec<char>>,
    cycles: usize,
    break_after_repitition: bool,
) -> SpinCycleReport {
    let mut cache: Vec<Vec<Vec<char>>> = Vec::new();
    let mut repetition_begins: usize = 0;
    let mut repetition_cycle: usize = 0;
    let mut cycles_processed: usize = 0;

    for cycle_no in 0..cycles {
        // Roll north.
        for i in 0..platform[0].len() {
            let mut column: Vec<char> = platform.iter().map(|row| row[i].clone()).collect();

            while !all_round_rocks_north(&column) {
                roll_rocks(&mut column);
            }

            for j in 0..platform.len() {
                platform[j][i] = column[j];
            }
        }

        // Roll west.
        for i in 0..platform.len() {
            let mut row = platform[i].clone();

            while !all_round_rocks_north(&row) {
                roll_rocks(&mut row);
            }

            platform[i] = row;
        }

        // Roll south.
        for i in 0..platform[0].len() {
            let mut column: Vec<char> = platform
                .iter()
                .map(|row| row[i].clone())
                .collect::<Vec<char>>()
                .iter()
                .rev()
                .map(|c| *c)
                .collect();

            while !all_round_rocks_north(&column) {
                roll_rocks(&mut column);
            }

            for j in 0..platform.len() {
                platform[j][i] = column.iter().rev().map(|c| *c).collect::<Vec<char>>()[j];
            }
        }

        // Roll east.
        for i in 0..platform.len() {
            let mut row: Vec<char> = platform[i].clone().iter().rev().map(|c| *c).collect();

            while !all_round_rocks_north(&row) {
                roll_rocks(&mut row);
            }

            platform[i] = row.iter().rev().map(|c| *c).collect::<Vec<char>>();
        }

        cycles_processed += 1;

        if repetition_begins == 0 && cache.contains(&platform) {
            cache.clear();
            repetition_begins = cycle_no;
        } else if repetition_cycle == 0 && cache.contains(&platform) {
            repetition_cycle = cycle_no - repetition_begins;

            if break_after_repitition {
                break;
            }
        }

        cache.push(platform.clone());
    }

    SpinCycleReport {
        cycles_processed,
        repetition_cycle,
    }
}

fn roll_rocks(vector: &mut Vec<char>) {
    for i in 1..vector.len() {
        if vector[i] == 'O' && vector[i - 1] == '.' {
            vector[i - 1] = 'O';
            vector[i] = '.';
        }
    }
}

fn all_round_rocks_north(vector: &Vec<char>) -> bool {
    for i in 1..vector.len() {
        if vector[i] == 'O' && !vec!['#', 'O'].contains(&vector[i - 1]) {
            return false;
        }
    }

    true
}

fn print_platform(platform: &Vec<Vec<char>>) {
    for line in platform.iter() {
        println!("{}", line.iter().collect::<String>());
    }

    println!("----------");
}

#[derive(Debug)]
struct SpinCycleReport {
    cycles_processed: usize,
    repetition_cycle: usize,
}
