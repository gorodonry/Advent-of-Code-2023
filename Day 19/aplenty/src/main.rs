use std::collections::HashMap;
use std::fs;

static CATEGORIES: &'static [char] = &['x', 'm', 'a', 's'];

fn main() {
    let file = fs::read_to_string("src/test.txt").unwrap();
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
    let mut ranges_in_progress: HashMap<String, Vec<PartRange>> = HashMap::new();

    for workflow in workflows.keys() {
        ranges_in_progress.insert(workflow.clone(), Vec::new());
    }

    ranges_in_progress.insert(String::from("in"), vec![PartRange::default()]);

    let mut part_two_total: usize = 0;

    while !all_workflows_complete(&ranges_in_progress) {
        let mut new_ranges: HashMap<String, Vec<PartRange>> = HashMap::new();

        for (workflow_name, ranges) in ranges_in_progress.iter_mut() {            
            if vec!["A", "R"].contains(&workflow_name.as_str()) {
                continue;
            }

            let workflow = workflows.get(workflow_name).unwrap();

            for range in ranges.iter_mut() {
                for condition in workflow.conditions.iter() {
                    if !range.all_ranges_valid() {
                        break;
                    }
                    
                    match condition.get_operator() {
                        '<' => {
                            if range.get_associated_range(condition.get_category()).unwrap().start >= condition.get_pass_value() {
                                match new_ranges.get_mut(&condition.failure_workflow) {
                                    Some(vec) => {
                                        vec.push(range.clone());
                                    }
                                    None => {
                                        new_ranges.insert(condition.failure_workflow.clone(), vec![range.clone()]);
                                    }
                                }
                            } else if range.get_associated_range(condition.get_category()).unwrap().end < condition.get_pass_value() {
                                continue;
                            } else {
                                let mut upper_range = range.clone();
                                
                                let mut associated_range = upper_range.get_associated_range(condition.get_category()).unwrap().clone();
                                associated_range.start = condition.get_pass_value();
                                upper_range.update_associated_range(associated_range, condition.get_category());

                                match new_ranges.get_mut(&condition.failure_workflow) {
                                    Some(vec) => {
                                        vec.push(upper_range);
                                    }
                                    None => {
                                        new_ranges.insert(condition.failure_workflow.clone(), vec![upper_range]);
                                    }
                                }
                                
                                let mut associated_range = range.get_associated_range(condition.get_category()).unwrap().clone();
                                associated_range.end = condition.get_pass_value() - 1;
                                range.update_associated_range(associated_range, condition.get_category());
                            }
                        }
                        '>' => {
                            if range.get_associated_range(condition.get_category()).unwrap().end <= condition.get_pass_value() {
                                match new_ranges.get_mut(&condition.failure_workflow) {
                                    Some(vec) => {
                                        vec.push(range.clone());
                                    }
                                    None => {
                                        new_ranges.insert(condition.failure_workflow.clone(), vec![range.clone()]);
                                    }
                                }
                            } else if range.get_associated_range(condition.get_category()).unwrap().start > condition.get_pass_value() {
                                continue;
                            } else {
                                let mut lower_range = range.clone();
                                
                                let mut associated_range = lower_range.get_associated_range(condition.get_category()).unwrap().clone();
                                associated_range.end = condition.get_pass_value();
                                lower_range.update_associated_range(associated_range, condition.get_category());
                                
                                match new_ranges.get_mut(&condition.failure_workflow) {
                                    Some(vec) => {
                                        vec.push(lower_range);
                                    }
                                    None => {
                                        new_ranges.insert(condition.failure_workflow.clone(), vec![lower_range]);
                                    }
                                }
                                
                                let mut associated_range = range.get_associated_range(condition.get_category()).unwrap().clone();
                                associated_range.start = condition.get_pass_value() + 1;
                                range.update_associated_range(associated_range, condition.get_category());
                            }
                        }
                        _ => panic!("Yeah, ummm, about that")
                    }
                }
                
                if range.all_ranges_valid() {
                    match new_ranges.get_mut(&workflow.end_behaviour) {
                        Some(vec) => {
                            vec.push(range.clone());
                        }
                        None => {
                            new_ranges.insert(workflow.end_behaviour.clone(), vec![range.clone()]);
                        }
                    }
                }
            }
        }
        
        match new_ranges.get(&String::from("A")) {
            Some(ranges) => {
                for range in ranges.iter() {
                    let x_combinations = calculate_sum(range.x.end) - calculate_sum(std::cmp::max(range.x.start - 1, 0));
                    let m_combinations = calculate_sum(range.m.end) - calculate_sum(std::cmp::max(range.m.start - 1, 0));
                    let a_combinations = calculate_sum(range.a.end) - calculate_sum(std::cmp::max(range.a.start - 1, 0));
                    let s_combinations = calculate_sum(range.s.end) - calculate_sum(std::cmp::max(range.s.start - 1, 0));
                    
                    part_two_total += x_combinations * m_combinations * a_combinations * s_combinations;
                }
            }
            None => ()
        }
        
        ranges_in_progress = new_ranges;
    }

    println!("Part 1: {}", part_one_total);
    println!("Part 2: {}", part_two_total);
}

fn calculate_sum(n: usize) -> usize {
    (n * (n + 1)) / 2
}

fn all_workflows_complete(workflows: &HashMap<String, Vec<PartRange>>) -> bool {
    for ranges in workflows.values() {
        if !ranges.is_empty() {
            return false;
        }
    }

    true
}

#[derive(Debug, Clone)]
struct Workflow {
    name: String,
    conditions: Vec<Condition>,
    end_behaviour: String,
}

#[derive(Debug, Clone)]
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

#[derive(Clone, PartialEq, Eq, Hash)]
struct PartRange {
    x: IntRange,
    m: IntRange,
    a: IntRange,
    s: IntRange
}

impl PartRange {
    fn get_associated_range(&self, category: char) -> Option<&IntRange> {
        match category {
            'x' => return Some(&self.x),
            'm' => return Some(&self.m),
            'a' => return Some(&self.a),
            's' => return Some(&self.s),
            _ => return None,
        }
    }
    
    fn update_associated_range(&mut self, new_range: IntRange, category: char) {
        match category {
            'x' => self.x = new_range,
            'm' => self.m = new_range,
            'a' => self.a = new_range,
            's' => self.s = new_range,
            _ => ()
        }
    }
    
    fn all_ranges_valid(&self) -> bool {
        self.x.is_valid() && self.m.is_valid() && self.a.is_valid() && self.s.is_valid()
    }
}

impl Default for PartRange {
    fn default() -> PartRange {
        let default_range = IntRange { start: 1, end: 4000 };

        PartRange { x: default_range.clone(), m: default_range.clone(), a: default_range.clone(), s: default_range.clone() }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct IntRange {
    start: usize,
    end: usize
}

impl IntRange {
    fn is_valid(&self) -> bool {
        self.start <= self.end
    }
}

enum Mode {
    InitialisingWorkflows,
    ProcessingParts,
}
