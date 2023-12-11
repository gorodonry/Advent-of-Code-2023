use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

const NORTH_APPROACH_PIPES: &'static [char] = &['|', 'L', 'J'];
const EAST_APPROACH_PIPES: &'static [char] = &['-', 'L', 'F'];
const SOUTH_APPROACH_PIPES: &'static [char] = &['|', 'F', '7'];
const WEST_APPROACH_PIPES: &'static [char] = &['-', 'J', '7'];

fn main() {
    let file = fs::read_to_string("src/test1.txt").unwrap();
    let maze: Vec<Vec<char>> = file
        .split_terminator("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&l| l.chars().collect::<Vec<char>>())
        .collect();

    let start_location: MazeLocation;

    // Determine start location.
    {
        let mut start = MazeLocation { row: 0, col: 0 };

        for i in 0..maze.len() {
            if maze[i].contains(&'S') {
                start = MazeLocation {
                    row: i as isize,
                    col: maze[i].iter().position(|&pipe| pipe == 'S').unwrap() as isize,
                };
                break;
            }
        }

        start_location = start;
    }

    let mut current_locations: Vec<Crawler> = Vec::new();

    // Check each of the four pipes surrounding the start pipe.
    if NORTH_APPROACH_PIPES.contains(&maze[start_location.row as usize + 1][start_location.col as usize]) {
        current_locations.push(Crawler {
            current_location: start_location.clone(),
            approach_direction: CompassDirection::NORTH,
        });
    }

    if EAST_APPROACH_PIPES.contains(&maze[start_location.row as usize][start_location.col as usize - 1]) {
        current_locations.push(Crawler {
            current_location: start_location.clone(),
            approach_direction: CompassDirection::EAST,
        });
    }

    if SOUTH_APPROACH_PIPES.contains(&maze[start_location.row as usize - 1][start_location.col as usize]) {
        current_locations.push(Crawler {
            current_location: start_location.clone(),
            approach_direction: CompassDirection::SOUTH,
        });
    }

    if WEST_APPROACH_PIPES.contains(&maze[start_location.row as usize][start_location.col as usize + 1]) {
        current_locations.push(Crawler {
            current_location: start_location.clone(),
            approach_direction: CompassDirection::WEST,
        });
    }

    let mut traversed_pipes: Vec<MazeLocation> = Vec::from([start_location]);
    let mut crawler_history: HashMap<usize, Vec<MazeLocation>> = HashMap::new();

    for i in 0..current_locations.len() {
        crawler_history.insert(i, Vec::from([start_location]));
    }

    // Figure out the coordinates in the loop.
    let mut loop_traversed = false;
    while !loop_traversed {
        let mut new_locations: Vec<Crawler> = Vec::new();

        for i in 0..current_locations.len() {
            let crawler = &current_locations[i];

            let new_location: MazeLocation;
            match crawler.approach_direction {
                CompassDirection::NORTH => {
                    new_location = MazeLocation {
                        row: crawler.current_location.row + 1,
                        col: crawler.current_location.col,
                    };
                }
                CompassDirection::EAST => {
                    new_location = MazeLocation {
                        row: crawler.current_location.row,
                        col: crawler.current_location.col - 1,
                    };
                }
                CompassDirection::SOUTH => {
                    new_location = MazeLocation {
                        row: crawler.current_location.row - 1,
                        col: crawler.current_location.col,
                    };
                }
                CompassDirection::WEST => {
                    new_location = MazeLocation {
                        row: crawler.current_location.row,
                        col: crawler.current_location.col + 1,
                    };
                }
            }

            if maze.get(new_location.row as usize).is_none() {
                continue;
            }

            if maze[new_location.row as usize].get(new_location.col as usize).is_none() {
                continue;
            }

            let new_crawler: Option<Crawler>;
            match crawler.approach_direction {
                CompassDirection::NORTH => match maze[new_location.row as usize][new_location.col as usize] {
                    '|' => {
                        new_crawler = Some(Crawler {
                            current_location: new_location,
                            approach_direction: CompassDirection::NORTH,
                        })
                    }
                    'L' => {
                        new_crawler = Some(Crawler {
                            current_location: new_location,
                            approach_direction: CompassDirection::WEST,
                        })
                    }
                    'J' => {
                        new_crawler = Some(Crawler {
                            current_location: new_location,
                            approach_direction: CompassDirection::EAST,
                        })
                    }
                    _ => new_crawler = None,
                },
                CompassDirection::EAST => match maze[new_location.row as usize][new_location.col as usize] {
                    '-' => {
                        new_crawler = Some(Crawler {
                            current_location: new_location,
                            approach_direction: CompassDirection::EAST,
                        })
                    }
                    'L' => {
                        new_crawler = Some(Crawler {
                            current_location: new_location,
                            approach_direction: CompassDirection::SOUTH,
                        })
                    }
                    'F' => {
                        new_crawler = Some(Crawler {
                            current_location: new_location,
                            approach_direction: CompassDirection::NORTH,
                        })
                    }
                    _ => new_crawler = None,
                },
                CompassDirection::SOUTH => match maze[new_location.row as usize][new_location.col as usize] {
                    '|' => {
                        new_crawler = Some(Crawler {
                            current_location: new_location,
                            approach_direction: CompassDirection::SOUTH,
                        })
                    }
                    'F' => {
                        new_crawler = Some(Crawler {
                            current_location: new_location,
                            approach_direction: CompassDirection::WEST,
                        })
                    }
                    '7' => {
                        new_crawler = Some(Crawler {
                            current_location: new_location,
                            approach_direction: CompassDirection::EAST,
                        })
                    }
                    _ => new_crawler = None,
                },
                CompassDirection::WEST => match maze[new_location.row as usize][new_location.col as usize] {
                    '-' => {
                        new_crawler = Some(Crawler {
                            current_location: new_location,
                            approach_direction: CompassDirection::WEST,
                        })
                    }
                    'J' => {
                        new_crawler = Some(Crawler {
                            current_location: new_location,
                            approach_direction: CompassDirection::SOUTH,
                        })
                    }
                    '7' => {
                        new_crawler = Some(Crawler {
                            current_location: new_location,
                            approach_direction: CompassDirection::NORTH,
                        })
                    }
                    _ => new_crawler = None,
                },
            }

            if new_crawler.is_none() {
                continue;
            }

            if traversed_pipes.contains(&new_crawler.as_ref().unwrap().current_location) {
                loop_traversed = true;
            } else {
                traversed_pipes.push(new_crawler.as_ref().unwrap().current_location.clone());

                let mut history = crawler_history.get(&i).unwrap().clone();
                history.push(new_crawler.as_ref().unwrap().current_location);
                crawler_history.insert(i, history);

                new_locations.push(new_crawler.unwrap());
            }
        }

        current_locations = new_locations;
    }

    let maze_loop: Vec<MazeLocation>;

    {
        let mut maze_loop_parts = Vec::from(crawler_history.get(&0).unwrap().clone());
        maze_loop_parts.extend(crawler_history.get(&1).unwrap().iter().rev().clone());
        maze_loop_parts.pop();
        maze_loop = maze_loop_parts;

        drop(crawler_history);
        drop(traversed_pipes);
        drop(current_locations);
    }

    // Figure out number of open locations enclosed in the maze.
    let mut number_of_potential_nests: u32 = 0;
    let mut visited_locations: HashSet<MazeLocation> = HashSet::from_iter(maze_loop.clone());

    for i in 0..maze.len() {
        for j in 0..maze[0].len() {
            let one_hugo_to_rule_them_all = MazeLocation { row: i as isize, col: j as isize };

            if visited_locations.contains(&one_hugo_to_rule_them_all) {
                continue;
            }

            // Try and find a valid way to the edge.
            let mut crusading_hugos = Vec::from([one_hugo_to_rule_them_all.clone()]);
            let mut retired_hugos = Vec::new();

            let mut edge_reached = false;
            while !crusading_hugos.is_empty() && !edge_reached {
                for hugo in crusading_hugos.clone().iter() {
                    // Try exploring in each of the four directions and just see what happens.
                    let mut new_hugos: Vec<MazeLocation> = Vec::new();

                    if hugo.row != 0 {
                        new_hugos.push(MazeLocation { row: hugo.row - 1, col: hugo.col });
                    } else {
                        new_hugos.push( MazeLocation { row: -1, col: -1 });
                    }

                    if hugo.col != 0 {
                        new_hugos.push(MazeLocation { row: hugo.row, col: hugo.col - 1 });
                    } else {
                        new_hugos.push( MazeLocation { row: -1, col: -1 });
                    }

                    if hugo.row != maze.len() as isize - 1 {
                        new_hugos.push(MazeLocation { row: hugo.row + 1, col: hugo.col });
                    } else {
                        new_hugos.push( MazeLocation { row: -1, col: -1 });
                    }

                    if hugo.col != maze[0].len() as isize - 1 {
                        new_hugos.push(MazeLocation { row: hugo.row, col: hugo.col + 1 });
                    } else {
                        new_hugos.push( MazeLocation { row: -1, col: -1 });
                    }

                    for voluntold in new_hugos.clone().iter() {
                        if (*voluntold == MazeLocation { row: -1, col: -1 }) {
                            new_hugos.remove(new_hugos.iter().position(|&h| h == *voluntold).unwrap());
                            continue;
                        }

                        if maze_loop.contains(voluntold) {
                            // Check surroundings for sneaky.
                            match new_hugos.iter().position(|&h| h == *voluntold).unwrap() + (4 - new_hugos.len()) {
                                0 => {
                                    let left = MazeLocation { row: voluntold.row, col: voluntold.col - 1 };
                                    let right = MazeLocation { row: voluntold.row, col: voluntold.col + 1 };

                                    if maze_loop.contains(&left) && maze_loop.contains(&right) {
                                        if !((maze_loop.iter().position(|&phugo| phugo == left).unwrap() as isize - maze_loop.iter().position(|&phugo| phugo == *voluntold).unwrap() as isize).abs() == 1) {
                                            // Sneaky has occurred.
                                        } else if!((maze_loop.iter().position(|&phugo| phugo == *voluntold).unwrap() as isize - maze_loop.iter().position(|&phugo| phugo == right).unwrap() as isize).abs() == 1) {
                                            // Sneaky has occurred.
                                        }
                                    }
                                },
                                1 => {
                                    // Implement for these as well.
                                },
                                2 => {

                                },
                                3 => {

                                },
                                _ => {
                                    panic!("Too many hugos AHHHHH");
                                }
                            }

                            new_hugos.remove(new_hugos.iter().position(|&h| h == *voluntold).unwrap());
                        } else if visited_locations.contains(voluntold) {
                            new_hugos.remove(new_hugos.iter().position(|&h| h == *voluntold).unwrap());
                        } else {
                            if voluntold.row == 0 || voluntold.row == maze.len() as isize || voluntold.col == 0 || voluntold.col == maze[0].len() as isize {
                                edge_reached = true;
                            }
                        }
                    }

                    crusading_hugos.remove(crusading_hugos.iter().position(|&h| h == *hugo).unwrap());
                    crusading_hugos.extend(new_hugos.clone());
                    retired_hugos.push(hugo.clone());
                    visited_locations.extend(new_hugos);
                }
            }

            println!("{:?}", retired_hugos);
            println!("Current hugos: {:?}", crusading_hugos);
            println!("All visited: {:?}", visited_locations);

            if !edge_reached {
                number_of_potential_nests += retired_hugos.len() as u32;
            }
        }
    }

    println!("{}", number_of_potential_nests);
}

#[derive(Copy, Clone, Debug, Hash)]
struct MazeLocation {
    row: isize,
    col: isize
}

impl PartialEq for MazeLocation {
    fn eq(&self, other: &Self) -> bool {
        (&self.row, &self.col) == (&other.row, &other.col)
    }
}

impl Eq for MazeLocation {}

#[derive(Debug)]
struct Crawler {
    current_location: MazeLocation,
    approach_direction: CompassDirection
}

impl PartialEq for Crawler {
    fn eq(&self, other: &Self) -> bool {
        (&self.current_location, &self.approach_direction)
            == (&other.current_location, &other.approach_direction)
    }
}

impl Eq for Crawler {}

#[derive(PartialEq, Eq, Debug)]
enum CompassDirection {
    NORTH,
    EAST,
    SOUTH,
    WEST
}
