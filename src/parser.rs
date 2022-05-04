use crate::{Config, InputSymbol, Op, Operations, Statement, Symbol};

/*
Turing Machine Notation

m-config.   symbol      operations      final m-config
    a   :     w    =     P(0), R    ->     b
    b   :     x    =     R          ->     c
    c   :     y    =     P(1), R    ->     d
    d   :     z    =     R          ->     a
 */

pub fn parse_statement(s: &str) -> Result<Statement, String> {
    // extract config
    let err = format!("malformed statement `{s}`");

    let [config, rest] = extract(s.split(":")).ok_or(err.clone())?;
    let config = parse_config(config.trim())?;

    // extract symbol
    let [symbol, rest] = extract(rest.split("=")).ok_or(err.clone())?;
    let symbol = parse_input_symbol(symbol.trim())?;

    // extract operations and final config
    let [ops, final_config] = extract(rest.split("->")).ok_or(err.clone())?;
    let ops = parse_ops(ops.trim())?;
    let final_config = parse_config(final_config.trim())?;

    Ok(Statement {
        config,
        symbol,
        ops,
        final_config,
    })
}

fn parse_ops(ops: &str) -> Result<Operations, String> {
    let ops = ops.trim();
    let mut v = Operations::new();
    if ops.is_empty() {
        return Ok(v);
    }
    for op in ops.split(",").map(str::trim) {
        v.push(parse_op(op)?);
    }
    Ok(v)
}

fn parse_op(op: &str) -> Result<Op, String> {
    match op {
        "L" => return Ok(Op::L),
        "R" => return Ok(Op::R),
        "E" => return Ok(Op::E),
        _ => (),
    };

    if op.starts_with("P(") && op.ends_with(")") {
        let inner = &op[2..(op.len() - 1)];
        let symbol = parse_symbol(inner).map_err(|e| format!("error parsing Op: {e}"))?;

        Ok(Op::P(symbol))
    } else {
        Err(format!("error parsing Op: unrecognized Op `{op}`"))
    }
}

fn parse_input_symbol(s: &str) -> Result<InputSymbol, String> {
    Ok(match s {
        "Any" | "any" => InputSymbol::Any,
        "None" | "none" => InputSymbol::None,
        "Else" | "else" => InputSymbol::Else,
        s => InputSymbol::Symbol(parse_symbol(s)?),
    })
}

fn parse_symbol(s: &str) -> Result<Symbol, String> {
    if is_ident(s) {
        Ok(Symbol(s.to_string()))
    } else {
        Err(format!("error parsing symbol `{s}`"))
    }
}

fn parse_config(s: &str) -> Result<Config, String> {
    if is_ident(s) {
        Ok(s.to_string())
    } else {
        Err(format!("error parsing config `{s}`"))
    }
}

fn is_ident(s: &str) -> bool {
    !s.is_empty()
        && !s
            .trim()
            .contains(|c: char| !(c.is_alphanumeric() || c == '_'))
}

fn extract<I: Iterator, const N: usize>(mut iter: I) -> Option<[I::Item; N]> {
    let a = std::array::try_from_fn(|_| iter.next())?;
    match iter.next() {
        Some(_) => None,
        None => Some(a),
    }
}
