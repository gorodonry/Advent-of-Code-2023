use std::fs;

fn main() {
    let file = fs::read_to_string("src/platform.txt").unwrap();
    let rows: Vec<&str> = file.split_terminator("\n").collect();

    let mut columns: Vec<Vec<char>> = Vec::new();

    for i in 0..rows[0].len() {
        columns.push(rows.iter().map(|&row| row.chars().collect::<Vec<char>>()[i].clone()).collect::<Vec<char>>());
    }

    let mut load: u32 = 0;

    for mut column in columns.into_iter() {
        while !all_round_rocks_north(&column) {
            for i in 1..column.len() {
                if column[i] == 'O' && column[i - 1] == '.' {
                    column[i - 1] = 'O';
                    column[i] = '.';
                }
            }
        }

        for i in 0..column.len() {
            if column[i] == 'O' {
                load += (column.len() - i) as u32;
            }
        }
    }

    println!("{}", load);
}

fn all_round_rocks_north(vector: &Vec<char>) -> bool {
    for i in 1..vector.len() {
        if vector[i] == 'O' && !vec!['#', 'O'].contains(&vector[i - 1]) {
            return false;
        }
    }

    true
}
