use std::str::Chars;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Expr {
    Unit,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ParseErr {
    UnexpectedChar(char, Option<char>),
    UnexpectedEOF(char),
}

enum ExprStart {
    LParen,
}

pub fn parse(str: &str) -> Result<Expr, ParseErr> {
    let mut chars = str.chars();
    let expr_res = _parse_expr(&mut chars);
    match chars.next() {
        Some(c) => Err(ParseErr::UnexpectedChar(c, None)),
        None => expr_res,
    }
}

fn _parse_expr(chars: &mut Chars<'_>) -> Result<Expr, ParseErr> {
    let start = match chars.next() {
        Some('(') => ExprStart::LParen,
        Some(c) => panic!("Unexpected character; expected start of expr: '{c}'"),
        None => panic!("Unexpected end of input; expected start of expr"),
    };

    match start {
        ExprStart::LParen => match chars.next() {
            Some(')') => Ok(Expr::Unit),
            Some(c) => Err(ParseErr::UnexpectedChar(c, Some(')'))),
            None => Err(ParseErr::UnexpectedEOF(')')),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Start
    #[test]
    #[should_panic(expected = "Unexpected end of input; expected start of expr")]
    fn test_parse_err_start_eof() {
        parse("");
    }

    #[test]
    #[should_panic(expected = "Unexpected character; expected start of expr: '?'")]
    fn test_parse_err_start_unrecognized() {
        parse("?");
    }

    #[test]
    fn test_parse_lparen() {
        assert_eq!(parse("("), Err(ParseErr::UnexpectedEOF(')')));
        assert_eq!(parse("(?"), Err(ParseErr::UnexpectedChar('?', Some(')'))));
    }

    #[test]
    fn test_parse_unit() {
        assert_eq!(parse("()"), Ok(Expr::Unit));
        assert_eq!(parse("()?"), Err(ParseErr::UnexpectedChar('?', None)));
    }
}
