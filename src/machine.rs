use crate::tape::{Cell, Tape};
use crate::{Config, InputSymbol, Op, Operations, Statement, Symbol};

use std::collections::HashMap;
use std::str::FromStr;

pub type Statements = HashMap<Config, HashMap<InputSymbol, (Operations, Config)>>;

#[derive(Debug, Clone)]
pub struct TuringMachine {
    tape: Tape,
    head: isize,
    config: Config,
    statements: Statements,
}

impl TuringMachine {
    pub fn new(tape: Tape, start: Config) -> Self {
        Self {
            tape,
            head: 0,
            config: start,
            statements: Statements::new(),
        }
    }

    pub fn tape(&self) -> &Tape {
        &self.tape
    }

    pub fn add_statement(&mut self, stmt: Statement) -> Result<(), String> {
        let config_entry = self.statements.entry(stmt.config).or_insert(HashMap::new());

        match config_entry.try_insert(stmt.symbol, (stmt.ops, stmt.final_config)) {
            Err(_) => Err(String::from(
                "more than one statement with same config and symbol",
            )),
            Ok(_) => Ok(()),
        }
    }

    pub fn step(&mut self) -> Option<(&Config, &Cell)> {
        //let state = self.state.clone();
        //let symbol = match self.tape[self.head] {
        //    Some(ref s) => Symbol::Some(s.to_string()),
        //    None => Symbol::None,
        //};

        // get symbol from current head location
        let cell = &self.tape[self.head];
        let config = self.statements.get(&self.config)?;

        let (ops, next_state) = match cell {
            Cell::None => config.get(&InputSymbol::None),
            Cell::Some(s) => config
                .get(&InputSymbol::Any)
                .or(config.get(&InputSymbol::Symbol(s.clone()))),
        }
        .or(config.get(&InputSymbol::Else))?;

        // find state current state derivations for symbol (Any, None, etc)
        //let (ops, next_state) = match self.transition.get(&self.state)?.symbols {
        //    SymbolLookup::Any(ops, next_state) => &(ops, next_state),
        //    SymbolLookup::Symbols(map) => map.get(&input_symbol)?,
        //};

        // update based on instructions
        for op in ops {
            match op {
                Op::L => self.head -= 1,
                Op::R => self.head += 1,
                Op::E => self.tape[self.head] = Cell::None,
                Op::P(s) => self.tape[self.head] = Cell::Some(s.clone()),
            }
        }

        self.config.clear();
        self.config.push_str(next_state);
        Some((&self.config, &self.tape[self.head]))
    }
}

use crate::parser::parse_statement;

impl FromStr for TuringMachine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stmts = Vec::new();
        for line in s.lines() {
            if line.is_empty() || line.starts_with("#") {
                continue;
            }
            stmts.push(parse_statement(line)?);
        }

        if stmts.is_empty() {
            return Err(String::from("empty description"));
        }

        let mut tm = TuringMachine::new(Tape::new(), stmts[0].config.clone());
        for (i, stmt) in stmts.into_iter().enumerate() {
            tm.add_statement(stmt)
                .map_err(|err| format!("`{err}` in line `{i}`"))?;
        }
        Ok(tm)
    }
}

//#[macro_export]
//macro_rules! transition {
//    () => {{
//        $crate::machine::Transition::new()
//    }};
//
//    ($($ops))
//    ($symbol:tt $(= $($op:expr),+)?) => {{
//        let mut ops = Vec::new();
//        $(
//            $(
//                ops.push($op);
//            )+
//        )?
//        (stringify!($symbol).to_string(), ops)
//    }};
//    ($( $state:tt { $( $($symbol:tt)|+ $(= $($op:expr),+)? => $final:tt);+ $(;)?})*) => {{
//        let mut t = Transition::new();
//
//        $(
//            let state = stringify!($state);
//            $(
//                let final_state = stringify!($final);
//                $(
//                    let tuple = transition!($symbol $(= $op)?);
//                    t.insert((state.to_string(), Some(tuple.0)), (tuple.1, final_state.to_string()));
//                )+
//            )+
//        )*
//        t
//
//    }};
//}
