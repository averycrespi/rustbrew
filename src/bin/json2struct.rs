use clap::{App, Arg};
use rustbrew::entities::EntitySource;
use std::collections::HashMap;
use std::fs;

extern crate clap;

fn main() {
    let matches = App::new("Convert JSON to a struct")
        .arg(Arg::with_name("JSON").required(true))
        .arg(Arg::with_name("megapak").short("m").long("megapak"))
        .get_matches();

    let json_file = matches.value_of("JSON").expect("missing JSON argument");
    let json = fs::read_to_string(&json_file).expect("failed to read JSON file");

    if matches.is_present("megapak") {
        let _: HashMap<String, EntitySource> =
            serde_json::from_str(&json).expect("failed to parse megapak");
    } else {
        let _: EntitySource = serde_json::from_str(&json).expect("failed to parse source");
    }
    println!("Success!");
}
