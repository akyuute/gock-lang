pub mod token;
pub use crate::token::{Token, TokenKind};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug() {
        let my_tok = Token{
            kind: TokenKind::Dollar,
            len: 1
        };
        assert_eq!(format!("{:?}", my_tok), "Token { kind: \"$\", len: 1 }");

    }
}
