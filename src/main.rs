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
    let parse_result = CSVParser::parse(Rule::test, &unparsed_file)
        .unwrap()
        .next()
        .unwrap();

    println!("{:#?}", parse_result);
}
