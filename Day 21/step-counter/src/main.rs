use std::fs;
use std::collections::HashSet;

fn main() {
    let file = fs::read_to_string("src/test.txt").unwrap();
    let map: Vec<Vec<char>> = file.split_terminator("\n").collect::<Vec<&str>>().iter().map(|&line| line.chars().collect::<Vec<char>>()).collect();
    
    let mut current_locations: HashSet<InfiniteMapLocation> = HashSet::new();
    
    'find_start_loop: for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == 'S' {
                current_locations.insert(InfiniteMapLocation { row: row as isize, col: col as isize , num_rows: map.len(), num_cols: map[0].len() });
                break 'find_start_loop;
            }
        }
    }
    
    for _ in 0..500 {
        let mut new_locations: HashSet<InfiniteMapLocation> = HashSet::new();
        
        for location in current_locations.iter() {
            for adj_location in get_adjacent_locations(location).into_iter() {
                if map[adj_location.get_row()][adj_location.get_col()] != '#' {
                    new_locations.insert(adj_location);
                }
            }
        }
        
        current_locations.clear();
        current_locations = new_locations;
    }
    
    println!("{}", current_locations.len());
}

fn get_adjacent_locations(location: &InfiniteMapLocation) -> Vec<InfiniteMapLocation> {
    let mut adjacent_locations: Vec<InfiniteMapLocation> = Vec::new();
    
    adjacent_locations.push(InfiniteMapLocation { row: location.row - 1, col: location.col, num_rows: location.num_rows, num_cols: location.num_cols });
    adjacent_locations.push(InfiniteMapLocation { row: location.row, col: location.col - 1, num_rows: location.num_rows, num_cols: location.num_cols });
    adjacent_locations.push(InfiniteMapLocation { row: location.row + 1, col: location.col, num_rows: location.num_rows, num_cols: location.num_cols });
    adjacent_locations.push(InfiniteMapLocation { row: location.row, col: location.col + 1, num_rows: location.num_rows, num_cols: location.num_cols });
    
    adjacent_locations
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct InfiniteMapLocation {
    row: isize,
    col: isize,
    num_rows: usize,
    num_cols: usize
}

impl InfiniteMapLocation {
    fn get_row(&self) -> usize {
        if self.row < 0 {
            return self.num_rows - 1 - (self.num_rows as isize - (self.row + 1)) as usize % self.num_rows;
        }
        
        self.row as usize % self.num_rows
    }
    
    fn get_col(&self) -> usize {
        if self.col < 0 {
            return self.num_cols - 1 - (self.num_cols as isize - (self.col + 1)) as usize % self.num_cols;
        }
        
        self.col as usize % self.num_cols
    }
}