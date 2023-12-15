use std::fs;

fn main() {
    let file = fs::read_to_string("src/test.txt").unwrap();
    let mut platform: Vec<Vec<char>> = file.split_terminator("\n").collect::<Vec<&str>>().iter().map(|&row| row.chars().collect::<Vec<char>>()).collect();

    let mut columns: Vec<Vec<char>> = Vec::new();

    for i in 0..platform[0].len() {
        columns.push(platform.iter().map(|row| row[i].clone()).collect::<Vec<char>>());
    }

    let mut part_one_load: u32 = 0;

    for mut column in columns.into_iter() {
        while !all_round_rocks_north(&column) {
            roll_rocks(&mut column);
        }

        for i in 0..column.len() {
            if column[i] == 'O' {
                part_one_load += (column.len() - i) as u32;
            }
        }
    }

    for k in 0..1000000000 {
        println!("{}", k);
        
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
            let mut column: Vec<char> = platform.iter().map(|row| row[i].clone()).collect::<Vec<char>>().iter().rev().map(|c| *c).collect();

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
    }

    let mut part_two_load: u32 = 0;
    for vector in platform.iter() {
        for i in 0..vector.len() {
            if vector[i] == 'O' {
                part_two_load += (vector.len() - i) as u32;
            }
        }
    }

    println!("Part 1: {}", part_one_load);
    println!("Part 2: {}", part_two_load);
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
