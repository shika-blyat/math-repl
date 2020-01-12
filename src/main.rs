use common::{
    check_not_char, repeat0, repeat0_with_state, take_alpha, take_alphanumeric, take_char,
    take_not_char, take_while0, take_while1, take_whitespaces0, take_whitespaces1,
};
use math::{eval_reverse_polish, shunting_yard, take_numbers, Expr, OpTerm, Operator};
use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
};
mod common;
mod math;

fn take_identifiers(s: String) -> Result<(String, Expr), String> {
    let mut identifier = String::new();
    take_alpha(s).and_then(|(remaining, c)| {
        identifier.push(c);
        take_while0(remaining, |x| take_alphanumeric(x))
            .and_then(|(remaining, vec)| {
                vec.iter().for_each(|c| identifier.push(*c));
                Ok((remaining, identifier))
            })
            .and_then(|(remaining, result)| {
                Ok((take_whitespaces0(remaining)?.0, Expr::Var(result)))
            })
    })
}

pub fn take_operator(s: String) -> Result<(String, Operator), String> {
    take_char(s.clone(), '+')
        .or_else(|x| take_char(x, '*'))
        .or_else(|x| take_char(x, '/'))
        .or_else(|x| take_char(x, '-'))
        .or_else(|x| {
            let new_x = x.clone();
            let mut chars = new_x.chars();
            let op = match chars.next() {
                Some(x) => x,
                None => return Err(x),
            };
            Ok((chars.collect(), op))
        })
        .and_then(|(remaining, op)| {
            let op = match op {
                '*' => Operator {
                    lexeme: "*".to_string(),
                    precedence: 10,
                },
                '+' => Operator {
                    lexeme: "+".to_string(),
                    precedence: 5,
                },
                '-' => Operator {
                    lexeme: "-".to_string(),
                    precedence: 5,
                },
                '/' => Operator {
                    lexeme: "/".to_string(),
                    precedence: 10,
                },
                _ => return Err(s),
            };
            Ok((remaining, op))
        })
        .or_else(|remaining| {
            let (remaining, _) = check_not_char(remaining, ' ')?;
            Ok((
                remaining,
                Operator {
                    lexeme: "".to_string(),
                    precedence: 0,
                },
            ))
        })
}
pub fn take_parenthesized(s: String) -> Result<(String, Expr), String> {
    let (remaining, _) = take_whitespaces0(s.clone())?;
    let (remaining, expr) = match take_char(remaining, '(')
        .and_then(|(remaining, _)| take_whitespaces0(remaining))
        .and_then(|(remaining, _)| take_ops(remaining))
    {
        Ok(x) => x,
        Err(_) => return Err(s),
    };
    let (remaining, _) =
        take_whitespaces0(remaining).and_then(|(remaining, _)| take_char(remaining, ')'))?;
    Ok((remaining, expr))
}
pub fn take_ops(s: String) -> Result<(String, Expr), String> {
    let (remaining, _) = take_whitespaces0(s.clone())?;
    let mut op_terms: Vec<OpTerm> = vec![];
    take_numbers(remaining)
        .or_else(take_identifiers)
        .or_else(take_parenthesized)
        .and_then(|(remaining, left)| {
            op_terms.push(OpTerm::OpTerm(left));
            let (remaining, _) = take_whitespaces0(remaining)?;
            Ok((remaining, ()))
        })
        .and_then(|(remaining, _)| {
            repeat0(remaining, |x| {
                let (remaining, _) = take_whitespaces0(x.clone())?;
                let mut operator = OpTerm::Op(Operator {
                    lexeme: "".to_string(),
                    precedence: 0,
                });
                match take_operator(remaining).and_then(|(remaining, op)| {
                    operator = OpTerm::Op(op);
                    let (remaining, _) = take_whitespaces0(remaining)?;
                    let (remaining, expr) = take_numbers(remaining)
                        .or_else(|remaining| take_identifiers(remaining))
                        .or_else(|remaining| take_parenthesized(remaining))?;
                    let (remaining, _) = take_whitespaces0(remaining)?;
                    Ok((remaining, expr))
                }) {
                    Ok((remaining, expr)) => Ok((remaining, (operator, OpTerm::OpTerm(expr)))),
                    Err(_) => return Err(x),
                }
            })
        })
        .and_then(|(remaining, vec_opterm)| {
            for (op, expr) in vec_opterm {
                if let OpTerm::Op(operator) = op.clone() {
                    if !(operator.lexeme == "" && operator.precedence == 0) {
                        op_terms.push(op);
                    }
                } else {
                    op_terms.push(op);
                }
                op_terms.push(expr)
            }
            let (remaining, _) = take_whitespaces0(remaining).and_then(|(remaining, _)| {
                take_char(remaining, ';').or_else(|remaining| Ok((remaining, '\0')))
            })?;
            Ok((remaining, Expr::Operation(op_terms)))
        })
        .or(Err(s))
}
pub fn take_decl(s: String) -> Result<(String, (Expr, Expr)), String> {
    let (remaining, _) = take_whitespaces0(s.clone())?;
    if remaining.starts_with("let") {
        let remaining = remaining[3..].to_string();
        let mut ident = Expr::Var(String::new());
        let mut tokens = vec![];
        let (remaining, _) = take_whitespaces1(remaining)
            .and_then(|(remaining, _)| {
                take_identifiers(remaining).and_then(|(remaining, identifier)| {
                    ident = identifier;
                    take_whitespaces0(remaining)
                })
            })
            .and_then(|(remaining, _)| take_char(remaining, '='))
            .and_then(|(remaining, _)| take_whitespaces0(remaining))
            .and_then(|(remaining, _)| {
                let (remaining, expr) = take_while1(remaining, |x| take_not_char(x, ';'))?;
                tokens = expr;
                Ok((remaining, ()))
            })
            .and_then(|(remaining, _)| take_char(remaining, ';'))?;
        let str_expr = tokens.iter().collect::<String>();
        let (_, expr) = take_ops(str_expr)?;
        Ok((remaining, (ident, expr)))
    } else {
        Err(s)
    }
}

