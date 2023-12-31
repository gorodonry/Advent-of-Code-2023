use std::fs;

fn main() {
    let file = fs::read_to_string("src/floor_plan.txt").unwrap();
    let floor: Vec<Vec<char>> = file
        .split_terminator("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&row| row.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // Part 1.
    println!(
        "Part 1: {}",
        determine_number_of_energised_tiles(
            &floor,
            Beam {
                location: Location { row: 0, col: -1 },
                direction: Direction::RIGHT
            }
        )
    );

    // Part 2.
    let mut energised_tiles: Vec<usize> = Vec::new();

    // Left and right sides.
    for i in 0..floor.len() {
        energised_tiles.push(determine_number_of_energised_tiles(
            &floor,
            Beam {
                location: Location {
                    row: i as isize,
                    col: -1,
                },
                direction: Direction::RIGHT,
            },
        ));
        energised_tiles.push(determine_number_of_energised_tiles(
            &floor,
            Beam {
                location: Location {
                    row: i as isize,
                    col: floor[0].len() as isize,
                },
                direction: Direction::LEFT,
            },
        ));
    }

    // Top and bottom.
    for i in 0..floor[0].len() {
        energised_tiles.push(determine_number_of_energised_tiles(
            &floor,
            Beam {
                location: Location {
                    row: -1,
                    col: i as isize,
                },
                direction: Direction::DOWN,
            },
        ));
        energised_tiles.push(determine_number_of_energised_tiles(
            &floor,
            Beam {
                location: Location {
                    row: floor.len() as isize,
                    col: i as isize,
                },
                direction: Direction::UP,
            },
        ));
    }

    println!("Part 2: {}", energised_tiles.iter().max().unwrap());
}

fn determine_number_of_energised_tiles(floor: &Vec<Vec<char>>, start_tile: Beam) -> usize {
    let mut energised_tiles: Vec<Location> = Vec::new();
    let mut beams: Vec<Beam> = Vec::from([start_tile]);
    let mut past_beams: Vec<Beam> = Vec::new();

    while !beams.is_empty() {
        let mut new_beams: Vec<Beam> = Vec::new();

        for beam in beams.iter() {
            if !energised_tiles.contains(&beam.location) {
                energised_tiles.push(beam.location.clone());
            }

            let next_location = get_next_location(&beam.location, &beam.direction);

            if next_location.row < 0
                || next_location.row >= floor.len() as isize
                || next_location.col < 0
                || next_location.col >= floor[0].len() as isize
            {
                continue;
            }

            match floor[next_location.row as usize][next_location.col as usize] {
                '-' => match beam.direction {
                    Direction::UP | Direction::DOWN => {
                        new_beams.push(Beam {
                            location: next_location.clone(),
                            direction: Direction::LEFT,
                        });
                        new_beams.push(Beam {
                            location: next_location,
                            direction: Direction::RIGHT,
                        });
                    }
                    _ => new_beams.push(Beam {
                        location: next_location,
                        direction: beam.direction,
                    }),
                },
                '|' => match beam.direction {
                    Direction::LEFT | Direction::RIGHT => {
                        new_beams.push(Beam {
                            location: next_location.clone(),
                            direction: Direction::UP,
                        });
                        new_beams.push(Beam {
                            location: next_location,
                            direction: Direction::DOWN,
                        });
                    }
                    _ => new_beams.push(Beam {
                        location: next_location,
                        direction: beam.direction,
                    }),
                },
                '/' => match beam.direction {
                    Direction::UP => new_beams.push(Beam {
                        location: next_location,
                        direction: Direction::RIGHT,
                    }),
                    Direction::DOWN => new_beams.push(Beam {
                        location: next_location,
                        direction: Direction::LEFT,
                    }),
                    Direction::LEFT => new_beams.push(Beam {
                        location: next_location,
                        direction: Direction::DOWN,
                    }),
                    Direction::RIGHT => new_beams.push(Beam {
                        location: next_location,
                        direction: Direction::UP,
                    }),
                },
                '\\' => match beam.direction {
                    Direction::UP => new_beams.push(Beam {
                        location: next_location,
                        direction: Direction::LEFT,
                    }),
                    Direction::DOWN => new_beams.push(Beam {
                        location: next_location,
                        direction: Direction::RIGHT,
                    }),
                    Direction::LEFT => new_beams.push(Beam {
                        location: next_location,
                        direction: Direction::UP,
                    }),
                    Direction::RIGHT => new_beams.push(Beam {
                        location: next_location,
                        direction: Direction::DOWN,
                    }),
                },
                '.' => new_beams.push(Beam {
                    location: next_location,
                    direction: beam.direction.clone(),
                }),
                _ => panic!("That's really damn sneaky"),
            }
        }

        for beam in beams.iter_mut() {
            past_beams.push(beam.to_owned());
        }

        beams.clear();

        for beam in new_beams.into_iter() {
            if past_beams.contains(&beam) {
                continue;
            }

            beams.push(beam);
        }
    }

    energised_tiles.len() - 1
}

fn get_next_location(location: &Location, direction: &Direction) -> Location {
    let new_location: Location;

    match direction {
        Direction::UP => {
            new_location = Location {
                row: location.row - 1,
                col: location.col,
            }
        }
        Direction::DOWN => {
            new_location = Location {
                row: location.row + 1,
                col: location.col,
            }
        }
        Direction::LEFT => {
            new_location = Location {
                row: location.row,
                col: location.col - 1,
            }
        }
        Direction::RIGHT => {
            new_location = Location {
                row: location.row,
                col: location.col + 1,
            }
        }
    }

    new_location
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Location {
    row: isize,
    col: isize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Beam {
    location: Location,
    direction: Direction,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
