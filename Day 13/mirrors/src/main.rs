use std::fs;

fn main() {
    let file = fs::read_to_string("src/notes.txt").unwrap();
    let lines: Vec<&str> = file.split_terminator("\n").collect();

    let mut current_pattern: Vec<&str> = Vec::new();
    let mut part_one_total: u32 = 0;
    let mut part_two_total: u32 = 0;

    for line in lines.into_iter() {
        if line.is_empty() {
            let part_one_summary = get_pattern_summary(&current_pattern, &Vec::new());
            part_one_total += part_one_summary.value;
            part_two_total += get_smudge_summary(&current_pattern, &part_one_summary);

            current_pattern.clear();
        } else {
            current_pattern.push(line);
        }
    }

    if !current_pattern.is_empty() {
        let part_one_summary = get_pattern_summary(&current_pattern, &Vec::new());
        part_one_total += part_one_summary.value;
        part_two_total += get_smudge_summary(&current_pattern, &part_one_summary);
    }

    println!("Part 1: {}", part_one_total);
    println!("Part 2: {}", part_two_total);
}

fn get_smudge_summary(current_pattern: &Vec<&str>, original_summary: &Summary) -> u32 {
    let mut discount_summaries: Vec<Summary> = Vec::from([original_summary.clone()]);
    let mut smudge_summary: u32 = 0;

    for i in 0..current_pattern.len() {
        for j in 0..current_pattern[0].len() {
            let mut mutant = current_pattern.clone();
            let mut row: Vec<char> = current_pattern[i].chars().collect();

            match row[j] {
                '.' => row[j] = '#',
                '#' => row[j] = '.',
                _ => panic!("This shouldn't be happening........")
            }

            let new_row = row.iter().collect::<String>();

            mutant[i] = new_row.as_str();

            let summary = get_pattern_summary(&mutant, &discount_summaries);

            smudge_summary += summary.value;
            discount_summaries.push(summary);
        }
    }

    smudge_summary
}

fn get_pattern_summary(pattern: &Vec<&str>, discount_summaries: &Vec<Summary>) -> Summary {
    let mut summary: u32 = 0;

    let mut row_match_indices: Vec<usize> = Vec::new();

    'row_loop: for i in 0..(pattern.len() - 1) {
        if pattern[i] == pattern[i + 1] {
            for discount_summary in discount_summaries.iter() {
                if discount_summary.row_match_indices.contains(&i) {
                    continue 'row_loop;
                }
            }

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

    'column_loop: for i in 0..(pattern[0].len() - 1) {
        if pattern
            .iter()
            .map(|&row| row.chars().collect::<Vec<char>>()[i])
            .collect::<String>()
            == pattern
                .iter()
                .map(|&row| row.chars().collect::<Vec<char>>()[i + 1])
                .collect::<String>()
        {
            for discount_summary in discount_summaries.iter() {
                if discount_summary.column_match_indices.contains(&i) {
                    continue 'column_loop;
                }
            }

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

    for index in row_match_indices.iter() {
        summary += 100 * (index + 1) as u32;
    }

    for index in column_match_indices.iter() {
        summary += *index as u32 + 1;
    }

    Summary { value: summary, row_match_indices, column_match_indices }
}

#[derive(Debug, Clone)]
struct Summary {
    value: u32,
    row_match_indices: Vec<usize>,
    column_match_indices: Vec<usize>
}

impl PartialEq for Summary {
    fn eq(&self, other: &Self) -> bool {
        (&self.value, &self.row_match_indices[..], &self.column_match_indices[..]) == (&other.value, &other.row_match_indices[..], &other.column_match_indices[..])
    }
}

impl Eq for Summary {}
