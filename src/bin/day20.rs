use std::collections::HashMap;


#[derive(Debug, Clone)]
enum Pulse {
    Low,
    High,
}

//
// #[derive(Debug, Clone)]
// enum ModuleType {
//     Broadcaster,
//     Flipflop(u8),
//     Conjunction(HashMap<String, Pulse>),
// }
//
//
// #[derive(Debug, Clone)]
// struct Module {
//     name: String,
//     module_type: ModuleType,
//     connections: Vec<String>,
// }
//
//


trait Module {
    fn name(&self) -> String;
    fn broadcast(&self, modules: &HashMap<String, Box<dyn Module>>);
}

struct Broadcaster {
    name: String,
    connections: Vec<String>,
}

impl Module for Broadcaster {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn broadcast(&self, modules: &HashMap<String, Box<dyn Module>>) {
        todo!()
    }
}

struct Flipflop {
    name: String,
    connections: Vec<String>,
    on_off: u8,
}

impl Module for Flipflop {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn broadcast(&self, modules: &HashMap<String, Box<dyn Module>>) {
        todo!()
    }
}

struct Conjunction {
    name: String,
    connections: Vec<String>,
    inputs: HashMap<String, Pulse>,
}


impl Module for Conjunction {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn broadcast(&self, modules: &HashMap<String, Box<dyn Module>>) {
        todo!()
    }
}


fn create_module(module: &str, connections: Vec<String>) -> Box<dyn Module> {
    if module.starts_with("broadcaster") {
        Box::new(Broadcaster {
            name: module.to_string(),
            connections,
        }) as Box<dyn Module>
    } else if module.starts_with("%") {
        let on_off = 0;
        Box::new(Flipflop {
            name: module[1..].to_string(),
            connections,
            on_off,
        }) as Box<dyn Module>
    } else if module.starts_with("&") {
        let inputs: HashMap<String, Pulse> = HashMap::new();
        Box::new(Conjunction {
            name: module[1..].to_string(),
            connections,
            inputs,
        }) as Box<dyn Module>
    } else {
        unreachable!("Unknown module type");
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
