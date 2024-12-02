#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Expr {
    Unit,
}

enum ExprStart {
    LParen,
}

pub fn parse(str: &str) -> Expr {
    let mut chars = str.chars();

    let start = match chars.next() {
        Some('(') => ExprStart::LParen,
        Some(c) => panic!("Unexpected character; expected start of expr: {}", c),
        None => panic!("Unexpected end of input; expected start of expr"),
    };

    match start {
        ExprStart::LParen => match chars.next() {
            Some(')') => Expr::Unit,
            Some(c) => panic!("Unexpected character; expected ')': {}", c),
            None => panic!("Unexpected end of input; expected ')'"),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ok() {
        assert_eq!(parse("()"), Expr::Unit);
        assert_eq!(parse("() "), Expr::Unit);
    }
}
