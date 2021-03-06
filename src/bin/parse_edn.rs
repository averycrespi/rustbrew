extern crate clap;

use clap::{App, Arg};
use rustbrew::edn::EdnElement;
use rustbrew::entity::parse_entities;
use rustbrew::error::RustbrewError;
use std::fs;
use std::str::FromStr;

fn main() {
    let matches = App::new("Parse EDN")
        .arg(Arg::with_name("EDN_FILE").required(true))
        .get_matches();

    let edn_file = matches.value_of("EDN_FILE").unwrap();
    let s = fs::read_to_string(edn_file).expect("failed to read EDN from file");
    let element = EdnElement::from_str(&s).map_err(|e| match e {
        RustbrewError::InvalidSyntax { message } => panic!(message),
        _ => panic!(e),
    });

    let json = serde_json::to_string_pretty(&element).expect("failed to serialize EDN");
    let json_file = format!("{}.json", edn_file);
    fs::File::create(&json_file).expect("failed to create JSON file");
    fs::write(json_file, &json).expect("failed to write JSON to file");

    let entities = parse_entities(&json).expect("failed to parse Orcbrew entities");
    println!("Successfully parsed {} Orcbrew entities!", entities.len());
}
