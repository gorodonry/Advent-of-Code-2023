use std::fs;

const RED_CUBES: u8 = 12;
const GREEN_CUBES: u8 = 13;
const BLUE_CUBES: u8 = 14;

fn main() {
    let file = fs::read_to_string("src/games.txt").unwrap();
    let games: Vec<&str> = file.split_terminator("\n").collect();

    let mut valid_game_ids: Vec<u16> = Vec::new();
    for game in games.iter() {
        let input: Vec<&str> = game.split_terminator(": ").collect();

        let all_cube_info: Vec<&str> = input[1].split_terminator("; ").collect();
        let mut valid_game = true;
        'draw_loop: for info in all_cube_info.iter() {
            let cube_info: Vec<&str> = info.split_terminator(", ").collect();
            for cube in cube_info.iter() {
                let number_of_cubes: u16 = cube.split(" ").collect::<Vec<&str>>()[0].parse::<u16>().unwrap();
                match cube.split(" ").collect::<Vec<&str>>()[1] {
                    "red" => {
                        if number_of_cubes > RED_CUBES as u16 {
                            valid_game = false;
                            break 'draw_loop;
                        }
                    },
                    "green" => {
                        if number_of_cubes > GREEN_CUBES as u16 {
                            valid_game = false;
                            break 'draw_loop;
                        }
                    },
                    "blue" => {
                        if number_of_cubes > BLUE_CUBES as u16 {
                            valid_game = false;
                            break 'draw_loop;
                        }
                    },
                    _ => panic!("Invalid colour: {}", cube.split(" ").collect::<Vec<&str>>()[1]),
                }
            }
        }

        if valid_game {
            valid_game_ids.push(input[0].split(" ").collect::<Vec<&str>>()[1].parse::<u16>().unwrap());
        }
    }

    let sum: u16 = valid_game_ids.iter().sum();

    println!("{}", sum);
}
