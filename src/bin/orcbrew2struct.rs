extern crate clap;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use clap::{App, Arg};
use pest::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "orcbrew/orcbrew.pest"]
struct OrcbrewParser;

fn main() {
    let matches = App::new("Convert Orcbrew to a struct")
        .arg(Arg::with_name("ORCBREW").required(true))
        .get_matches();

    let orcbrew_file = matches.value_of("ORCBREW").expect("missing ORCBREW arg");
    let orcbrew = fs::read_to_string(orcbrew_file).expect("failed to read Orcbrew file");
    let _ = OrcbrewParser::parse(Rule::orcbrew, &orcbrew).unwrap_or_else(|e| panic!("{}", e));

    println!("Success!");
}
