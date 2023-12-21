use std::fs;
use std::collections::LinkedList;
use std::collections::HashMap;

fn main() {
    let file = fs::read_to_string("src/modules.txt").unwrap();
    let lines: Vec<&str> = file.split_terminator("\n").collect();

    let mut modules: HashMap<String, ModuleType> = HashMap::new();

    for line in lines.iter() {
        let info = line.split("->").collect::<Vec<&str>>()[0].trim();
        let pulse_destinations: Vec<String> = line.split("->").collect::<Vec<&str>>()[1].trim().split(",").collect::<Vec<&str>>().iter().map(|&destination| String::from(destination.trim())).collect();
        
        let name: String;
        let module: ModuleType;
        match info.chars().nth(0).unwrap() {
            '%' => {
                name = info.chars().collect::<Vec<char>>()[1..].iter().collect();
                module = ModuleType::FlipFlop(FlipFlop { name: name.clone(), pulse_destinations, state: FlipFlopState::Off, pulses_queue: LinkedList::new() });
            }
            '&' => {
                name = info.chars().collect::<Vec<char>>()[1..].iter().collect();
                
                let mut inputs: Vec<String> = Vec::new();
                for line in lines.iter() {
                    if line.split("->").collect::<Vec<&str>>()[1].trim().split(",").collect::<Vec<&str>>().iter().map(|&destination| destination.trim()).collect::<Vec<&str>>().contains(&name.as_str()) {
                        let input = line.split("->").collect::<Vec<&str>>()[0].trim();
                        
                        match input.chars().nth(0).unwrap() {
                            '%' => inputs.push(input.chars().collect::<Vec<char>>()[1..].iter().collect::<String>()),
                            '&' => inputs.push(input.chars().collect::<Vec<char>>()[1..].iter().collect::<String>()),
                            _ => inputs.push(String::from(input))
                        }
                    }
                }
                
                module = ModuleType::Conjunction(Conjunction::new(name.clone(), pulse_destinations, inputs));
            }
            _ => {
                name = String::from(info);
                module = ModuleType::Broadcaster(Broadcaster { pulse_destinations });
            }
        }
        
        modules.insert(name, module);
    }
    
    let mut low_pulses_sent: u64 = 0;
    let mut high_pulses_sent: u64 = 0;
    
    for _ in 0..1000 {
        // Push the button (send low pulse to broadcaster).
        low_pulses_sent += 1;
        
        // Send pulse from broadcaster.
        let broadcaster: Option<Broadcaster>;
    
        if let ModuleType::Broadcaster(module) = modules.get(&String::from("broadcaster")).unwrap() {
            broadcaster = Some(module.clone());
        } else {
            broadcaster = None;
        }
        
        for destination in broadcaster.as_ref().unwrap().get_destinations().iter() {
            low_pulses_sent += 1;
            
            match modules.get_mut(destination).unwrap() {
                ModuleType::FlipFlop(module) => module.receive_pulse(Pulse::Low, broadcaster.as_ref().unwrap().get_name()),
                ModuleType::Conjunction(module) => module.receive_pulse(Pulse::Low, broadcaster.as_ref().unwrap().get_name()),
                ModuleType::Broadcaster(_) => panic!("Can't broadcast to yourself!!")
            }
        }

        let mut all_pulses_processed = false;
    
        while !all_pulses_processed {
            let mut pulses_to_send: HashMap<String, LinkedList<Pulse>> = HashMap::new();
    
            for module in modules.values_mut() {            
                match module {
                    ModuleType::FlipFlop(module) => {
                        pulses_to_send.insert(module.get_name(), module.process_pulses());
                    }
                    ModuleType::Conjunction(module) => {
                        pulses_to_send.insert(module.get_name(), module.process_pulses());
                    }
                    ModuleType::Broadcaster(_) => ()
                }
            }
            
            all_pulses_processed = true;
    
            for (sender, pulses_to_send) in pulses_to_send.into_iter() {
                if pulses_to_send.is_empty() {
                    continue;
                }
                
                for pulse in pulses_to_send.into_iter() {                
                    let mut destinations: Vec<String> = Vec::new();
                    match modules.get(&sender).unwrap() {
                        ModuleType::FlipFlop(module) => destinations = module.get_destinations().clone(),
                        ModuleType::Conjunction(module) => destinations = module.get_destinations().clone(),
                        ModuleType::Broadcaster(_) => ()
                    }
                    
                    match pulse {
                        Pulse::Low => low_pulses_sent += destinations.len() as u64,
                        Pulse::High => high_pulses_sent += destinations.len() as u64
                    }
                    
                    for destination in destinations.into_iter() {
                        if modules.get(&destination).is_none() {
                            continue;
                        }
                        
                        match modules.get_mut(&destination).unwrap() {
                            ModuleType::FlipFlop(module) => module.receive_pulse(pulse.clone(), sender.clone()),
                            ModuleType::Conjunction(module) => module.receive_pulse(pulse.clone(), sender.clone()),
                            ModuleType::Broadcaster(_) => ()
                        }
                    }
                }
                
                all_pulses_processed = false;
            }
        }
    }
        
    println!("{}", low_pulses_sent * high_pulses_sent);
}

