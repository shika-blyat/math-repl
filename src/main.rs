#![allow(unused_imports)]
mod common;
mod error;
mod math;

use common::{
    check_char, repeat0, repeat0_with_state, take_alpha, take_alphanumeric, take_char,
    take_identifier, take_not_char, take_str, take_while0, take_while1, take_whitespaces0,
    take_whitespaces1,
};
use error::ParserError;
use math::{eval_postfix, into_postfix, take_numbers, take_operator, Atom, Expr};
use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
};

pub fn take_atom(s: String, vec: &mut Expr) -> Result<(String, ()), ParserError> {
    let (remaining, _) = take_whitespaces0(s)
        .and_then(|(error, _)| take_numbers(error))
        .and_then(|(remaining, num)| {
            vec.push(num);
            take_whitespaces0(remaining)
        })
        .or_else(|error| {
            take_identifier(error.remaining()).and_then(|(remaining, ident)| {
                vec.push(Atom::Var(ident));
                take_whitespaces0(remaining)
            })
        })
        .or_else(|error| {
            take_char(error.remaining(), '(')
                .and_then(|(remaining, _)| take_whitespaces0(remaining))
                .and_then(|(remaining, _)| {
                    let (remaining, expr) = take_expr(remaining)?;
                    let (remaining, _) = take_whitespaces0(remaining)
                        .and_then(|(remaining, _)| take_char(remaining, ')'))
                        .and_then(|(remaining, _)| take_whitespaces0(remaining))?;
                    vec.push(Atom::Parens(expr));
                    Ok((remaining, ()))
                })
        })
        .or_else(|_| {
            Err(ParserError::newr(
                "".to_string(),
                format!("Expected either number or a parenthesized expression here"),
            ))
        })?;
    Ok((remaining, ()))
}
pub fn take_expr(s: String) -> Result<(String, Expr), ParserError> {
    let mut expr = vec![];
    let (remaining, _) = take_atom(s, &mut expr).and_then(|(remaining, _)| {
        repeat0(remaining, |remaining| {
            take_operator(remaining).and_then(|(remaining, op)| {
                expr.push(op);
                let (remaining, _) = take_whitespaces0(remaining)?;
                take_atom(remaining, &mut expr)
            })
        })
    })?;
    Ok((remaining, expr))
}
pub fn eval_expr(
    s: String,
    variables: &mut HashMap<String, i128>,
) -> Result<(String, i128), ParserError> {
    let mut rem = String::new();
    let result = take_expr(s)
        .and_then(|(remaining, expr)| {
            rem = remaining;
            into_postfix(expr)
        })
        .and_then(|expr| eval_postfix(expr, variables))?;
    Ok((rem, result))
}
pub fn take_decl(
    s: String,
    variables: &mut HashMap<String, i128>,
) -> Result<(String, i128), ParserError> {
    let mut ident = String::new();
    let (remaining, value) = take_str(s.clone(), "let")
        .and_then(|(remaining, _)| take_whitespaces1(remaining))
        .and_then(|(remaining, _)| take_identifier(remaining))
        .and_then(|(remaining, identifier)| {
            ident = identifier;
            take_whitespaces0(remaining)
        })
        .and_then(|(remaining, _)| take_char(remaining, '='))
        .and_then(|(remaining, _)| take_whitespaces0(remaining))
        .and_then(|(remaining, _)| eval_expr(remaining, variables))
        .and_then(|(remaining, value)| Ok((take_char(remaining, ';')?.0, value)))
        .or_else(|_| Err(ParserError::new(s)))?;
    variables.insert(ident, value);
    Ok((remaining, value))
}
pub fn eval_line(
    s: String,
    variables: &mut HashMap<String, i128>,
) -> Result<(String, i128), ParserError> {
    let result = take_decl(s, variables).or_else(|error| eval_expr(error.remaining(), variables));
    println!("{:#?}", variables);
    println!("{:#?}", result);
    result
}
fn main() {
    let mut variables = HashMap::new();
    loop {
        let mut input = String::new();
        print!(">>> ");
        stdout().flush().expect("Failed to write line");
        stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        if input == "quit" {
            break;
        }
        eval_line(input.to_string(), &mut variables);
    }
}

mod test {
    use crate::eval_line;
    use std::collections::HashMap;

    #[test]
    fn var_decl() {
        let mut variables = HashMap::new();
        assert_eq!(
            eval_line("let a = 15 * 2;".to_string(), &mut variables),
            Ok(("".to_string(), 30))
        );
        let mut to_compare = HashMap::new();
        to_compare.insert("a".to_string(), 30);
        assert_eq!(variables, to_compare);
        assert_eq!(
            eval_line("(a + 2) * 3".to_string(), &mut variables),
            Ok(("".to_string(), (30 + 2) * 3))
        );
    }
}
