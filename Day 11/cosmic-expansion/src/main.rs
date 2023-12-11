use std::fs;

fn main() {
    let file = fs::read_to_string("src/map.txt").unwrap();
    let map: Vec<Vec<char>> = file
        .split_terminator("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&l| l.chars().collect::<Vec<char>>())
        .collect();

    let mut galaxy_locations: Vec<GalaxyLocation> = Vec::new();

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '#' {
                galaxy_locations.push(GalaxyLocation {
                    row: i as isize,
                    col: j as isize,
                });
            }
        }
    }

    let mut part_one_total: u32 = 0;
    let mut part_two_total: u64 = 0;
    for g in 0..galaxy_locations.len() {
        for og in (g + 1)..galaxy_locations.len() {
            let steps: u16 = ((galaxy_locations[g].row - galaxy_locations[og].row).abs()
                + (galaxy_locations[g].col - galaxy_locations[og].col).abs())
                as u16;

            part_one_total += steps as u32;
            part_two_total += steps as u64;

            for row in std::cmp::min(galaxy_locations[g].row + 1, galaxy_locations[og].row + 1)
                ..std::cmp::max(galaxy_locations[g].row, galaxy_locations[og].row)
            {
                if empty_space(&map[row as usize]) {
                    part_one_total += 1;
                    part_two_total += 999999;
                }
            }

            for col in std::cmp::min(galaxy_locations[g].col + 1, galaxy_locations[og].col)
                ..std::cmp::max(galaxy_locations[g].col, galaxy_locations[og].col)
            {
                let column: Vec<char> = map.iter().map(|r| r[col as usize]).collect();
                if empty_space(&column) {
                    part_one_total += 1;
                    part_two_total += 999999;
                }
            }
        }
    }

    println!("Part 1: {}", part_one_total);
    println!("Part 2: {}", part_two_total);
}

fn empty_space(vector: &Vec<char>) -> bool {
    for c in vector.iter() {
        if *c != '.' {
            return false;
        }
    }

    true
}

struct GalaxyLocation {
    row: isize,
    col: isize,
}
