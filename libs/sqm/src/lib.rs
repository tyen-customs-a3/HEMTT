// SQM file format parser for HEMTT
//
// This library provides utilities for parsing SQM files used in Arma games.
// The IntegratedLexer implementation efficiently processes input for fast
// tokenization and class boundary detection in a single pass.

use std::collections::HashMap;

mod parser;
mod lexer;
mod scanner;

pub use parser::{parse_sqm, parse_sqm_with_config, emit_diagnostics, ParseError, ParallelConfig};
pub use scanner::{BoundaryMap, ClassBoundary, ScanError};
pub use lexer::{Token, TokenPosition, IntegratedLexer};

#[derive(Debug, Clone, PartialEq)]
pub struct SqmFile {
    pub version: Option<i32>,
    pub defines: Vec<String>,
    pub classes: HashMap<String, Vec<Class>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    pub name: String,
    pub properties: HashMap<String, Value>,
    pub classes: HashMap<String, Vec<Class>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    Integer(i64),
    Array(Vec<Value>),
    Boolean(bool),
}
