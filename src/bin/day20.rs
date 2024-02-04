use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug)]
enum Module {
    Broadcaster(Broadcaster),
    Flipflop(Flipflop),
    Conjunction(Conjunction),
}

impl Module {
    fn name(&self) -> &str {
        match self {
            Module::Broadcaster(module) => module.name.as_str(),
            Module::Flipflop(module) => module.name.as_str(),
            Module::Conjunction(module) => module.name.as_str(),
        }
    }
}

#[derive(Debug)]
enum FlipflopState {
    On,
    Off,
}

trait Transmittable {
    fn transmit(&self, modules: &mut HashMap<String, Module>);
}

trait Receivable {
    fn receive(&mut self, from: &str, pulse: &Pulse);
}

#[derive(Debug)]
struct Broadcaster {
    name: String,
    connections: Vec<String>,
}

impl Transmittable for Broadcaster {
    fn transmit(&self, modules: &mut HashMap<String, Module>) {
        let pulse = Pulse::Low;
        for connection in self.connections.iter() {
            let module = modules.get_mut(connection).unwrap();
            match module {
                Module::Conjunction(conj) => conj.receive(&self.name, &pulse),
                Module::Flipflop(flipflop) => flipflop.receive(&self.name, &pulse),
                Module::Broadcaster(_) => unreachable!("Broadcaster should not be a connection"),
            }
        }
    }
}

#[derive(Debug)]
struct Flipflop {
    name: String,
    connections: Vec<String>,
    state: FlipflopState,
    ignored_last_receive: bool,
}

impl Flipflop {
    fn flip(&mut self) {
        self.state = match self.state {
            FlipflopState::Off => FlipflopState::On,
            FlipflopState::On => FlipflopState::Off,
        }
    }
}

impl Transmittable for Flipflop {
    fn transmit(&self, modules: &mut HashMap<String, Module>) {
        if self.ignored_last_receive {
            return;
        }
        let pulse = match self.state {
            FlipflopState::Off => Pulse::Low,
            FlipflopState::On => Pulse::High,
        };

        for connection in self.connections.iter() {
            let module = modules.get_mut(connection).unwrap();
            match module {
                Module::Conjunction(conj) => conj.receive(&self.name, &pulse),
                Module::Flipflop(flipflop) => flipflop.receive(&self.name, &pulse),
                Module::Broadcaster(_) => unreachable!("Broadcaster should not be a connection"),
            }
        }
    }
}

impl Receivable for Flipflop{
    fn receive(&mut self, from: &str, pulse: &Pulse) {
        // Flipflop doesn't need to know who it received from
        // Do nothing with the 'from' information

        // if pulse is low, flip the state
        // if pulse is high, dont do anything
        match pulse {
            Pulse::Low => {
                self.ignored_last_receive = false;
                self.flip()
            },
            Pulse::High => self.ignored_last_receive = true,
        }
    }
}

#[derive(Debug)]
struct Conjunction {
    name: String,
    connections: Vec<String>,
    inputs: HashMap<String, Pulse>,
}


impl Transmittable for Conjunction {
    fn transmit(&self, modules: &mut HashMap<String, Module>) {
        let all_high = self.inputs
            .iter()
            .all(|(_, pulse)| match pulse {
                Pulse::High => true,
                Pulse::Low => false
                }
            );
        let pulse = if all_high {
            Pulse::High
        } else {
            Pulse::Low
        };
        for connection in self.connections.iter() {
            let module = modules.get_mut(connection).unwrap();
            match module {
                Module::Conjunction(conj) => conj.receive(&self.name, &pulse),
                Module::Flipflop(flipflop) => flipflop.receive(&self.name, &pulse),
                Module::Broadcaster(_) => unreachable!("Broadcaster should not be a connection"),
            }
        }
    }
}

impl Receivable for Conjunction {
    fn receive(&mut self, from: &str, pulse: &Pulse) {
        self.inputs.insert(from.to_string(), pulse.clone());
    }
}


fn create_module(module: &str, connections: Vec<String>) -> Module {
    if module.starts_with("broadcaster") {
        Module::Broadcaster(Broadcaster {
            name: module.to_string(),
            connections,
        })
    } else if module.starts_with("%") {
        Module::Flipflop(Flipflop {
            name: module[1..].to_string(),
            connections,
            state: FlipflopState::Off,
            ignored_last_receive: false,
        })
    } else if module.starts_with("&") {
        Module::Conjunction(Conjunction {
            name: module[1..].to_string(),
            connections,
            inputs: HashMap::new(),
        })
    } else {
        panic!("Unknown module type: {}", module);
    }
}


fn add_conjunction_module_inputs(module_name: &String, modules: &mut HashMap<String, Module>) {
    let connections = match modules.get(module_name).unwrap() {
        Module::Conjunction(conjunction) => conjunction.connections.clone(),
        Module::Broadcaster(broadcaster) => broadcaster.connections.clone(),
        Module::Flipflop(flipflop) => flipflop.connections.clone(),
    };

    for connection in connections.iter() {
        let connection = modules.get_mut(connection).unwrap();
        match connection {
            Module::Conjunction(connection) => {
                connection.inputs.insert(module_name.clone(), Pulse::Low);
            },
            _ => (),
        }
    }
}



fn main() {
    let input = include_str!("../../inputs/day20.in");
    let mut modules: HashMap<String, Module> = HashMap::new();

    for line in input.lines() {
        let mut parts = line.split(" -> ");
        let module = parts.next().unwrap();
        let connections = parts.next().unwrap().split(", ").map(|s| s.to_string()).collect();
        let module = create_module(module, connections);
        let module_name = module.name();
        modules.insert(module_name.to_string(), module);
    };

    let module_names = modules.keys().cloned().collect::<Vec<String>>();
    for name in module_names.iter() {
        add_conjunction_module_inputs(name, &mut modules);
    }
    dbg!(modules);

}
