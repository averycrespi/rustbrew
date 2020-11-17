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

/// An EDN element.
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
    // Maps may be nested, so use key-value tuples.
    Map(Vec<(EdnElement, EdnElement)>),
}

impl FromStr for EdnElement {
    type Err = RustbrewError;

    /// Convert a string to an EDN element.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pairs =
            EdnParser::parse(Rule::stream, s).map_err(|e| RustbrewError::InvalidSyntax {
                message: e.to_string(),
            })?;
        match pairs.next() {
            Some(p) => parse_edn_element(p),
            None => Err(RustbrewError::MissingPair {
                expected: "top-level element".to_string(),
            }),
        }
    }
}

/// Parses an EDN element from a Pest pair.
fn parse_edn_element(pair: Pair<Rule>) -> Result<EdnElement, RustbrewError> {
    match pair.as_rule() {
        Rule::nil => Ok(EdnElement::Nil),
        Rule::boolean => match pair.as_str() {
            "true" => Ok(EdnElement::True),
            "false" => Ok(EdnElement::False),
            _ => Err(RustbrewError::InvalidBoolean {
                value: pair.as_str().to_string(),
            }),
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
            let mut entries = vec![];
            for subpair in pair.into_inner() {
                let mut kv_pair = subpair.into_inner();
                if let (Some(key_pair), Some(value_pair)) = (kv_pair.next(), kv_pair.next()) {
                    entries.push((parse_edn_element(key_pair)?, parse_edn_element(value_pair)?));
                } else {
                    return Err(RustbrewError::MissingPair {
                        expected: "key or value".to_string(),
                    });
                }
            }
            Ok(EdnElement::Map(entries))
        }
        _ => Err(RustbrewError::UnexpectedPair {
            rule: format!("{:?}", pair.as_rule()),
            value: pair.as_str().to_string(),
        }),
    }
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
