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
pub fn take_not_char(s: String, c: char) -> Result<(String, char), String> {
    let mut chars = s.chars();
    let first = chars.next();
    first
        .filter(|x| *x != c)
        .ok_or_else(|| s.clone())
        .and_then(|x| Ok((chars.collect::<String>(), x)))
}

pub fn check_not_char(s: String, c: char) -> Result<(String, char), String> {
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

pub fn repeat0<V, T: Fn(String) -> Result<(String, V), String>>(
    s: String,
    predicate: T,
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
