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
    match input.next() {
        None => panic!("[1] Unexpected end of input"),
        Some(c) => {
            if is_alpha(c) {
                let ident = parse_ident(input, c);
                println!("Ident: {ident}");
                return true;
            }

            panic!("[2] Unexpected char '{}'", c);
        }
    }
}

fn parse_ident(input: &mut Chars<'_>, first: char) -> String {
    let mut chars = vec![first];

    loop {
        let prev_input = input.clone();

        match input.next() {
            None => {
                *input = prev_input;
                return chars.into_iter().collect();
            }
            Some(c) => {
                if !is_ident_next(c) {
                    *input = prev_input;
                    return chars.into_iter().collect();
                }
                chars.push(c)
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
