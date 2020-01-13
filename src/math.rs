use crate::{
    common::{take_str, take_while1, take_whitespaces0},
    error::ParserError,
};
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
fn take_digit(s: String) -> Result<(String, char), ParserError> {
    let mut chars = s.chars();
    let first = chars.next();
    first
        .filter(|x| x.is_digit(10))
        .ok_or_else(|| ParserError::new(s.clone()))
        .and_then(|x| Ok((chars.collect::<String>(), x)))
}
pub fn take_numbers(s: String) -> Result<(String, Atom), ParserError> {
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
        .map_err(|_| ParserError::new(s))
}

pub fn take_operator(s: String) -> Result<(String, Atom), ParserError> {
    take_str(s.clone(), "+")
        .or_else(|error| take_str(error.remaining(), "*"))
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
            _ => Err(ParserError::newr(s, format!("Unknwon operator: {}", op))),
        })
}

pub fn into_postfix(tokens: Expr) -> Result<Expr, ParserError> {
    let mut op_stack: Expr = vec![];
    let mut output = vec![];
    for i in tokens {
        match i {
            Atom::Lit(lit) => output.push(Atom::Lit(lit)),
            Atom::Var(ident) => output.push(Atom::Var(ident)),
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
            _ => {
                return Err(ParserError::newr(
                    "".to_string(),
                    format!("Token {:#?} isn't allowed here", i),
                ))
            }
        }
    }
    for i in op_stack.into_iter().rev() {
        output.push(i);
    }
    Ok(output)
}
pub fn eval_postfix(vec: Expr, variables: &mut HashMap<String, i128>) -> Result<i128, ParserError> {
    let mut stack: Expr = vec![];
    for i in vec {
        match i {
            Atom::Op(op) => {
                let operand2 = if let Some(Atom::Lit(Literal::Num(num))) = stack.pop() {
                    num
                } else {
                    return Err(ParserError::newr(
                        "".to_string(),
                        format!("Expression wasn't valid"),
                    ));
                };
                let operand1 = if let Some(Atom::Lit(Literal::Num(num))) = stack.pop() {
                    num
                } else {
                    return Err(ParserError::newr(
                        "".to_string(),
                        format!("Expression wasn't valid"),
                    ));
                };
                match op.lexeme.as_str() {
                    "+" => stack.push(Atom::Lit(Literal::Num(operand1 + operand2))),
                    "*" => stack.push(Atom::Lit(Literal::Num(operand1 * operand2))),
                    _ => {
                        return Err(ParserError::newr(
                            "".to_string(),
                            format!("Unknwon operator: {:#?}", op),
                        ))
                    }
                }
            }
            Atom::Var(value) => match variables.get(&value) {
                Some(num) => stack.push(Atom::Lit(Literal::Num(*num as i32))),
                None => {
                    return Err(ParserError::newr(
                        "".to_string(),
                        format!("Undefined variable: {:#?}", value),
                    ))
                }
            },
            _ => stack.push(i),
        }
    }
    if stack.len() == 1 {
        match stack[0] {
            Atom::Lit(Literal::Num(num)) => Ok(num as i128),
            _ => {
                return Err(ParserError::newr(
                    "".to_string(),
                    format!("Syntax Error: Unknown"),
                ))
            }
        }
    } else {
        return Err(ParserError::newr(
            "".to_string(),
            format!("Syntax Error: Unknown"),
        ));
    }
}

mod test {
    use crate::eval_expr;
    use std::collections::HashMap;
    #[test]
    fn ops() {
        let mut var = HashMap::new();
        assert_eq!(
            eval_expr("1 + 2 * 3".to_string(), &mut var),
            Ok(("".to_string(), 1 + 2 * 3))
        );
        assert_eq!(
            eval_expr("(1 + 2) * 3".to_string(), &mut var),
            Ok(("".to_string(), (1 + 2) * 3))
        );
        assert_eq!(
            eval_expr("(12 + (2 * 3)) * ( 5 +(3* 8)) + 3".to_string(), &mut var),
            Ok(("".to_string(), (12 + (2 * 3)) * (5 + (3 * 8)) + 3))
        );
    }
}
