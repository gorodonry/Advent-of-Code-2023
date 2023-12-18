use std::fs;
use std::collections::HashSet;

fn main() {
    let file = fs::read_to_string("src/instructions.txt").unwrap();
    let instructions: Vec<&str> = file.split_terminator("\n").collect();

    let mut map: Vec<Vec<char>> = Vec::from(vec![vec!['#']]);
    let mut number_of_edges: u16 = 0;

    let mut current_location = Location { row: 0, col: 0 };

    // Determine outline.
    for line in instructions.into_iter() {
        let direction = line.split(" ").collect::<Vec<&str>>()[0];
        let length = line.split(" ").collect::<Vec<&str>>()[1].parse::<u8>().unwrap();

        match direction {
            "U" => {
                for row in 1..=length {
                    if (current_location.row as isize - row as isize) < 0 {
                        insert_rows(&mut map, 1, 0);
                        current_location.row += 1;
                    }

                    map[current_location.row - row as usize][current_location.col] = '#';
                }

                current_location.row -= length as usize;
            },
            "D" => {
                let num_rows = map.len();

                let mut num_rows_inserted: u8 = 0;

                for row in 1..=length {
                    if current_location.row + row as usize >= num_rows {
                        insert_rows(&mut map, 1, num_rows + num_rows_inserted as usize);
                        num_rows_inserted += 1;
                    }

                    map[current_location.row + row as usize][current_location.col] = '#';
                }

                current_location.row += length as usize;
            },
            "L" => {
                for col in 1..=length {
                    if (current_location.col as isize - col as isize) < 0 {
                        insert_cols(&mut map, 1, 0);
                        current_location.col += 1;
                    }

                    map[current_location.row][current_location.col - col as usize] = '#';
                }

                current_location.col -= length as usize;
            },
            "R" => {
                let num_cols: usize;

                match map.get(0) {
                    Some(val) => num_cols = val.len(),
                    None => num_cols = 0
                }

                let mut num_cols_inserted: u8 = 0;

                for col in 1..=length {
                    if current_location.col + col as usize >= num_cols {
                        insert_cols(&mut map, 1, num_cols + num_cols_inserted as usize);
                        num_cols_inserted += 1;
                    }

                    map[current_location.row][current_location.col + col as usize] = '#';
                }

                current_location.col += length as usize;
            },
            _ => panic!("Well that's... interesting")
        }

        number_of_edges += length as u16;
    }

    // Determine inner area.
    let mut all_traversed_locations: HashSet<Location> = HashSet::new();
    let mut this_traversed_locations: HashSet<Location> = HashSet::new();
    let mut for_traversal: Vec<Location> = Vec::new();
    let mut inner_area: u32 = 0;

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == '#' {
                continue;
            }

            if all_traversed_locations.contains(&Location { row, col }) {
                continue;
            }

            for_traversal.push(Location { row, col });

            let mut external = false;

            while !for_traversal.is_empty() {
                current_location = for_traversal.pop().unwrap();

                for neighbour in get_adjacent_locations(&current_location) {
                    if this_traversed_locations.contains(&neighbour) {
                        continue;
                    }

                    if neighbour.row >= map.len() || neighbour.col >= map[0].len() {
                        continue;
                    }

                    if map[neighbour.row][neighbour.col] == '#' {
                        continue;
                    }

                    if neighbour.row == 0 || neighbour.col == 0 || neighbour.row == map.len() - 1 || neighbour.col == map[0].len() - 1 {
                        external = true;
                    }

                    for_traversal.push(neighbour);
                }

                this_traversed_locations.insert(current_location.clone());
            }

            if !external {
                inner_area += this_traversed_locations.len() as u32;
            }

            all_traversed_locations.extend(this_traversed_locations);
            this_traversed_locations = HashSet::new();
        }
    }

    for line in map.iter() {
        println!("{}", line.iter().collect::<String>());
    }

    println!();

    println!("Part 1: {}", inner_area + number_of_edges as u32);
}

fn get_adjacent_locations(location: &Location) -> Vec<Location> {
    let mut adjacent_locations: Vec<Location> = Vec::new();

    if location.row > 0 {
        adjacent_locations.push(Location { row: location.row - 1, col: location.col });
    }

    adjacent_locations.push(Location { row: location.row + 1, col: location.col });

    if location.col > 0 {
        adjacent_locations.push(Location { row: location.row, col: location.col - 1 });
    }

    adjacent_locations.push(Location { row: location.row, col: location.col + 1 });

    adjacent_locations
}

fn insert_rows(map: &mut Vec<Vec<char>>, number_of_rows: usize, below_index: usize) {
    let cols: usize;

    match map.get(0) {
        Some(val) => cols = val.len(),
        None => cols = 0
    }

    for _ in 0..number_of_rows {
        map.insert(below_index, vec!['.'; cols]);
    }
}

fn insert_cols(map: &mut Vec<Vec<char>>, number_of_cols: usize, below_index: usize) {
    for _ in 0..number_of_cols {
        for row in 0..map.len() {
            map[row].insert(below_index, '.');
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Location {
    row: usize,
    col: usize
}
