use std::fs;
use std::collections::HashSet;

fn main() {
    let file = fs::read_to_string("src/map.txt").unwrap();
    let map: Vec<Vec<char>> = file.split_terminator("\n").collect::<Vec<&str>>().iter().map(|&line| line.chars().collect::<Vec<char>>()).collect();
    
    let mut current_locations: HashSet<Location> = HashSet::new();
    
    'find_start_loop: for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == 'S' {
                current_locations.insert(Location { row, col });
                break 'find_start_loop;
            }
        }
    }
    
    for _ in 0..64 {
        let mut new_locations: HashSet<Location> = HashSet::new();
        
        for location in current_locations.iter() {
            for adj_location in get_adjacent_locations(location, map.len(), map[0].len()).into_iter() {
                if map[adj_location.row][adj_location.col] != '#' {
                    new_locations.insert(adj_location);
                }
            }
        }
        
        current_locations.clear();
        current_locations = new_locations;
    }
    
    println!("{}", current_locations.len());
}

fn get_adjacent_locations(location: &Location, num_rows: usize, num_cols: usize) -> Vec<Location> {
    let mut adjacent_locations: Vec<Location> = Vec::new();
    
    if location.row > 0 {
        adjacent_locations.push(Location { row: location.row - 1, col: location.col });
    }
    
    if location.col > 0 {
        adjacent_locations.push(Location { row: location.row, col: location.col - 1 });
    }
    
    if location.row < num_rows - 1 {
        adjacent_locations.push(Location { row: location.row + 1, col: location.col });
    }
    
    if location.col < num_cols - 1 {
        adjacent_locations.push(Location { row: location.row, col: location.col + 1 });
    }
    
    adjacent_locations
}

#[derive(PartialEq, Eq, Hash)]
struct Location {
    row: usize,
    col: usize
}