use super::*;

#[test]
fn test_debug() {
    let my_tok = Token {
        kind: TokenKind::Dollar,
        len: 1
    };
    assert_eq!(format!("{:?}", my_tok), "Token { kind: \"$\", len: 1 }");
}

#[test]
fn test_cursor_chars() {
    let my_cur = Cursor::new("Hello");
    assert_eq!(my_cur.second(), 'e');
}
