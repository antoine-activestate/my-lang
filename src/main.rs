use std::str::Chars;

const LOWER_A: char = 'a';
const LOWER_Z: char = 'z';

const UPPER_A: char = 'A';
const UPPER_Z: char = 'Z';

const DIGIT_0: char = '0';
const DIGIT_9: char = '9';

const UNDERSCORE: char = '_';

fn main() {
    println!("{}", parse(&mut "True".chars()));
    println!("{}", parse(&mut "False".chars()));
    println!("{}", parse(&mut "Nil".chars()));
}

fn parse(input: &mut Chars<'_>) -> bool {
    let first = match input.next() {
        None => panic!("parse: unexpected end of input"),
        Some(c) => c,
    };

    // Ident
    if is_alpha(first) {
        let (_, ident) = parse_ident(input, first);
        println!("Ident: {ident}");
        return true;
    }

    panic!("[2] Unexpected char '{}'", first);
}

fn parse_ident(input: &mut Chars<'_>, first: char) -> (Option<char>, String) {
    let mut chars = vec![first];
    loop {
        let next = input.next();
        match next {
            Some(c) if is_ident_next(c) => {
                chars.push(c);
            }
            _ => {
                return (next, chars.into_iter().collect());
            }
        }
    }
}

fn is_ident_next(c: char) -> bool {
    is_alpha(c) || is_num(c) || c == UNDERSCORE
}

fn is_alpha(c: char) -> bool {
    LOWER_A <= c && c <= LOWER_Z || UPPER_A <= c && c <= UPPER_Z
}

fn is_num(c: char) -> bool {
    DIGIT_0 <= c && c <= DIGIT_9
}
