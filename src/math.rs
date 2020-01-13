use crate::{
    common::{take_char, take_str, take_while0, take_while1, take_whitespaces0},
    error::ParserError,
};
use std::{
    collections::HashMap,
    num::ParseIntError,
    ops::{Add, Div, Mul, Sub},
};

pub type Expr = Vec<Atom>;

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    U32(u32),
    I32(i32),
    F32(f32),
}
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    U32,
    I32,
    F32,
}
impl Add for Number {
    type Output = f32;
    fn add(self, other: Self) -> f32 {
        match self {
            Number::F32(op1) => match other {
                Number::F32(op2) => op1 + op2,
                _ => unreachable!(),
            },
            Number::I32(op1) => match other {
                Number::I32(op2) => (op1 + op2) as f32,
                Number::U32(op2) => (op1 as u32 + op2) as f32,
                _ => unreachable!(),
            },
            Number::U32(op1) => match other {
                Number::I32(op2) => (op1 as i32 + op2) as f32,
                Number::U32(op2) => (op1 + op2 as u32) as f32,
                _ => unreachable!(),
            },
        }
    }
}
impl Sub for Number {
    type Output = f32;
    fn sub(self, other: Self) -> f32 {
        match self {
            Number::F32(op1) => match other {
                Number::F32(op2) => op1 - op2,
                _ => unreachable!(),
            },
            Number::I32(op1) => match other {
                Number::I32(op2) => (op1 - op2) as f32,
                Number::U32(op2) => (op1 as u32 - op2) as f32,
                _ => unreachable!(),
            },
            Number::U32(op1) => match other {
                Number::I32(op2) => (op1 as i32 - op2) as f32,
                Number::U32(op2) => (op1 - op2 as u32) as f32,
                _ => unreachable!(),
            },
        }
    }
}
impl Mul for Number {
    type Output = f32;
    fn mul(self, other: Self) -> f32 {
        match self {
            Number::F32(op1) => match other {
                Number::F32(op2) => op1 * op2,
                _ => unreachable!(),
            },
            Number::I32(op1) => match other {
                Number::I32(op2) => (op1 * op2) as f32,
                Number::U32(op2) => (op1 as u32 * op2) as f32,
                _ => unreachable!(),
            },
            Number::U32(op1) => match other {
                Number::I32(op2) => (op1 as i32 * op2) as f32,
                Number::U32(op2) => (op1 * op2 as u32) as f32,
                _ => unreachable!(),
            },
        }
    }
}
impl Div for Number {
    type Output = f32;
    fn div(self, other: Self) -> f32 {
        match self {
            Number::F32(op1) => match other {
                Number::F32(op2) => op1 * op2,
                _ => unreachable!(),
            },
            Number::I32(op1) => match other {
                Number::I32(op2) => (op1 * op2) as f32,
                Number::U32(op2) => (op1 as u32 * op2) as f32,
                _ => unreachable!(),
            },
            Number::U32(op1) => match other {
                Number::I32(op2) => (op1 as i32 * op2) as f32,
                Number::U32(op2) => (op1 * op2 as u32) as f32,
                _ => unreachable!(),
            },
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Num(Number),
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
pub fn take_float(s: String) -> Result<(String, Atom), ParserError> {
    let mut float = String::new();
    let (remaining, _) = take_while1(s.clone(), |x| take_digit(x))
        .and_then(|(remaining, int)| {
            float.push_str(int.into_iter().collect::<String>().as_str());
            take_char(remaining, '.')
        })
        .and_then(|(remaining, c)| {
            float.push(c);
            take_while1(remaining, |x| take_digit(x))
        })
        .and_then(|(remaining, decimals)| {
            float.push_str(decimals.into_iter().collect::<String>().as_str());
            Ok((remaining, ()))
        })
        .map_err(|_| ParserError::newr(s, format!("Invalid float literal")))?;
    Ok((
        remaining,
        Atom::Lit(Literal::Num(Number::F32(float.parse::<f32>().unwrap()))),
    ))
}
pub fn take_int(s: String) -> Result<(String, Atom), ParserError> {
    let (remaining, num) = take_while1(s.clone(), |x| take_digit(x))
        .map_err(|_| ParserError::newr(s, format!("Invalid int literal")))?;
    Ok((
        remaining,
        Atom::Lit(Literal::Num(Number::I32(
            num.into_iter().collect::<String>().parse().unwrap(),
        ))),
    ))
}
pub fn take_numbers(s: String) -> Result<(String, Atom), ParserError> {
    let mut number = Atom::Lit(Literal::Num(Number::I32(0)));
    take_float(s.clone())
        .or_else(|error| take_int(error.remaining()))
        .and_then(|(remaining, num)| {
            number = num;
            Ok((take_whitespaces0(remaining)?.0, ()))
        })
        .and_then(|(remaining, _)| Ok((remaining, number)))
        .map_err(|_| ParserError::new(s))
}

pub fn take_operator(s: String) -> Result<(String, Atom), ParserError> {
    take_str(s.clone(), "+")
        .or_else(|error| take_str(error.remaining(), "*"))
        .or_else(|error| take_str(error.remaining(), "/"))
        .or_else(|error| take_str(error.remaining(), "-"))
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
            "/" => Ok((
                remaining,
                Atom::Op(Operator {
                    lexeme: op,
                    precedence: 10,
                }),
            )),
            "-" => Ok((
                remaining,
                Atom::Op(Operator {
                    lexeme: op,
                    precedence: 5,
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
        }
    }
    for i in op_stack.into_iter().rev() {
        output.push(i);
    }
    Ok(output)
}
pub fn type_check_op(
    operand1: Number,
    operator: String,
    operand2: Number,
) -> Result<Type, ParserError> {
    match operand1 {
        Number::F32(num) => match operand2 {
            Number::F32(num) => Ok(Type::F32),
            _ => {
                return Err(ParserError::newr(
                    "".to_string(),
                    format!(
                        "Cannot apply operator {:#?} between {:#?} and {:#?}",
                        operator, operand1, operand2
                    ),
                ))
            }
        },
        _ => match operand2 {
            Number::F32(num) => {
                return Err(ParserError::newr(
                    "".to_string(),
                    format!(
                        "Cannot apply operator {:#?} between {:#?} and {:#?}",
                        operator, operand1, operand2
                    ),
                ))
            }
            Number::I32(_) => Ok(Type::I32),
            Number::U32(_) => Ok(Type::U32),
        },
    }
}
pub fn eval_postfix(vec: Expr, variables: &mut HashMap<String, f32>) -> Result<f32, ParserError> {
    let mut stack: Expr = vec![];
    for i in vec {
        match i {
            Atom::Op(op) => {
                let operand2 = match stack.pop() {
                    Some(Atom::Lit(Literal::Num(num))) => num,
                    _ => {
                        return Err(ParserError::newr(
                            "".to_string(),
                            format!("Expression wasn't valid"),
                        ))
                    }
                };
                let operand1 = match stack.pop() {
                    Some(Atom::Lit(Literal::Num(num))) => num,
                    _ => {
                        return Err(ParserError::newr(
                            "".to_string(),
                            format!("Expression wasn't valid"),
                        ))
                    }
                };
                let expr_type =
                    type_check_op(operand1.clone(), op.lexeme.clone(), operand2.clone())?;
                match op.lexeme.as_str() {
                    "+" => match expr_type {
                        Type::I32 => stack.push(Atom::Lit(Literal::Num(Number::I32(
                            (operand1 + operand2) as i32,
                        )))),
                        Type::F32 => {
                            stack.push(Atom::Lit(Literal::Num(Number::F32(operand1 + operand2))))
                        }
                        Type::U32 => stack.push(Atom::Lit(Literal::Num(Number::U32(
                            (operand1 + operand2) as u32,
                        )))),
                    },
                    "*" => match expr_type {
                        Type::I32 => stack.push(Atom::Lit(Literal::Num(Number::I32(
                            (operand1 * operand2) as i32,
                        )))),
                        Type::F32 => {
                            stack.push(Atom::Lit(Literal::Num(Number::F32(operand1 * operand2))))
                        }
                        Type::U32 => stack.push(Atom::Lit(Literal::Num(Number::U32(
                            (operand1 * operand2) as u32,
                        )))),
                    },
                    "/" => {
                        match expr_type {
                            Type::I32 => stack
                                .push(Atom::Lit(Literal::Num(Number::F32(operand1 / operand2)))),
                            Type::F32 => stack
                                .push(Atom::Lit(Literal::Num(Number::F32(operand1 / operand2)))),
                            Type::U32 => stack
                                .push(Atom::Lit(Literal::Num(Number::F32(operand1 / operand2)))),
                        }
                    }
                    "-" => match expr_type {
                        Type::I32 => stack.push(Atom::Lit(Literal::Num(Number::I32(
                            (operand1 - operand2) as i32,
                        )))),
                        Type::F32 => {
                            stack.push(Atom::Lit(Literal::Num(Number::F32(operand1 - operand2))))
                        }
                        Type::U32 => stack.push(Atom::Lit(Literal::Num(Number::U32(
                            (operand1 - operand2) as u32,
                        )))),
                    },
                    _ => {
                        return Err(ParserError::newr(
                            "".to_string(),
                            format!("Unknwon operator: {:#?}", op),
                        ))
                    }
                }
            }
            Atom::Var(value) => match variables.get(&value) {
                Some(num) => stack.push(Atom::Lit(Literal::Num(Number::F32(*num as f32)))),
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
        match stack[0].clone() {
            Atom::Lit(Literal::Num(num)) => match num {
                Number::I32(num) => Ok(num as f32),
                Number::F32(num) => Ok(num),
                Number::U32(num) => Ok(num as f32),
            },
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
            eval_expr("(12 + (2 - 3)) * ( 5 +(3 / 8)) + 3".to_string(), &mut var),
            Ok(("".to_string(), (12 + (2 - 3)) * (5 + (3 / 8)) + 3))
        );
    }
}
