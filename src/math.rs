use crate::common::{take_while1, take_whitespaces0};
use std::{collections::HashMap, num::ParseIntError};
#[derive(Debug, Clone, PartialEq)]
pub struct Operator {
    pub lexeme: String,
    pub precedence: i32,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Num(i32),
}
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Lit(Literal),
    Var(String),
    Operation(Vec<OpTerm>),
}
#[derive(Debug, Clone, PartialEq)]
pub enum OpTerm {
    Op(Operator),
    OpTerm(Expr),
}
fn take_digit(s: String) -> Result<(String, char), String> {
    let mut chars = s.chars();
    let first = chars.next();
    first
        .filter(|x| x.is_digit(10))
        .ok_or_else(|| s.clone())
        .and_then(|x| Ok((chars.collect::<String>(), x)))
}
pub fn take_numbers(s: String) -> Result<(String, Expr), String> {
    take_while1(s.clone(), |x| take_digit(x))
        .map(|(remaining, vec)| {
            (
                remaining,
                vec.into_iter().collect::<String>().parse().unwrap(),
            )
        })
        .and_then(|(remaining, num)| {
            Ok((
                take_whitespaces0(remaining)?.0,
                Expr::Lit(Literal::Num(num)),
            ))
        })
        .map_err(|_| s)
}

pub fn shunting_yard(tokens: Vec<OpTerm>, variables: &HashMap<String, i128>) -> Vec<String> {
    let mut output = vec![];
    let mut stack: Vec<Operator> = vec![];
    for token in tokens.into_iter().rev() {
        match token {
            OpTerm::OpTerm(expr) => match expr {
                Expr::Lit(lit) => match lit {
                    Literal::Num(num) => output.push(num.to_string()),
                },
                Expr::Var(name) => {
                    if let Some(x) = &variables.get(&name) {
                        output.push(x.to_string())
                    }
                }
                Expr::Operation(vec) => {
                    for i in shunting_yard(vec, &variables) {
                        output.push(i);
                    }
                }
            },
            OpTerm::Op(op) => {
                while let Some(x) = stack.last() {
                    if op.precedence <= x.precedence {
                        output.push(stack.pop().unwrap().lexeme)
                    }
                }
                stack.push(op);
            }
        }
    }
    for i in stack {
        output.push(i.lexeme)
    }
    output
}
fn apply_op(left: String, right: String, op: String) -> Result<i128, ParseIntError> {
    match op.as_str() {
        "*" => return Ok(left.parse::<i128>()? * right.parse::<i128>()?),
        "+" => return Ok(left.parse::<i128>()? + right.parse::<i128>()?),
        "-" => return Ok(left.parse::<i128>()? - right.parse::<i128>()?),
        "/" => return Ok(left.parse::<i128>()? / right.parse::<i128>()?),
        _ => todo!("No other operation than addition and multiplication is implemented yet"),
    }
}
pub fn eval_reverse_polish(tokens: Vec<String>) -> i128 {
    let mut op_stack = vec![];
    let mut last: Option<String> = None;
    for i in tokens.into_iter().rev() {
        if i.parse::<i128>().is_err() {
            op_stack.push(i);
        } else {
            if let None = last {
                last = Some(i);
            } else {
                last = Some(
                    apply_op(
                        last.unwrap(),
                        i,
                        op_stack.pop().expect("Operator stack was empty"),
                    )
                    .unwrap()
                    .to_string(),
                );
            }
        }
    }
    last.unwrap().parse().unwrap()
}
