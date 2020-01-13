use crate::math::Atom;

pub fn take_while1<V, X, T: Fn(String) -> Result<(X, V), String>>(
    s: String,
    predicate: T,
) -> Result<(String, Vec<V>), String> {
    let mut chars = s.chars().peekable();
    let mut results = vec![];
    loop {
        match chars.peek() {
            Some(c) => match predicate(c.to_string()) {
                Ok((_, res)) => {
                    results.push(res);
                    chars.next();
                }
                Err(x) => {
                    if results.len() == 0 {
                        return Err(x);
                    } else {
                        break;
                    }
                }
            },
            None => {
                if results.len() == 0 {
                    return Err(s);
                } else {
                    break;
                }
            }
        }
    }
    Ok((chars.collect(), results))
}
#[allow(dead_code)]
fn take_identifiers(s: String) -> Result<(String, Atom), String> {
    let mut identifier = String::new();
    take_alpha(s).and_then(|(remaining, c)| {
        identifier.push(c);
        take_while0(remaining, |x| take_alphanumeric(x))
            .and_then(|(remaining, vec)| {
                vec.iter().for_each(|c| identifier.push(*c));
                Ok((remaining, identifier))
            })
            .and_then(|(remaining, result)| {
                Ok((take_whitespaces0(remaining)?.0, Atom::Var(result)))
            })
    })
}

pub fn take_while0<V, X, T: Fn(String) -> Result<(X, V), String>>(
    s: String,
    predicate: T,
) -> Result<(String, Vec<V>), String> {
    take_while1(s.clone(), predicate).or_else(|_| Ok((s, vec![])))
}
pub fn take_alphanumeric(s: String) -> Result<(String, char), String> {
    let mut chars = s.chars();
    let first = chars.next();
    first
        .filter(|x| x.is_alphanumeric())
        .ok_or_else(|| s.clone())
        .and_then(|x| Ok((chars.collect::<String>(), x)))
}
pub fn take_alpha(s: String) -> Result<(String, char), String> {
    let mut chars = s.chars();
    let first = chars.next();
    first
        .filter(|x| x.is_alphabetic())
        .ok_or_else(|| s.clone())
        .and_then(|x| Ok((chars.collect::<String>(), x)))
}

fn take_ws(s: String) -> Result<(String, char), String> {
    let mut chars = s.chars();
    let next = chars.next();
    next.filter(|c| c.is_whitespace())
        .map(|c| (chars.collect::<String>(), c))
        .ok_or(s)
}
#[allow(dead_code)]
pub fn take_whitespaces1(s: String) -> Result<(String, ()), String> {
    take_while1(s, |x| take_ws(x)).and_then(|(remaining, _)| Ok((remaining, ())))
}
pub fn take_whitespaces0(s: String) -> Result<(String, ()), String> {
    take_while0(s, |x| take_ws(x)).and_then(|(remaining, _)| Ok((remaining, ())))
}

pub fn take_char(s: String, c: char) -> Result<(String, char), String> {
    let mut chars = s.chars();
    let first = chars.next();
    first
        .filter(|x| *x == c)
        .ok_or_else(|| s.clone())
        .and_then(|x| Ok((chars.collect::<String>(), x)))
}
pub fn take_str<'a>(s: String, s_to_match: &'a str) -> Result<(String, String), String> {
    let chars_to_match = s_to_match.clone().chars();
    let mut schars = s.chars();
    for i in chars_to_match {
        match schars.next() {
            Some(x) => {
                if i != x {
                    return Err(s);
                }
            }
            None => return Err(s),
        }
    }
    Ok((schars.collect(), s_to_match.to_string()))
}
#[allow(dead_code)]
pub fn take_not_char(s: String, c: char) -> Result<(String, char), String> {
    let mut chars = s.chars();
    let first = chars.next();
    first
        .filter(|x| *x != c)
        .ok_or_else(|| s.clone())
        .and_then(|x| Ok((chars.collect::<String>(), x)))
}
#[allow(dead_code)]
pub fn check_char(s: String, c: char) -> Result<(String, char), String> {
    let mut chars = s.chars();
    let first = match chars.next() {
        Some(x) => x,
        None => return Err(s),
    };
    if first == c {
        return Ok((chars.collect(), first));
    } else {
        Err(s)
    }
}
#[allow(dead_code)]
pub fn repeat0<V, T: FnMut(String) -> Result<(String, V), String>>(
    s: String,
    mut predicate: T,
) -> Result<(String, Vec<V>), String> {
    let mut remaining = s;
    let mut results = vec![];
    loop {
        match predicate(remaining.clone()) {
            Ok((rem, value)) => {
                remaining = rem;
                results.push(value);
            }
            Err(_) => return Ok((remaining, results)),
        }
    }
}
#[allow(dead_code)]
pub fn repeat0_with_state<K, V, T: Fn(String, &mut K) -> Result<(String, V), String>>(
    s: String,
    predicate: T,
    mut state: &mut K,
) -> Result<(String, Vec<V>), String> {
    let mut remaining = s;
    let mut results = vec![];
    loop {
        match predicate(remaining.clone(), &mut state) {
            Ok((rem, value)) => {
                remaining = rem;
                results.push(value);
            }
            Err(_) => return Ok((remaining, results)),
        }
    }
}
