//! Types and methods for parsing EDN elements.

extern crate pest;

use crate::error::RustbrewError;
use pest::iterators::Pair;
use pest::Parser;
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};
use std::str::FromStr;

#[derive(Parser)]
#[grammar = "grammar/edn.pest"]
struct EdnParser;

/// A single EDN element.
///
/// Maps may be nested, so entries must be represented as key-value pairs.
#[derive(Debug, Clone)]
pub enum EdnElement {
    Nil,
    True,
    False,
    String(String),
    Symbol(String),
    Keyword(String),
    Integer(i64),
    Float(f64),
    List(Vec<EdnElement>),
    Vector(Vec<EdnElement>),
    Set(Vec<EdnElement>),
    Map(Vec<(EdnElement, EdnElement)>),
}

impl Serialize for EdnElement {
    /// Serializes an EDN element.
    ///
    /// This serialization is lossy and cannot be reversed.
    /// Symbols and keywords are serialized to strings.
    /// Lists, vectors, and sets are serialized to sequences.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            EdnElement::Nil => serializer.serialize_none(),
            EdnElement::True => serializer.serialize_bool(true),
            EdnElement::False => serializer.serialize_bool(false),
            EdnElement::String(s) => {
                // Remove leading and trailing double-quote.
                serializer.serialize_str(&s[1..s.len() - 1])
            }
            EdnElement::Symbol(s) => serializer.serialize_str(s),
            EdnElement::Keyword(s) => {
                // Remove trailing colon, then replace dashes with underscores.
                serializer.serialize_str(&s.trim_start_matches(":").replace("-", "_"))
            }
            EdnElement::Integer(i) => serializer.serialize_i64(*i),
            EdnElement::Float(f) => serializer.serialize_f64(*f),
            EdnElement::List(elements)
            | EdnElement::Vector(elements)
            | EdnElement::Set(elements) => {
                let mut seq = serializer.serialize_seq(Some(elements.len()))?;
                for e in elements {
                    seq.serialize_element(e)?;
                }
                seq.end()
            }
            EdnElement::Map(pairs) => {
                let mut map = serializer.serialize_map(Some(pairs.len()))?;
                for p in pairs {
                    map.serialize_entry(&p.0, &p.1)?;
                }
                map.end()
            }
        }
    }
}

/// A stream of EDN elements.
#[derive(Debug, Clone)]
pub struct EdnStream {
    elements: Vec<EdnElement>,
}

impl Serialize for EdnStream {
    /// Serializes a stream of EDN elements.
    ///
    /// The elements are wrapped in a top-level sequence.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.elements.len()))?;
        for e in self.elements.iter().cloned() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}

impl FromStr for EdnStream {
    type Err = RustbrewError;

    /// Convert a string to a stream of EDN elements.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pairs = EdnParser::parse(Rule::stream, s)
            .map_err(|e| RustbrewError::InvalidSyntax(e.to_string()))?;
        match pairs.map(parse_edn_element).collect() {
            Ok(elements) => Ok(EdnStream { elements }),
            Err(e) => Err(e),
        }
    }
}

/// Parses a single EDN element from a Pest pair.
///
/// # Panics
///
/// Panics if any Pest pairs are invalid.
/// This should only happen if there is an error in the grammar.
fn parse_edn_element(pair: Pair<Rule>) -> Result<EdnElement, RustbrewError> {
    match pair.as_rule() {
        Rule::nil => Ok(EdnElement::Nil),
        Rule::boolean => match pair.as_str() {
            "true" => Ok(EdnElement::True),
            "false" => Ok(EdnElement::False),
            _ => panic!("invalid boolean: `{:?}`", pair.as_str()),
        },
        Rule::string => Ok(EdnElement::String(pair.as_str().to_string())),
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
