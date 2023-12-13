use std::fs;

fn main() {
    let file = fs::read_to_string("src/notes.txt").unwrap();
    let lines: Vec<&str> = file.split_terminator("\n").collect();

    let mut current_pattern: Vec<&str> = Vec::new();
    let mut total: u32 = 0;

    for line in lines.into_iter() {
        if line.is_empty() {
            total += get_pattern_summary(&current_pattern);

            current_pattern.clear();
        } else {
            current_pattern.push(line);
        }
    }

    if !current_pattern.is_empty() {
        total += get_pattern_summary(&current_pattern);
    }

    println!("{}", total);
}

fn get_pattern_summary(pattern: &Vec<&str>) -> u32 {
    let mut summary: u32 = 0;

    let mut row_match_indices: Vec<usize> = Vec::new();

    for i in 0..(pattern.len() - 1) {
        if pattern[i] == pattern[i + 1] {
            let row_match_index = i;

            if row_match_index == 0 {
                row_match_indices.push(row_match_index);
            }

            for j in (0..row_match_index).rev() {
                let corresponding_index = row_match_index * 2 - j + 1;

                if corresponding_index >= pattern.len() {
                    row_match_indices.push(row_match_index);
                    break;
                }

                if pattern[j] != pattern[corresponding_index] {
                    break;
                } else if j == 0 {
                    row_match_indices.push(row_match_index);
                }
            }
        }
    }

    let mut column_match_indices: Vec<usize> = Vec::new();

    for i in 0..(pattern[0].len() - 1) {
        if pattern
            .iter()
            .map(|&row| row.chars().collect::<Vec<char>>()[i])
            .collect::<String>()
            == pattern
                .iter()
                .map(|&row| row.chars().collect::<Vec<char>>()[i + 1])
                .collect::<String>()
        {
            let column_match_index = i;

            if column_match_index == 0 {
                column_match_indices.push(column_match_index);
            }

            for j in (0..column_match_index).rev() {
                let corresponding_index = column_match_index * 2 - j + 1;

                if corresponding_index >= pattern[0].len() {
                    column_match_indices.push(column_match_index);
                    break;
                }

                if pattern
                    .iter()
                    .map(|&row| row.chars().collect::<Vec<char>>()[j])
                    .collect::<String>()
                    != pattern
                        .iter()
                        .map(|&row| row.chars().collect::<Vec<char>>()[corresponding_index])
                        .collect::<String>()
                {
                    break;
                } else if j == 0 {
                    column_match_indices.push(column_match_index);
                }
            }
        }
    }

    for index in row_match_indices.into_iter() {
        summary += 100 * (index + 1) as u32;
    }

    for index in column_match_indices.into_iter() {
        summary += index as u32 + 1;
    }

    summary
}
