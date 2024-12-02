use std::str::Chars;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Expr {
    Unit,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ParseErr {
    Raw(String),
}

enum ExprStart {
    LParen,
}

pub fn parse(str: &str) -> Result<Expr, ParseErr> {
    let mut chars = str.chars();
    let expr = _parse_expr(&mut chars);
    match chars.next() {
        Some(c) => panic!("Unexpected character; expected end of input: '{c}'"),
        None => {}
    }
    return Ok(expr);
}

fn _parse_expr(chars: &mut Chars<'_>) -> Expr {
    let start = match chars.next() {
        Some('(') => ExprStart::LParen,
        Some(c) => panic!("Unexpected character; expected start of expr: '{c}'"),
        None => panic!("Unexpected end of input; expected start of expr"),
    };

    match start {
        ExprStart::LParen => match chars.next() {
            Some(')') => Expr::Unit,
            Some(c) => panic!("Unexpected character; expected ')': '{c}'"),
            None => panic!("Unexpected end of input; expected ')'"),
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

    // LParen
    #[test]
    #[should_panic(expected = "Unexpected end of input; expected ')'")]
    fn test_parse_lparen_err_eof() {
        parse("(");
    }

    #[test]
    #[should_panic(expected = "Unexpected character; expected ')': '?'")]
    fn test_parse_lparen_err_unrecognized() {
        parse("(?");
    }

    // Unit
    #[test]
    fn test_parse_unit_ok() {
        assert_eq!(parse("()"), Ok(Expr::Unit));
    }

    #[test]
    #[should_panic(expected = "Unexpected character; expected end of input: '?'")]
    fn test_parse_unit_err_end_not_eof() {
        parse("()?");
    }
}
