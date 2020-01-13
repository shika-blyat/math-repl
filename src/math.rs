use crate::common::{take_str, take_while1, take_whitespaces0};
use std::{collections::HashMap, num::ParseIntError};

pub type Expr = Vec<Atom>;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Num(i32),
}
#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Lit(Literal),
    Var(String),
    Op(Operator),
    Parens(Expr),
}
#[derive(Debug, Clone, PartialEq)]
pub struct Operator {
    lexeme: String,
    precedence: i32,
}
fn take_digit(s: String) -> Result<(String, char), String> {
    let mut chars = s.chars();
    let first = chars.next();
    first
        .filter(|x| x.is_digit(10))
        .ok_or_else(|| s.clone())
        .and_then(|x| Ok((chars.collect::<String>(), x)))
}
pub fn take_numbers(s: String) -> Result<(String, Atom), String> {
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
                Atom::Lit(Literal::Num(num)),
            ))
        })
        .map_err(|_| s)
}

pub fn take_operator(s: String) -> Result<(String, Atom), String> {
    take_str(s.clone(), "+")
        .or_else(|remaining| take_str(remaining, "*"))
        .and_then(|(remaining, op)| match op.as_str() {
            "+" => Ok((
                remaining,
                Atom::Op(Operator {
                    lexeme: op,
                    precedence: 5,
                }),
            )),
            "*" => Ok((
                remaining,
                Atom::Op(Operator {
                    lexeme: op,
                    precedence: 10,
                }),
            )),
            _ => Err(s),
        })
}

pub fn into_postfix(tokens: Expr) -> Result<Expr, String> {
    let mut op_stack: Expr = vec![];
    let mut output = vec![];
    for i in tokens {
        match i {
            Atom::Lit(lit) => output.push(Atom::Lit(lit)),
            Atom::Op(op) => {
                while op_stack.last().is_some() {
                    let last = op_stack.last().unwrap();
                    if let Atom::Op(last_op) = last {
                        if last_op.precedence >= op.precedence {
                            output.push(op_stack.pop().unwrap());
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                op_stack.push(Atom::Op(op));
            }
            Atom::Parens(expr) => into_postfix(expr)?.into_iter().for_each(|x| output.push(x)),
            _ => unimplemented!(),
        }
    }
    for i in op_stack.into_iter().rev() {
        output.push(i);
    }
    Ok(output)
}
pub fn eval_postfix(vec: Expr) -> Result<i128, String> {
    let mut stack: Expr = vec![];
    for i in vec {
        match i {
            Atom::Op(op) => {
                let operand2 = if let Atom::Lit(Literal::Num(num)) = stack.pop().unwrap() {
                    num
                } else {
                    panic!("Unexpected {:#?}", op);
                };
                let operand1 = if let Atom::Lit(Literal::Num(num)) = stack.pop().unwrap() {
                    num
                } else {
                    panic!("Unexpected {:#?}", op);
                };
                match op.lexeme.as_str() {
                    "+" => stack.push(Atom::Lit(Literal::Num(operand1 + operand2))),
                    "*" => stack.push(Atom::Lit(Literal::Num(operand1 * operand2))),
                    _ => unimplemented!(),
                }
            }
            _ => stack.push(i),
        }
    }
    if stack.len() == 1 {
        match stack[0] {
            Atom::Lit(Literal::Num(num)) => Ok(num as i128),
            _ => panic!("Evaluation stack didn't contained a number"),
        }
    } else {
        panic!("Evaluation stack wasn't empty")
    }
}

mod test {
    use crate::eval_expr;
    #[test]
    fn addition() {
        assert_eq!(
            eval_expr("1 + 2 * 3".to_string()),
            Ok(("".to_string(), 1 + 2 * 3))
        );
        assert_eq!(
            eval_expr("(1 + 2) * 3".to_string()),
            Ok(("".to_string(), (1 + 2) * 3))
        );
        assert_eq!(
            eval_expr("(12 + (2 * 3)) * ( 5 +(3* 8)) + 3".to_string()),
            Ok(("".to_string(), (12 + (2 * 3)) * (5 + (3 * 8)) + 3))
        );
    }
}
