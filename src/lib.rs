pub mod token;
pub use crate::token::{Token, TokenKind};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structs() {
        let my_tok = Token{
            kind: TokenKind::Dollar,
            len: 1
        };
        println!("{:?}", my_tok);

    }
}
