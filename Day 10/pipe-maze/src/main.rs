use std::collections::HashMap;
use std::fs;

const NORTH_APPROACH_PIPES: &'static [char] = &['|', 'L', 'J'];
const EAST_APPROACH_PIPES: &'static [char] = &['-', 'L', 'F'];
const SOUTH_APPROACH_PIPES: &'static [char] = &['|', 'F', '7'];
const WEST_APPROACH_PIPES: &'static [char] = &['-', 'J', '7'];

fn main() {
    let file = fs::read_to_string("src/maze.txt").unwrap();
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
                    row: i,
                    col: maze[i].iter().position(|&pipe| pipe == 'S').unwrap(),
                };
                break;
            }
        }

        start_location = start;
    }

    let mut current_locations: Vec<Crawler> = Vec::new();

    // Check each of the four pipes surrounding the start pipe.
    if NORTH_APPROACH_PIPES.contains(&maze[start_location.row + 1][start_location.col]) {
        current_locations.push(Crawler {
            current_location: start_location.clone(),
            approach_direction: CompassDirection::NORTH,
        });
    }

    if EAST_APPROACH_PIPES.contains(&maze[start_location.row][start_location.col - 1]) {
        current_locations.push(Crawler {
            current_location: start_location.clone(),
            approach_direction: CompassDirection::EAST,
        });
    }

    if SOUTH_APPROACH_PIPES.contains(&maze[start_location.row - 1][start_location.col]) {
        current_locations.push(Crawler {
            current_location: start_location.clone(),
            approach_direction: CompassDirection::SOUTH,
        });
    }

    if WEST_APPROACH_PIPES.contains(&maze[start_location.row][start_location.col + 1]) {
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

            if maze.get(new_location.row).is_none() {
                continue;
            }

            if maze[new_location.row].get(new_location.col).is_none() {
                continue;
            }

            let new_crawler: Option<Crawler>;
            match crawler.approach_direction {
                CompassDirection::NORTH => match maze[new_location.row][new_location.col] {
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
                CompassDirection::EAST => match maze[new_location.row][new_location.col] {
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
                CompassDirection::SOUTH => match maze[new_location.row][new_location.col] {
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
                CompassDirection::WEST => match maze[new_location.row][new_location.col] {
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
}

#[derive(Copy, Clone, Debug)]
struct MazeLocation {
    row: usize,
    col: usize,
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
    approach_direction: CompassDirection,
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
    WEST,
}
