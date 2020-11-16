extern crate clap;

use clap::{App, Arg};
use rustbrew::edn::EdnStream;
use rustbrew::entity::EntitySet;
use serde_json;
use std::fs;
use std::str::FromStr;

fn main() {
    let matches = App::new("Parse EDN")
        .arg(Arg::with_name("EDN_FILE").required(true))
        .get_matches();
    let edn_file = matches.value_of("EDN_FILE").unwrap();
    let s = fs::read_to_string(edn_file).expect("failed to read EDN file");
    let stream = EdnStream::from_str(&s).expect("failed to parse EDN");
    let json = serde_json::to_string(&stream).expect("failed to serialize EDN");
    println!("{}", json);
    let entities: EntitySet = serde_json::from_str(&json).expect("failed to deserialize entities");
    println!("{:?}", entities);
}
