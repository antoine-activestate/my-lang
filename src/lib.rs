use std::str::Chars;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Expr {
    Unit,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ParseErr {
    UnexpectedChar(char, Expected),
    UnexpectedEOF(Expected),
}

fn unexpected_char<T>(c: char, expected: Expected) -> Result<T, ParseErr> {
    Err(ParseErr::UnexpectedChar(c, expected))
}

fn unexpected_eof<T>(expected: Expected) -> Result<T, ParseErr> {
    Err(ParseErr::UnexpectedEOF(expected))
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expected {
    EOF,
    Char(char),
    ExprStart,
}

enum ExprStart {
    LParen,
}

pub fn parse(str: &str) -> Result<Expr, ParseErr> {
    let mut chars = str.chars();
    let expr_res = _parse_expr(&mut chars);
    _parse_whitespace(&mut chars);

    match chars.next() {
        Some(c) => unexpected_char(c, Expected::EOF),
        None => expr_res,
    }
}

fn _parse_expr(chars: &mut Chars<'_>) -> Result<Expr, ParseErr> {
    _parse_whitespace(chars);

    let start = match chars.next() {
        Some('(') => ExprStart::LParen,
        Some(c) => return unexpected_char(c, Expected::ExprStart),
        None => return unexpected_eof(Expected::ExprStart),
    };

    match start {
        ExprStart::LParen => match chars.next() {
            Some(')') => Ok(Expr::Unit),
            Some(c) => unexpected_char(c, Expected::Char(')')),
            None => unexpected_eof(Expected::Char(')')),
        },
    }
}

fn _parse_whitespace(chars: &mut Chars<'_>) {
    loop {
        let prev_chars = chars.clone();
        match chars.next() {
            Some(' ') => continue,
            Some('\n') => continue,
            _ => {
                *chars = prev_chars;
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_start() {
        assert_eq!(parse(""), unexpected_eof(Expected::ExprStart));
        assert_eq!(parse("?"), unexpected_char('?', Expected::ExprStart));

        assert_eq!(parse(" "), unexpected_eof(Expected::ExprStart));
        assert_eq!(parse(" ?"), unexpected_char('?', Expected::ExprStart));
        assert_eq!(parse("\n"), unexpected_eof(Expected::ExprStart));
        assert_eq!(parse("\n?"), unexpected_char('?', Expected::ExprStart));
    }

    #[test]
    fn test_parse_lparen() {
        assert_eq!(parse("("), unexpected_eof(Expected::Char(')')));
        assert_eq!(parse("(?"), unexpected_char('?', Expected::Char(')')));

        // With whitespace
        assert_eq!(parse(" ("), unexpected_eof(Expected::Char(')')));
        assert_eq!(parse(" (?"), unexpected_char('?', Expected::Char(')')));
        assert_eq!(parse("\n("), unexpected_eof(Expected::Char(')')));
        assert_eq!(parse("\n(?"), unexpected_char('?', Expected::Char(')')));
    }

    #[test]
    fn test_parse_unit() {
        assert_eq!(parse("()"), Ok(Expr::Unit));
        assert_eq!(parse("()?"), unexpected_char('?', Expected::EOF));

        // With whitespace
        assert_eq!(parse(" ()"), Ok(Expr::Unit));
        assert_eq!(parse(" ()?"), unexpected_char('?', Expected::EOF));
        assert_eq!(parse("\n()"), Ok(Expr::Unit));
        assert_eq!(parse("\n()?"), unexpected_char('?', Expected::EOF));

        assert_eq!(parse("() "), Ok(Expr::Unit));
        assert_eq!(parse("() ?"), unexpected_char('?', Expected::EOF));
        assert_eq!(parse("()\n"), Ok(Expr::Unit));
        assert_eq!(parse("()\n?"), unexpected_char('?', Expected::EOF));
    }
}
