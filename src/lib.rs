use pest::error::Error;
use pest::Parser;
use pest::Token;
use pest_derive::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "air.pest"]
pub struct AIRParser;

#[derive(Default, Debug)]
pub struct Action {
    number: u64,
}

struct ActionBuilder {
    number: u64,
}

impl ActionBuilder {
    fn new() -> ActionBuilder {
        ActionBuilder { number: 0 }
    }

    fn number(mut self, number: u64) -> Self {
        self.number = number;
        self
    }

    fn build(&self) -> Action {
        Action {
            number: self.number,
        }
    }
}

fn build_action(pairs: pest::iterators::Pairs<Rule>) -> Action {
    let mut action_builder = ActionBuilder::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::action_def => {
                let mut action_number_rule = pair.into_inner();
                let action_number_str = action_number_rule.next().unwrap().as_str();
                let action_number = str::parse(action_number_str).unwrap();
                action_builder = action_builder.number(action_number);
            }
            Rule::action_element => {
                dbg!("action_element");
            }
            Rule::clsn => {
                dbg!("clsn");
            }
            Rule::interpolation_group => {
                dbg!("interpolation_group");
            }
            _ => {
                dbg!(pair.as_rule());
            }
        }
    }

    action_builder.build()
}

pub fn parse(source: &str) -> Result<HashMap<u64, Action>, Error<Rule>> {
    let mut actions = HashMap::new();

    let files = AIRParser::parse(Rule::file, source)?;
    dbg!(files);
    Ok(actions)
}