fn parse_expr(s: String, variables: &mut HashMap<String, i128>) -> Result<(String, ()), String> {
    (take_decl(s.to_string()).and_then(|(remaining, (var_name, value))| {
        let value = match value {
            Expr::Operation(vec) => eval_reverse_polish(shunting_yard(vec, &variables)),
            _ => unreachable!(),
        };
        let var_name = match var_name {
            Expr::Var(name) => name,
            _ => unreachable!(),
        };
        println!("assigned {} to {}", value, var_name);
        variables.insert(var_name, value);
        Ok((remaining, ()))
    }))
    .or_else(|remaining| {
        let (remaining, ops) = match take_ops(remaining.clone()) {
            Ok(x) => match x {
                (remaining, Expr::Operation(ops)) => (remaining, ops),
                _ => unreachable!(),
            },
            Err(_) => return Err(remaining),
        };
        let val = eval_reverse_polish(shunting_yard(ops, &variables));
        println!("{:#?}", val);
        Ok((remaining, ()))
    })
}
fn parse_program(
    s: String,
    mut variables: HashMap<String, i128>,
) -> Result<(String, HashMap<String, i128>), String> {
    let (remaining, _) = repeat0_with_state(s, |x, state| parse_expr(x, state), &mut variables)?;
    Ok((remaining, variables))
}

fn main() {
    let mut variables = HashMap::new();
    loop {
        let mut input = String::new();
        print!(">>> ");
        stdout().flush().expect("Failed to write line");
        stdin().read_line(&mut input).expect("Failed to read line");
        if input.as_str().trim() == "quit" {
            break;
        }
        variables = parse_program(input, variables).unwrap().1;
    }
}
