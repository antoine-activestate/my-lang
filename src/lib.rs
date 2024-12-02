#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Expr {
    Unit,
}

pub fn parse(str: &str) -> Expr {
    let mut chars = str.chars();

    let first = match chars.next() {
        None => panic!("Unexpected end of input; expected expr"),
        Some(c) => c,
    };

    match first {
        '(' => match chars.next() {
            Some(')') => Expr::Unit,
            Some(c) => panic!("Unexpected character; expected ')': {}", c),
            None => panic!("Unexpected end of input; expected ')'"),
        },
        c => panic!("Unexpected character; expected expr: {}", c),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse("()"), Expr::Unit);
        assert_eq!(parse("() "), Expr::Unit);
    }
}
