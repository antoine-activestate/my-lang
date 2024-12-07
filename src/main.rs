use std::str::Chars;

const LOWER_A: char = 'a';
const LOWER_Z: char = 'z';

const UPPER_A: char = 'A';
const UPPER_Z: char = 'Z';

const DIGIT_0: char = '0';
const DIGIT_9: char = '9';

const DIGIT_0_INT: u32 = 0x30;

const QUOTE: char = '"';
const MINUS: char = '-';
const UNDERSCORE: char = '_';

#[derive(Clone, Debug, PartialEq)]
enum Value {
    Nil,
    Bool(bool),
    Int(i64),
    Str(String),
}

fn main() {
    let cases = vec![
        // Idents
        " True",
        "\nFalse",
        "Nil",
        // Ints
        " 0",
        "\n123",
        "-0",
        "-123",
        // Strs
        " \"\"",
        "\n\"abc 123 éß😊\"",
    ];
    for case in cases {
        println!("{case}: {:?}", parse(&mut case.chars()));
    }
}

fn parse(input: &mut Chars<'_>) -> Vec<Value> {
    let (_, values) = parse_many(input);
    values
}

fn parse_many(input: &mut Chars<'_>) -> (Option<char>, Vec<Value>) {
    let (next, value) = parse_one(input);
    (next, vec![value])
}

fn parse_one(input: &mut Chars<'_>) -> (Option<char>, Value) {
    let first = match ignore_comments_whitespace(input) {
        None => panic!("parse: unexpected end of input"),
        Some(c) => c,
    };

    // Ident
    if is_alpha(first) {
        return parse_ident(input, first);
    }

    // Int
    if first == MINUS || is_num(first) {
        return parse_int(input, first);
    }

    // Str
    if first == QUOTE {
        return parse_str(input);
    }

    panic!("parse: unexpected char '{}'", first);
}

fn ignore_comments_whitespace(input: &mut Chars<'_>) -> Option<char> {
    loop {
        let next = input.next();
        match next {
            None => return None,
            Some(c) if c == ' ' => continue,
            Some(c) if c == '\n' => continue,
            _ => return next,
        }
    }
}

// Ident
fn parse_ident(input: &mut Chars<'_>, first: char) -> (Option<char>, Value) {
    let mut acc = vec![first];
    loop {
        let next = input.next();
        match next {
            Some(c) if is_ident_next(c) => {
                acc.push(c);
            }
            _ => {
                let ident_str = acc.into_iter().collect();
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

// Int
fn parse_int(input: &mut Chars<'_>, first: char) -> (Option<char>, Value) {
    let (neg, first) = if first == MINUS {
        let second = match input.next() {
            None => panic!("parse_int: unexpected end of input (expected digit after minus sign)"),
            Some(c) => c,
        };

        (true, second)
    } else {
        (false, first)
    };

    let mut acc: i64 = char_to_digit(first);
    loop {
        let next = input.next();
        match next {
            Some(c) if is_num(c) => {
                acc = acc * 10 + char_to_digit(c);
            }
            _ => {
                let signed = if neg { -acc } else { acc };
                return (next, Value::Int(signed));
            }
        }
    }
}

fn char_to_digit(c: char) -> i64 {
    i64::from(u32::from(c) - DIGIT_0_INT)
}

// Str
fn parse_str(input: &mut Chars<'_>) -> (Option<char>, Value) {
    let mut acc = vec![];
    loop {
        let next = input.next();
        match next {
            Some(c) if c != QUOTE => {
                acc.push(c);
            }
            _ => {
                let str = acc.into_iter().collect();
                return (None, Value::Str(str));
            }
        }
    }
}

// Utils
fn is_alpha(c: char) -> bool {
    LOWER_A <= c && c <= LOWER_Z || UPPER_A <= c && c <= UPPER_Z
}

fn is_num(c: char) -> bool {
    DIGIT_0 <= c && c <= DIGIT_9
}
