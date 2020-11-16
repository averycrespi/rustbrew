extern crate clap;

use clap::{App, Arg};
use rustbrew::edn::EdnStream;
use std::fs;
use std::str::FromStr;

fn main() {
    let matches = App::new("Parse EDN")
        .arg(Arg::with_name("EDN_FILE").required(true))
        .get_matches();
    let edn_file = matches.value_of("EDN_FILE").unwrap();
    let s = fs::read_to_string(edn_file).expect("failed to read EDN file");
    let stream = EdnStream::from_str(&s).unwrap();
    println!("{:?}", stream);
}
