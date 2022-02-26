mod lib_test;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::error::Error;

use pest::Parser;

#[derive(Parser)]
#[grammar = "apl.pest"]
struct IdentParser;

pub fn phases(s: &str) -> Vec<String> {
    let mut phases = s.split("shape ")
        .filter(|substr| !substr.trim().is_empty())
        .collect::<Vec<_>>();

    phases.reverse();

    let mut prev_phase = String::new();
    phases.iter()
        .enumerate()
        .map(|(index, phase)| {

            if index == phases.len()-1 {
                return s.to_owned()
            }

            let enclosures = phase
                .chars()
                .filter(|c| c.eq(&'('))
                .count();

            let closures = phase
                .chars()
                .enumerate()
                .filter_map(|(index, c)| match c.eq(&')') {
                    true => Some(index),
                    false => None,
                })
                .collect::<Vec<_>>();

            let filter = closures.get(0..closures.len()-enclosures).unwrap();

            let filtered_phase = phase
                .chars()
                .enumerate()
                .filter_map(|(index, char)| match filter.contains(&index) {
                    true => None,
                    false => Some(char),
                })
                .collect::<String>();

            prev_phase = format!("shape {}", filtered_phase);
            format!("shape {}", filtered_phase)
        })
        .collect::<Vec<String>>()
}

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
            let next = pair.into_inner().next().unwrap();
            let rule = next.as_rule();
            match rule {
                Rule::IndexGenerator => {
                    format!("{:?} (shape ({}))", name, val_from_some(next))
                },
                Rule::SomeScalar
                | Rule::SomeVect => {
                    format!("{:?} ({})", name, type_from_some(next))
                },
                _ => {
                    format!("{:?} (shape ({}))", name, ast_from_phase(next))
                },
            }
        }
        Rule::IndexGenerator => {
            let name = pair.as_rule();
            format!("{:?} {}", name, val_from_some(pair.into_inner().next().unwrap()))
        }
        _ => unreachable!()
    }
}

fn type_from_some(pair: pest::iterators::Pair<Rule>) -> String {
    format!("{:?} {}", pair.as_rule(), pair.into_inner().count())
}

// Some functions, like iota, need this.
fn val_from_some(pair: pest::iterators::Pair<Rule>) -> String {
    pair.as_str().to_string()
}