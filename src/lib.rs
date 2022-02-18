extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::error::Error;

use pest::Parser;

#[derive(Parser)]
#[grammar = "apl.pest"]
struct IdentParser;

pub fn parse(input: &str) -> Result<String, Box<dyn Error>> {
    let pairs = IdentParser::parse(Rule::Main, input)?;

    let mut result = String::new();
    for pair in pairs {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::Phase => {
                    result = format!("shape ({})", ast_from_phase(inner_pair))
                },
                _ => unreachable!()
            };
        }
    }
    Ok(result)
}

fn ast_from_phase(pair: pest::iterators::Pair<Rule>) -> String {
    match pair.as_rule() {
        Rule::Phase => ast_from_phase(pair.into_inner().next().unwrap()),
        Rule::Sum => {
            let mut pair = pair.into_inner();
            let lhs = type_from_some(pair.next().unwrap());
            let rhs = ast_from_phase(pair.next().unwrap());
            format!("Sum ({}) (shape ({}))", lhs, rhs)
        },
        Rule::Reduce => {
            let name = pair.as_rule();
            let lhs = type_from_some(pair.into_inner().next().unwrap());
            format!("{:?} ({})", name, lhs)
        }
        _ => unreachable!()
    }
}

fn type_from_some(pair: pest::iterators::Pair<Rule>) -> String {
    format!("{:?} {}", pair.as_rule(), pair.into_inner().count())
}