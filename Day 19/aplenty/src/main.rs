use std::collections::HashMap;
use std::fs;

static CATEGORIES: &'static [char] = &['x', 'm', 'a', 's'];

fn main() {
    let file = fs::read_to_string("src/input.txt").unwrap();
    let lines: Vec<&str> = file.split_terminator("\n").collect();

    let mut mode = Mode::InitialisingWorkflows;
    let mut part_one_total: u32 = 0;

    let mut workflows: HashMap<String, Workflow> = HashMap::new();

    // Set up workflows and part 1.
    for line in lines.into_iter() {
        if line.is_empty() {
            mode = Mode::ProcessingParts;
            continue;
        }

        match mode {
            Mode::InitialisingWorkflows => {
                let name = String::from(line.split("{").collect::<Vec<&str>>()[0]);
                let conditions: Vec<String> = line.split("{").collect::<Vec<&str>>()[1]
                    .split("}")
                    .collect::<Vec<&str>>()[0]
                    .split(",")
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|&condition| String::from(condition))
                    .collect();

                let mut workflow = Workflow {
                    name: name.clone(),
                    conditions: Vec::new(),
                    end_behaviour: conditions[conditions.len() - 1].clone(),
                };

                for i in 0..(conditions.len() - 1) {
                    let break_down: Vec<String> = conditions[i]
                        .split(":")
                        .collect::<Vec<&str>>()
                        .iter()
                        .map(|&info| String::from(info))
                        .collect();
                    workflow.conditions.push(Condition {
                        condition: break_down[0].clone(),
                        failure_workflow: break_down[1].clone(),
                    });
                }

                workflows.insert(name, workflow);
            }
            Mode::ProcessingParts => {
                let mut current_workflow = workflows.get(&String::from("in")).unwrap();

                let info: Vec<String> = line
                    .chars()
                    .filter(|&c| c != '{' && c != '}')
                    .collect::<String>()
                    .split(",")
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|&category| String::from(category))
                    .collect();

                let mut part = Part::default();
                for category in 0..4 {
                    let rating = info[category].split("=").collect::<Vec<&str>>()[1]
                        .parse::<u32>()
                        .unwrap();
                    match category {
                        0 => part.x = rating,
                        1 => part.m = rating,
                        2 => part.a = rating,
                        3 => part.s = rating,
                        _ => panic!("Shit is so very broken"),
                    }
                }

                let mut end_destination_determined = false;
                'destination_loop: while !end_destination_determined {
                    for condition in current_workflow.conditions.iter() {
                        if condition.is_passed(
                            part.get_associated_rating(condition.get_category())
                                .unwrap() as usize,
                        ) {
                            match condition.failure_workflow.as_str() {
                                "A" => {
                                    for category in CATEGORIES.iter() {
                                        part_one_total +=
                                            part.get_associated_rating(*category).unwrap() as u32;
                                    }

                                    end_destination_determined = true;
                                }
                                "R" => end_destination_determined = true,
                                _ => {
                                    current_workflow =
                                        workflows.get(&condition.failure_workflow).unwrap()
                                }
                            }

                            continue 'destination_loop;
                        }
                    }

                    match current_workflow.end_behaviour.as_str() {
                        "A" => {
                            for category in CATEGORIES.iter() {
                                part_one_total +=
                                    part.get_associated_rating(*category).unwrap() as u32;
                            }

                            end_destination_determined = true;
                        }
                        "R" => end_destination_determined = true,
                        _ => {
                            current_workflow =
                                workflows.get(&current_workflow.end_behaviour).unwrap()
                        }
                    }
                }
            }
        }
    }

    // Part 2.

    println!("{}", part_one_total);
}

#[derive(Debug)]
struct Workflow {
    name: String,
    conditions: Vec<Condition>,
    end_behaviour: String,
}

#[derive(Debug)]
struct Condition {
    condition: String,
    failure_workflow: String,
}

impl Condition {
    fn get_category(&self) -> char {
        self.condition.chars().nth(0).unwrap()
    }

    fn get_operator(&self) -> char {
        self.condition.chars().nth(1).unwrap()
    }

    fn get_pass_value(&self) -> usize {
        self.condition.chars().collect::<Vec<char>>()[2..]
            .iter()
            .collect::<String>()
            .parse::<usize>()
            .unwrap()
    }

    fn is_passed(&self, value: usize) -> bool {
        match self.get_operator() {
            '<' => return value < self.get_pass_value(),
            '=' => return value == self.get_pass_value(),
            '>' => return value > self.get_pass_value(),
            _ => panic!("Unsupported operator: {}", self.get_operator()),
        }
    }
}

#[derive(Debug, Default)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn get_associated_rating(&self, category: char) -> Option<u32> {
        match category {
            'x' => return Some(self.x),
            'm' => return Some(self.m),
            'a' => return Some(self.a),
            's' => return Some(self.s),
            _ => return None,
        }
    }
}

enum Mode {
    InitialisingWorkflows,
    ProcessingParts,
}
