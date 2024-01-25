use std::collections::HashMap;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Category {
    ExtremelyCoolLooking(u32),
    Musical(u32),
    Aerodynamical(u32),
    Shiny(u32),
}

impl Category {
    fn from_str(s: &str) -> Self {
        let (category, value) = s.split_once("=").unwrap();
        let value = value.parse::<u32>().unwrap();
        match category {
            "x" => Category::ExtremelyCoolLooking(value),
            "m" => Category::Musical(value),
            "a" => Category::Aerodynamical(value),
            "s" => Category::Shiny(value),
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Clone)]
enum Outcome {
    Accept,
    Reject,
    AnotherWorkflow(String),
}

impl Outcome {
    fn from_str(s: &str) -> Self {
        match s {
            "A" => Outcome::Accept,
            "R" => Outcome::Reject,
            _ => Outcome::AnotherWorkflow(s.to_string())
        }
    }
}

#[derive(Debug, Clone)]
struct Rule {
    category: Category,
    comp_symbol: char,
    outcome: Outcome,
}

impl Rule {
    fn from_str(s: &str) -> Self {
        let (comparison, outcome) = s.split_once(":").unwrap();

        let outcome = Outcome::from_str(outcome);

        let i_comp_symbol = comparison.find(|c| c == '<' || c == '>').unwrap();
        let category = &comparison[..i_comp_symbol];
        let test_value = comparison[i_comp_symbol + 1..].parse::<u32>().unwrap();
        let comp_symbol = comparison.chars().nth(i_comp_symbol).unwrap();

        let category = match category {
            "x" => Category::ExtremelyCoolLooking(test_value),
            "m" => Category::Musical(test_value),
            "a" => Category::Aerodynamical(test_value),
            "s" => Category::Shiny(test_value),
            _ => unreachable!()
        };

        Rule {
            category,
            comp_symbol,
            outcome
        }
    }
    fn test(&self, part: &Part) -> Option<Outcome> {
        match self.category {
            Category::ExtremelyCoolLooking(x) => {
                if self.compare(part.cool, x) {
                    Some(self.outcome.clone())
                } else {
                    None
                }
            },
            Category::Musical(m) => {
                if self.compare(part.musical, m) {
                    Some(self.outcome.clone())
                } else {
                    None
                }
            },
            Category::Aerodynamical(a) => {
                if self.compare(part.aerodynamic, a) {
                    Some(self.outcome.clone())
                } else {
                    None
                }
            },
            Category::Shiny(s) => {
                if self.compare(part.shiny, s) {
                    Some(self.outcome.clone())
                } else {
                    None
                }
            },
        }
    }

    fn compare(&self, a: u32, b: u32) -> bool {
        match self.comp_symbol {
            '<' => a < b,
            '>' => a > b,
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Clone)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    finally: Outcome
}

fn parse_rules_and_finally(s: &str) -> (Vec<Rule>, Outcome) {
    let mut rules = Vec::new();
    let mut finally: Outcome = Outcome::Accept;

    for part in s.split(',') {
        if part.contains(":") {
            let rule = Rule::from_str(part);
            rules.push(rule);
        } else {
            finally = Outcome::from_str(part);
        }
    }
    (rules, finally)
}


fn parse_workflow(workflow: &str) -> Workflow {
    let brace_idx = workflow.find("{").unwrap();
    let name = workflow[..brace_idx].trim().to_string();
    let rules = workflow[brace_idx + 1..workflow.len() - 1].trim();
    let (rules, finally) = parse_rules_and_finally(rules);
    Workflow {
        name,
        rules,
        finally
    }
}

#[derive(Debug)]
struct Part {
    cool: u32,
    musical: u32,
    aerodynamic: u32,
    shiny: u32
}



fn process_part(workflows: &HashMap<String, Workflow>, part: &Part) -> Outcome {
    // start by putting the part to 'in' workflow
    let mut current_workflow = workflows.get("in").unwrap();
    loop {
        let mut final_outcome_determined = false;
        for rule in current_workflow.rules.iter() {
            match rule.test(part)  {
                Some(Outcome::Accept) => {
                    return Outcome::Accept;
                },
                Some(Outcome::Reject) => {
                    return Outcome::Reject;
                },
                Some(Outcome::AnotherWorkflow(name)) => {
                    current_workflow = workflows.get(&name).unwrap();
                    final_outcome_determined = true;
                    break;
                },
                None => {
                    continue;
                }
            }
        }

        // only test finally if no rule matched
        if !final_outcome_determined {
            match &current_workflow.finally {
                Outcome::Accept => {
                    return Outcome::Accept;
                },
                Outcome::Reject => {
                    return Outcome::Reject;
                },
                Outcome::AnotherWorkflow(name) => {
                    current_workflow = workflows.get(name).unwrap();
                }
            }
        }
    }
}


fn main() {
    let input = include_str!("../../inputs/day19.in");
    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let (wflows, items) = input.split_once("\n\n").unwrap();
    for line in wflows.lines() {
        let workflow = parse_workflow(line);
        workflows.insert(workflow.name.clone(), workflow);
    }

    let mut parts: Vec<Part> = Vec::new();
    for line in items.lines() {
        let (x, m, a, s) = {
            line[1..line.len() - 1]
                .split(',')
                .map(|s| {
                    let (_, amount) = s.split_once("=").unwrap();
                    amount.parse::<u32>().unwrap()
                })
                .collect_tuple()
                .unwrap()
        };
        let part = Part {
            cool: x,
            musical: m,
            aerodynamic: a,
            shiny: s
        };
        parts.push(part);
    }

    let mut accepted: Vec<Part> = Vec::new();
    let mut rejected: Vec<Part> = Vec::new();
    for part in parts.into_iter() {
        let outcome = process_part(&workflows, &part);
        match outcome {
            Outcome::Accept => {
                accepted.push(part);
            },
            Outcome::Reject => {
                rejected.push(part);
            },
            _ => unreachable!()
        }
    }

    let sum_of_ratings: u32 = accepted.iter().map(|p| p.cool + p.musical + p.aerodynamic + p.shiny).sum();
    dbg!(sum_of_ratings);


}