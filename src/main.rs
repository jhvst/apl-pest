extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "apl.pest"]
struct IdentParser;

fn main() {
    let pairs = IdentParser::parse(Rule::Main, "1 2 3 4 + +/ 1 2 3 4").unwrap_or_else(|e| panic!("{}", e));

    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::Phase => {
                    println!("shape ({})", ast_from_phase(inner_pair))
                },
                _ => unreachable!()
            };
        }
    }
}

fn ast_from_phase(pair: pest::iterators::Pair<Rule>) -> String {
    match pair.as_rule() {
        Rule::Phase => ast_from_phase(pair.into_inner().next().unwrap()),
        Rule::Sum => {
            println!("Rule:    {:?}", pair.as_rule());
            println!("Span:    {:?}", pair.as_span());
            println!("Text:    {}", pair.as_str());

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