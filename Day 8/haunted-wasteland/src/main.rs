use std::collections::HashMap;
use std::fs;

fn main() {
    let file = fs::read_to_string("src/map.txt").unwrap();
    let mut lines: Vec<&str> = file.split("\n").filter(|&l| !l.is_empty()).collect();

    let instructions: Vec<char> = lines.remove(0).chars().collect();
    let mut connections: HashMap<String, Node> = HashMap::new();

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

        connections.insert(key, Node { left, right });
    }

    let mut current_node = String::from("AAA");
    let mut steps: u32 = 0;

    while current_node != "ZZZ" {
        for step in instructions.iter() {
            match step {
                'L' => current_node = connections.get(&current_node).unwrap().left.clone(),
                'R' => current_node = connections.get(&current_node).unwrap().right.clone(),
                _ => panic!("Unrecognised direction: {}", step),
            }

            steps += 1;
        }
    }

    println!("{}", steps);
}

struct Node {
    left: String,
    right: String,
}
