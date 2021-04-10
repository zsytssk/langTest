// extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::fs;

use std::env;

#[derive(Parser)]
#[grammar = "csv.pest"]
pub struct CSVParser;

fn main() {
    let path = env::current_dir().expect("err");
    println!("{}", path.display());
    let unparsed_file = fs::read_to_string("src/number.csv").expect("cannot read file");
    let file = CSVParser::parse(Rule::json, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next()
        .unwrap();

    for item in file.into_inner() {
        println!("{:?}", item.as_str());
    }
}
