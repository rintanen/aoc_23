use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Category {
    ExtremelyCoolLooking(u32),
    Musical(u32),
    Aerodynamical(u32),
    Shinny(u32),
}

#[derive(Debug)]
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

#[derive(Debug)]
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
            "s" => Category::Shinny(test_value),
            _ => unreachable!()
        };

        Rule {
            category,
            comp_symbol,
            outcome
        }

    }
//
//     fn test(&self, category: Category) -> Option<Outcome> {
//         // if category is same enum variant as self.category compare the values
//         // else return None
//         if category == self.category {
//             match self.comp_symbol {
//                 '<' => {
//                     if category < self.category {
//                         Some(self.outcome)
//                     } else {
//                         None
//                     }
//                 },
//                 '>' => {
//                     if category > self.category {
//                         Some(self.outcome)
//                     } else {
//                         None
//                     }
//                 },
//         } else {
//             None
//         }
//     }
}

#[derive(Debug)]
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


fn main() {
    let input = include_str!("../../inputs/day19.in");
    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let (wflows, items) = input.split_once("\n\n").unwrap();
    for line in wflows.lines() {
        let workflow = parse_workflow(line);
        workflows.insert(workflow.name.clone(), workflow);
    }

    // let fars = 1;
    dbg!(workflows);
}