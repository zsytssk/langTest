// extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "csv.pest"]
pub struct CSVParser;

fn main() {
    let unparsed_file = fs::read_to_string("src/number.csv").expect("cannot read file");
    let parse_result = CSVParser::parse(Rule::statement, &unparsed_file);

    match parse_result {
        Ok(mut pairs) => {
            let pair = pairs.next().unwrap();
            println!("{:?}", pair.as_str());
            println!("{:?}", pair);
        }
        Err(_err) => {}
    }
}
