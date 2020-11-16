extern crate clap;

use clap::{App, Arg};
use rustbrew::edn::EdnStream;
use rustbrew::entity::{Entity, EntitySet};
use std::fs;
use std::str::FromStr;

fn main() {
    let matches = App::new("Parse EDN")
        .arg(Arg::with_name("EDN_FILE").required(true))
        .get_matches();
    let edn_file = matches.value_of("EDN_FILE").unwrap();
    let s = fs::read_to_string(edn_file).expect("failed to read EDN from file");
    let stream = EdnStream::from_str(&s).expect("failed to parse EDN");
    let json = serde_json::to_string_pretty(&stream).expect("failed to serialize EDN");
    let json_file = format!("{}.json", edn_file);
    fs::File::create(&json_file).expect("failed to create JSON file");
    fs::write(json_file, &json).expect("failed to write JSON to file");
    let entity_sets: Vec<EntitySet> =
        serde_json::from_str(&json).expect("failed to deserialize entity sets");
    let mut entities: Vec<Entity> = vec![];
    for entity_set in entity_sets {
        let tmp: Vec<Entity> = entity_set.into();
        entities.extend(tmp);
    }
    println!("{:?}", entities);
}
