//! Types for different kinds of errors.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RustbrewError {
    #[error("invalid syntax: `{0}`")]
    InvalidSyntax(String),

    #[error("invalid integer: `{0}`")]
    InvalidInteger(#[from] std::num::ParseIntError),

    #[error("invalid float: `{0}`")]
    InvalidFloat(#[from] std::num::ParseFloatError),
}
