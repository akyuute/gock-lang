use gock_lang::token::*;

fn main() {
    let my_tok = Token{
        kind: TokenKind::Dollar,
        len: 1
    };
    println!("{:?}", my_tok);
}
