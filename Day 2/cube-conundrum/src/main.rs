use std::fs;
use std::collections::HashMap;

const RED_CUBES: u8 = 12;
const GREEN_CUBES: u8 = 13;
const BLUE_CUBES: u8 = 14;

fn main() {
    let file = fs::read_to_string("src/games.txt").unwrap();
    let games: Vec<&str> = file.split_terminator("\n").collect();

    let mut valid_game_ids: Vec<u16> = Vec::new();
    let mut minimum_cubes_needed: Vec<Vec<u16>> = Vec::new();
    for game in games.iter() {
        let input: Vec<&str> = game.split_terminator(": ").collect();

        let mut minimum_cubes_this_cube = HashMap::new();
        let all_cube_info: Vec<&str> = input[1].split_terminator("; ").collect();
        
        minimum_cubes_this_cube.insert("red", 0);
        minimum_cubes_this_cube.insert("green", 0);
        minimum_cubes_this_cube.insert("blue", 0);

        let mut valid_game = true;
        for info in all_cube_info.iter() {
            let cube_info: Vec<&str> = info.split_terminator(", ").collect();

            for cube in cube_info.iter() {
                let number_of_cubes: u16 = cube.split(" ").collect::<Vec<&str>>()[0].parse::<u16>().unwrap();
                match cube.split(" ").collect::<Vec<&str>>()[1] {
                    "red" => {
                        if number_of_cubes > RED_CUBES as u16 {
                            valid_game = false;
                        }

                        if number_of_cubes > *minimum_cubes_this_cube.get(&"red").unwrap() {
                            minimum_cubes_this_cube.insert("red", number_of_cubes);
                        }
                    },
                    "green" => {
                        if number_of_cubes > GREEN_CUBES as u16 {
                            valid_game = false;
                        }

                        if number_of_cubes > *minimum_cubes_this_cube.get(&"green").unwrap() {
                            minimum_cubes_this_cube.insert("green", number_of_cubes);
                        }
                    },
                    "blue" => {
                        if number_of_cubes > BLUE_CUBES as u16 {
                            valid_game = false;
                        }

                        if number_of_cubes > *minimum_cubes_this_cube.get(&"blue").unwrap() {
                            minimum_cubes_this_cube.insert("blue", number_of_cubes);
                        }
                    },
                    _ => panic!("Invalid colour: {}", cube.split(" ").collect::<Vec<&str>>()[1]),
                }
            }
        }

        if valid_game {
            valid_game_ids.push(input[0].split(" ").collect::<Vec<&str>>()[1].parse::<u16>().unwrap());
        }

        minimum_cubes_needed.push(minimum_cubes_this_cube.values().map(|v| *v).collect::<Vec<u16>>());
    }

    let valid_id_sum: u16 = valid_game_ids.iter().sum();
    let power_sum: usize = minimum_cubes_needed.iter().map(|v| multiply_elements(v)).collect::<Vec<usize>>().iter().sum();

    println!("Sum of valid IDs: {}", valid_id_sum);
    println!("Sum of power: {}", power_sum);
}

fn multiply_elements(vec: &Vec<u16>) -> usize {
    if vec.is_empty() {
        return 0;
    }

    let mut total: usize = 1;
    
    for element in vec.iter() {
        total *= *element as usize;
    }

    total
}

