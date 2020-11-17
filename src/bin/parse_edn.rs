extern crate clap;

use clap::{App, Arg};
use rustbrew::edn::EdnElement;
use rustbrew::entity::{Entity, EntitySet};
use std::fs;
use std::str::FromStr;

fn main() {
    let matches = App::new("Parse EDN")
        .arg(Arg::with_name("EDN_FILE").required(true))
        .get_matches();

    let edn_file = matches.value_of("EDN_FILE").unwrap();
    let s = fs::read_to_string(edn_file).expect("failed to read EDN from file");
    let element = EdnElement::from_str(&s).expect("failed to parse EDN");

    let json = serde_json::to_string_pretty(&element).expect("failed to serialize EDN");
    let json_file = format!("{}.json", edn_file);
    fs::File::create(&json_file).expect("failed to create JSON file");
    fs::write(json_file, &json).expect("failed to write JSON to file");

    let entity_set: EntitySet =
        serde_json::from_str(&json).expect("failed to deserialize Orcbrew entities");
    let entities: Vec<Entity> = entity_set.into();
    println!("{:?}", entities);
}
