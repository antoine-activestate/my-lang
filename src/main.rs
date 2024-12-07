use std::str::Chars;

const LOWER_A: char = 'a';
const LOWER_Z: char = 'z';

const UPPER_A: char = 'A';
const UPPER_Z: char = 'Z';

const DIGIT_0: char = '0';
const DIGIT_9: char = '9';

const UNDERSCORE: char = '_';

#[derive(Clone, Copy, Debug, PartialEq)]
enum Value {
    Nil,
    Bool(bool),
}

fn main() {
    let cases = vec!["True", "False", "Nil"];
    for case in cases {
        println!("{case}: {:?}", parse(&mut case.chars()));
    }
}

fn parse(input: &mut Chars<'_>) -> (Option<char>, Value) {
    let first = match input.next() {
        None => panic!("parse: unexpected end of input"),
        Some(c) => c,
    };

    // Ident
    if is_alpha(first) {
        return parse_ident(input, first);
    }

    panic!("[2] Unexpected char '{}'", first);
}

// Ident
fn parse_ident(input: &mut Chars<'_>, first: char) -> (Option<char>, Value) {
    let mut chars = vec![first];
    loop {
        let next = input.next();
        match next {
            Some(c) if is_ident_next(c) => {
                chars.push(c);
            }
            _ => {
                let ident_str = chars.into_iter().collect();
                return (next, resolve_ident(ident_str));
            }
        }
    }
}

fn is_ident_next(c: char) -> bool {
    is_alpha(c) || is_num(c) || c == UNDERSCORE
}

fn resolve_ident(ident: String) -> Value {
    match ident.as_str() {
        "Nil" => Value::Nil,
        "False" => Value::Bool(false),
        "True" => Value::Bool(true),
        _ => panic!("resolve_ident: unknown identifier '{ident}'"),
    }
}

// Utils
fn is_alpha(c: char) -> bool {
    LOWER_A <= c && c <= LOWER_Z || UPPER_A <= c && c <= UPPER_Z
}

fn is_num(c: char) -> bool {
    DIGIT_0 <= c && c <= DIGIT_9
}
