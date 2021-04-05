// extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::{collections::HashMap, fs};

#[derive(Parser)]
#[grammar = "csv.pest"]
pub struct CSVParser;

fn main() {
    let unparsed_file = fs::read_to_string("src/number.csv").expect("cannot read file");
    let parse_result = CSVParser::parse(Rule::file, &unparsed_file)
        .unwrap()
        .next()
        .unwrap()
        .into_inner();

    let mut properties: HashMap<&str, HashMap<&str, &str>> = HashMap::new();
    let mut current_section_name = "";
    for item in parse_result {
        match item.as_rule() {
            Rule::section => {
                let mut inner_rules = item.into_inner();
                current_section_name = inner_rules.next().unwrap().as_str();
            }
            Rule::property => {
                let mut inner_rules = item.into_inner();
                let name = inner_rules.next().unwrap().as_str();
                let value = inner_rules.next().unwrap().as_str();

                let section = properties.entry(current_section_name).or_default();
                section.insert(name, value);
            }
            Rule::EOI => (),
            _ => {
                println!("error {} xdxx", item.as_str())
            }
        }
    }

    println!("{:#?}", properties);
}
