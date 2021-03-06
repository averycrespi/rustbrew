//! Types for different kinds of errors.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RustbrewError {
    #[error("invalid syntax: {message}")]
    InvalidSyntax { message: String },

    #[error("invalid boolean: `{value}`")]
    InvalidBoolean { value: String },

    #[error("invalid integer: `{0}`")]
    InvalidInteger(#[from] std::num::ParseIntError),

    #[error("invalid float: `{0}`")]
    InvalidFloat(#[from] std::num::ParseFloatError),

    #[error("unexpected pair with rule: `{rule}` and value: `{value}`")]
    UnexpectedPair { rule: String, value: String },

    #[error("missing pair: expected `{expected}`")]
    MissingPair { expected: String },
}
