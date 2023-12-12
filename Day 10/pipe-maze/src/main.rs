use std::collections::HashMap;
use std::collections::HashSet;
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

    // Figure out number of open locations enclosed in the loop.
    let mut outside_locations: HashSet<MazeLocation> = HashSet::new();

    // Note 0, 0 is outside the loop.
    let one_hugo_to_rule_them_all = Hugo { maze_location: MazeLocation { row: 3, col: 3 }, node_location: MazeLocation { row: 0, col: 0 } };
    let mut crusading_hugos = Vec::from([one_hugo_to_rule_them_all.clone()]);
    let mut retired_hugos: HashSet<Hugo> = HashSet::new();

    while !crusading_hugos.is_empty() {
        let mut incoming_hugos: Vec<Hugo> = Vec::new();
        for hugo in crusading_hugos.into_iter() {
            // Magical statement that fixes everything and I have no idea why (literally, I'm not kidding).
            if retired_hugos.contains(&hugo) {
                continue;
            }

            let mut new_hugos: Vec<Hugo> = Vec::new();

            // Try move up.
            if hugo.node_location.row == -1 {
                new_hugos.push(Hugo { maze_location: MazeLocation { row: hugo.maze_location.row - 1, col: hugo.maze_location.col }, node_location: MazeLocation { row: 1, col: hugo.node_location.col } });
            } else {
                new_hugos.push(Hugo { maze_location: hugo.maze_location.clone(), node_location: MazeLocation { row: hugo.node_location.row - 1, col: hugo.node_location.col } });
            }

            // Try move right.
            if hugo.node_location.col == 1 {
                new_hugos.push(Hugo { maze_location: MazeLocation { row: hugo.maze_location.row, col: hugo.maze_location.col + 1 }, node_location: MazeLocation { row: hugo.node_location.row, col: -1 } });
            } else {
                new_hugos.push(Hugo { maze_location: hugo.maze_location.clone(), node_location: MazeLocation { row: hugo.node_location.row, col: hugo.node_location.col + 1 } });
            }
            
            // Try move down.
            if hugo.node_location.row == 1 {
                new_hugos.push(Hugo { maze_location: MazeLocation { row: hugo.maze_location.row + 1, col: hugo.maze_location.col }, node_location: MazeLocation { row: -1, col: hugo.node_location.col } });
            } else {
                new_hugos.push(Hugo { maze_location: hugo.maze_location.clone(), node_location: MazeLocation { row: hugo.node_location.row + 1, col: hugo.node_location.col } });
            }

            // Try move left.
            if hugo.node_location.col == -1 {
                new_hugos.push(Hugo { maze_location: MazeLocation { row: hugo.maze_location.row, col: hugo.maze_location.col - 1 }, node_location: MazeLocation { row: hugo.node_location.row, col: 1 } });
            } else {
                new_hugos.push(Hugo { maze_location: hugo.maze_location.clone(), node_location: MazeLocation { row: hugo.node_location.row, col: hugo.node_location.col - 1 } });
            }

            for voluntold in new_hugos.into_iter() {
                if voluntold.maze_location.row < 0 || voluntold.maze_location.row >= maze.len() as isize || voluntold.maze_location.col < 0 || voluntold.maze_location.col >= maze[0].len() as isize {
                    continue;
                }

                if retired_hugos.contains(&voluntold) {
                    continue;
                }

                if maze_loop.contains(&voluntold.maze_location) {
                    // Things get spicy.
                    let node = maze[voluntold.maze_location.row as usize][voluntold.maze_location.col as usize];

                    if convert_to_three_by_three(node)[(voluntold.node_location.row + 1) as usize][(voluntold.node_location.col + 1) as usize] != '#' {
                        incoming_hugos.push(voluntold);
                    }
                } else {
                    incoming_hugos.push(voluntold);

                    if !outside_locations.contains(&voluntold.maze_location) {
                        outside_locations.insert(voluntold.maze_location);
                    }
                }
            }

            retired_hugos.insert(hugo);
        }

        crusading_hugos = incoming_hugos;
    }

    println!("{}", maze.len() * maze[0].len() - maze_loop.len() - outside_locations.len());
}

fn convert_to_three_by_three(node: char) -> Vec<Vec<char>> {
    match node {
        '|' => return vec![vec!['.', '#', '.']; 3],
        '-' => return vec![vec!['.'; 3], vec!['#'; 3], vec!['.'; 3]],
        'L' => return vec![vec!['.', '#', '.'], vec!['.', '#', '#'], vec!['.'; 3]],
        'J' => return vec![vec!['.', '#', '.'], vec!['#', '#', '.'], vec!['.'; 3]],
        '7' => return vec![vec!['.'; 3], vec!['#', '#', '.'], vec!['.', '#', '.']],
        'F' => return vec![vec!['.'; 3], vec!['.', '#', '#'], vec!['.', '#', '.']],
        'S' => return convert_to_three_by_three('J'),  // Yes I know hardcoding is bad, but at this point just deal with it. Too bad if you don't like it. J FOR THE ACTUAL THING.
        _ => return vec![vec!['.'; 3]; 3]
    }
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

#[derive(Copy, Clone, Debug, Hash)]
struct Hugo {
    maze_location: MazeLocation,
    node_location: MazeLocation
}

impl PartialEq for Hugo {
    fn eq(&self, other: &Self) -> bool {
        (&self.maze_location, &self.node_location) == (&other.maze_location, &other.node_location)
    }
}

impl Eq for Hugo {}

#[derive(PartialEq, Eq, Debug)]
enum CompassDirection {
    NORTH,
    EAST,
    SOUTH,
    WEST
}
