use thiserror::Error;

use crate::ast::{Token, Value};

#[derive(Debug, Error)]
pub enum LexerError {
    #[error("expected identifier")]
    ExpectedIdentifier,
    #[error("expected token: {0}")]
    ExpectedToken(String),
    #[error("file not found: {0}")]
    FileNotFound(String),
    #[error("bad number")]
    BadNumber,
}

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("unexpected token")]
    UnexpectedToken,
    #[error("expected token: {0}")]
    ExpectedToken(Token),
    #[error("unknown statement")]
    UnexpectedStatement,
    #[error("expected identifier")]
    ExpectedIdent,
}

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("type error: {0}")]
    TypeError(String),
    #[error("unknown identifier: {0}")]
    UnknownIdent(String),
    #[error("unknown function: {0}")]
    UnknownFunction(String),
    #[error("wrong arg count: expected {expected}, got {got}")]
    WrongArgCount { expected: usize, got: usize },
    #[error("index out of bounds")]
    IndexOutOfBounds,
    #[error("not an array")]
    NotAnArray,
    #[error("not an error, a return statement was called")]
    Return(Value),
}
