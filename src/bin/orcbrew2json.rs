extern crate clap;

use clap::{App, Arg};
use edn_rs::Edn;
use std::fs;
use std::str::FromStr;

fn main() {
    let matches = App::new("Convert Orcbrew to JSON")
        .arg(Arg::with_name("ORCBREW").required(true))
        .get_matches();
    let orcbrew = matches.value_of("ORCBREW").expect("missing Orcbrew");
    let s = fs::read_to_string(orcbrew).unwrap().replace('\u{feff}', "");
    let edn = Edn::from_str(&s).unwrap();
    let json = edn.to_json();
    println!("{}", json);
}
