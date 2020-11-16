extern crate pest;

use crate::error::RustbrewError;
use pest::iterators::Pair;
use pest::Parser;
use std::str::FromStr;

#[derive(Parser)]
#[grammar = "grammar/edn.pest"]
struct EdnParser;

#[derive(Debug)]
pub enum EdnElement {
    Nil,
    True,
    False,
    String(String),
    Character(String),
    Symbol(String),
    Keyword(String),
    Integer(i64),
    Float(f64),
    List(Vec<EdnElement>),
    Vector(Vec<EdnElement>),
    Set(Vec<EdnElement>),
    Map(Vec<(EdnElement, EdnElement)>),
}

#[derive(Debug)]
pub struct EdnStream {
    elements: Vec<EdnElement>,
}

impl FromStr for EdnStream {
    type Err = RustbrewError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pairs = EdnParser::parse(Rule::stream, s)
            .map_err(|e| RustbrewError::InvalidSyntax(e.to_string()))?;
        match pairs.map(parse_edn_element).collect() {
            Ok(elements) => Ok(EdnStream { elements }),
            Err(e) => Err(e),
        }
    }
}

fn parse_edn_element(pair: Pair<Rule>) -> Result<EdnElement, RustbrewError> {
    match pair.as_rule() {
        Rule::nil => Ok(EdnElement::Nil),
        Rule::boolean => match pair.as_str() {
            "true" => Ok(EdnElement::True),
            "false" => Ok(EdnElement::False),
            _ => panic!("invalid boolean: `{:?}`", pair.as_str()),
        },
        Rule::string => Ok(EdnElement::String(pair.as_str().to_string())),
        Rule::character => Ok(EdnElement::Character(pair.as_str().to_string())),
        Rule::symbol => Ok(EdnElement::Symbol(pair.as_str().to_string())),
        Rule::keyword => Ok(EdnElement::Keyword(pair.as_str().to_string())),
        Rule::integer => Ok(EdnElement::Integer(pair.as_str().parse()?)),
        Rule::float => Ok(EdnElement::Float(pair.as_str().parse()?)),
        Rule::list => match pair.into_inner().map(parse_edn_element).collect() {
            Ok(elements) => Ok(EdnElement::List(elements)),
            Err(e) => Err(e),
        },
        Rule::vector => match pair.into_inner().map(parse_edn_element).collect() {
            Ok(elements) => Ok(EdnElement::Vector(elements)),
            Err(e) => Err(e),
        },
        Rule::set => match pair.into_inner().map(parse_edn_element).collect() {
            Ok(elements) => Ok(EdnElement::Set(elements)),
            Err(e) => Err(e),
        },
        Rule::map => {
            let mut map = vec![];
            for subpair in pair.into_inner() {
                let mut kv_pair = subpair.into_inner();
                let key_pair = kv_pair.next().expect("failed to unwrap key");
                let value_pair = kv_pair.next().expect("failed to unwrap value");
                map.push((parse_edn_element(key_pair)?, parse_edn_element(value_pair)?));
            }
            Ok(EdnElement::Map(map))
        }
        _ => panic!(
            "invalid rule: `{:?}` with value: `{}`",
            pair.as_rule(),
            pair.as_str()
        ),
    }
}
