#![feature(array_from_fn)]
#![feature(map_try_insert)]

pub mod machine;
pub mod parser;
pub mod tape;

//pub type Symbol = Option<String>;

use std::str::FromStr;

pub use machine::*;
pub use tape::*;

pub type Config = String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InputSymbol {
    Symbol(Symbol),
    None,
    Any,
    Else,
}

pub type Operations = Vec<Op>;

#[derive(Debug, Clone)]
pub enum Op {
    L,
    R,
    E,
    P(Symbol),
}

#[derive(Debug, Clone)]
pub struct Statement {
    pub config: Config,
    pub symbol: InputSymbol,
    pub ops: Operations,
    pub final_config: Config,
}
