use clap::{App, Arg};
use orcbrew::entity;
use std::fs;

extern crate clap;

fn main() {
    let matches = App::new("Convert JSON to a struct")
        .arg(Arg::with_name("JSON").required(true))
        .arg(Arg::with_name("megapak").short("m").long("megapak"))
        .get_matches();
    let json = matches.value_of("JSON").expect("missing JSON");
    let str = fs::read_to_string(&json).unwrap();
    if matches.is_present("megapak") {
        let _: entity::EntityMegaPak = serde_json::from_str(&str).unwrap();
    } else {
        let _: entity::EntitySource = serde_json::from_str(&str).unwrap();
    }
}
