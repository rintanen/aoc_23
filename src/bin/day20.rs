use std::collections::HashMap;


#[derive(Debug, Clone)]
enum Pulse {
    Low,
    High,
}

enum Module {
    Broadcaster(Broadcaster),
    Flipflop(Flipflop),
    Conjunction(Conjunction),
}

trait SendPulse {
    fn send_pulse(&self, modules: &HashMap<String, Module>);
}

struct Broadcaster {
    name: String,
    connections: Vec<String>,
}

impl SendPulse for Broadcaster {
    fn send_pulse(&self, modules: &HashMap<String, Module>) {
        todo!()
    }
}

struct Flipflop {
    name: String,
    connections: Vec<String>,
    on_off: u8,
}

impl SendPulse for Flipflop {
    fn send_pulse(&self, modules: &HashMap<String, Module>) {
        todo!()
    }
}

struct Conjunction {
    name: String,
    connections: Vec<String>,
    inputs: HashMap<String, Pulse>,
}


impl SendPulse for Conjunction {
    fn send_pulse(&self, modules: &HashMap<String, Module>) {
        todo!()
    }
}


fn create_module(module: &str, connections: Vec<String>) -> Module {
    let parts = module.split(" ");
    let module_type = parts.next().unwrap();
    let name = parts.next().unwrap();
    match module_type {
        "broadcaster" => {
            let broadcaster = Broadcaster {
                name: name.to_string(),
                connections,
            };
            Module::Broadcaster(broadcaster)
        },
        "flipflop" => {
            let flipflop = Flipflop {
                name: name.to_string(),
                connections,
                on_off: 0,
            };
            Module::Flipflop(flipflop)
        },
        "conjunction" => {
            let conjunction = Conjunction {
                name: name.to_string(),
                connections,
                inputs: HashMap::new(),
            };
            Module::Conjunction(conjunction)
        },
        _ => panic!("Unknown module type"),
    }
}



fn add_conjunction_module_inputs(module_name: &String, modules: &mut HashMap<String, Module>) {
    let module = modules.get_mut(module_name).unwrap();
    let connections = module.connections.clone();
    for connection in connections.iter() {
        let connection_module = modules.get_mut(connection).unwrap();
        match connection_module.module_type {
            ModuleType::Conjunction(ref mut inputs) => {
                inputs.insert(module_name.clone(), Pulse::Low);
            },
            _ => continue,
        }
    }
}

fn main() {
    let input = include_str!("../../inputs/day20.in");
    let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
    let mut broadcast_queue: Vec<String> = Vec::new();

    for line in input.lines() {
        let mut parts = line.split(" -> ");
        let module = parts.next().unwrap();
        let connections = parts.next().unwrap().split(", ").map(|s| s.to_string()).collect();
        let module = create_module(module, connections);
        let module_name = module.name();
        modules.insert(module.name.clone(), module);
    };

    let module_names = modules.keys().cloned().collect::<Vec<String>>();
    for name in module_names.iter() {
        add_conjunction_module_inputs(name, &mut modules);
    }
    dbg!(modules);

}