trait Module {
    fn get_name(&self) -> String;
    fn process_pulses(&mut self) -> LinkedList<Pulse>;
    fn get_destinations(&self) -> Vec<String>;
    fn receive_pulse(&mut self, pulse: Pulse, sender: String);
}

#[derive(Debug)]
enum ModuleType {
    Broadcaster(Broadcaster),
    FlipFlop(FlipFlop),
    Conjunction(Conjunction)
}

#[derive(Debug, Clone)]
struct Broadcaster {
    pulse_destinations: Vec<String>,
}

impl Module for Broadcaster {
    fn get_name(&self) -> String {
        String::from("broadcaster")
    }
    
    fn process_pulses(&mut self) -> LinkedList<Pulse> {
        LinkedList::from([Pulse::Low])
    }
    
    fn get_destinations(&self) -> Vec<String> {
        self.pulse_destinations.clone()
    }
    
    fn receive_pulse(&mut self, pulse: Pulse, sender: String) {
        drop(pulse);
        drop(sender);
    }
}

#[derive(Debug, Clone)]
struct FlipFlop {
    name: String,
    pulse_destinations: Vec<String>,
    state: FlipFlopState,
    pulses_queue: LinkedList<Pulse>
}

impl Module for FlipFlop {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn process_pulses(&mut self) -> LinkedList<Pulse> {
        let mut pulses_to_send: LinkedList<Pulse> = LinkedList::new();
        
        while !self.pulses_queue.is_empty() {
            match self.pulses_queue.pop_front().unwrap() {
                Pulse::High => (),
                Pulse::Low => {
                    match self.state {
                        FlipFlopState::On => {
                            self.state = FlipFlopState::Off;
                            pulses_to_send.push_back(Pulse::Low);
                        }
                        FlipFlopState::Off => {
                            self.state = FlipFlopState::On;
                            pulses_to_send.push_back(Pulse::High);
                        }
                    }
                }
            }
        }

        pulses_to_send
    }

    fn get_destinations(&self) -> Vec<String> {
        self.pulse_destinations.clone()
    }

    fn receive_pulse(&mut self, pulse: Pulse, sender: String) {
        self.pulses_queue.push_back(pulse);
        drop(sender);
    }
}

#[derive(Debug, Clone)]
struct Conjunction {
    name: String,
    pulse_destinations: Vec<String>,
    pulse_memory: HashMap<String, Pulse>,
    pulse_received: bool
}

impl Module for Conjunction {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn process_pulses(&mut self) -> LinkedList<Pulse> {
        if !self.pulse_received {
            return LinkedList::new();
        }
        
        self.pulse_received = false;

        for pulse in self.pulse_memory.values() {
            if pulse == &Pulse::Low {
                return LinkedList::from([Pulse::High]);
            }
        }

        LinkedList::from([Pulse::Low])
    }

    fn get_destinations(&self) -> Vec<String> {
        self.pulse_destinations.clone()
    }

    fn receive_pulse(&mut self, pulse: Pulse, sender: String) {
        self.pulse_memory.insert(sender, pulse);
        self.pulse_received = true;
    }
}

impl Conjunction {
    fn new(name: String, pulse_destinations: Vec<String>, inputs: Vec<String>) -> Self {
        let mut new_conjunction = Conjunction { name, pulse_destinations, pulse_memory: HashMap::new(), pulse_received: false };
        
        for input in inputs.into_iter() {
            new_conjunction.pulse_memory.insert(input, Pulse::Low);
        }
        
        new_conjunction
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum FlipFlopState {
    On,
    Off
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Pulse {
    High,
    Low
}
