extern crate num;

use std::collections::HashMap;
use std::fs;

fn main() {
    let file = fs::read_to_string("src/map.txt").unwrap();
    let mut lines: Vec<&str> = file.split("\n").filter(|&l| !l.is_empty()).collect();

    let instructions: Vec<char> = lines.remove(0).chars().collect();
    let mut connections: HashMap<String, Node> = HashMap::new();

    let mut start_nodes: Vec<String> = Vec::new();

    for line in lines.into_iter() {
        let key = String::from(line.split("=").collect::<Vec<&str>>()[0].trim());
        let left = String::from(
            line.split("(").collect::<Vec<&str>>()[1]
                .split(",")
                .collect::<Vec<&str>>()[0],
        );
        let right = String::from(
            line.split(",").collect::<Vec<&str>>()[1]
                .trim()
                .split(")")
                .collect::<Vec<&str>>()[0],
        );

        if key.chars().nth(2).unwrap() == 'A' {
            start_nodes.push(key.clone());
        }

        connections.insert(key, Node { left, right });
    }

    let mut steps_by_start_node: Vec<u32> = Vec::new();

    for node in start_nodes.into_iter() {
        let mut steps: u32 = 0;
        let mut current_node = node.clone();

        while current_node.chars().nth(2).unwrap() != 'Z' {
            for step in instructions.iter() {
                match step {
                    'L' => current_node = connections.get(&current_node).unwrap().left.clone(),
                    'R' => current_node = connections.get(&current_node).unwrap().right.clone(),
                    _ => panic!("Unrecognised direction: {}", step),
                }

                steps += 1;
            }
        }

        steps_by_start_node.push(steps);
    }

    println!(
        "{}",
        lcm(&steps_by_start_node.iter().map(|&e| e as usize).collect()).unwrap()
    );
}

fn lcm(ints: &Vec<usize>) -> Option<usize> {
    if ints.len() == 0 {
        return None;
    }

    let mut to_reduce = ints.clone();

    while to_reduce.len() > 1 {
        let first_int = to_reduce.remove(0);
        let second_int = to_reduce.remove(0);

        to_reduce.push(num::integer::lcm(first_int, second_int));
    }

    Some(*to_reduce.get(0).unwrap())
}

struct Node {
    left: String,
    right: String,
}
