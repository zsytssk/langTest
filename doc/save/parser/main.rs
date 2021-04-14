// extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::fs;

use std::env;

#[derive(Parser)]
#[grammar = "pest/parser.pest"]
pub struct CSVParser;

fn main() {
    let path = env::current_dir().expect("err");
    println!("{}", path.display());
    let unparsed_file = fs::read_to_string("test/number.ts").expect("cannot read file");
    let file = CSVParser::parse(Rule::file, &unparsed_file).expect("unsuccessful parse");

    println!("{:#?}", file);

    // for item in file.into_inner() {
    //     println!("{:?}", item.as_str());
    // }
}
