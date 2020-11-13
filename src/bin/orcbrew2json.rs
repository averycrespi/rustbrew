extern crate clap;

use clap::{App, Arg};
use edn_rs::Edn;
use std::fs;
use std::str::FromStr;

fn main() {
    let matches = App::new("Convert Orcbrew to JSON")
        .arg(Arg::with_name("ORCBREW").required(true))
        .get_matches();

    let orcbrew_file = matches
        .value_of("ORCBREW")
        .expect("missing ORCBREW argument");
    let orcbrew = fs::read_to_string(orcbrew_file)
        .expect("failed to read Orcbrew file")
        .replace('\u{feff}', "");
    let edn = Edn::from_str(&orcbrew).expect("failed to convert string to EDN");
    let json = edn.to_json();

    println!("{}", json);
}
