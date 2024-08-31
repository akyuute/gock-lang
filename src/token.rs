use std::fmt;

//#[derive(Debug)]
pub enum TokenKind {

    // Multi-char tokens:
    /// Single-line comments
    /// //comment
    LineComment,

    /// Block comments
    /// /* comment */
    BlockComment,

    /// A sequence of whitespace characters
    Whitespace,

    /// An identity or keyword
    /// `foo` or `for`
    Ident,

    /// Like the above, but containing invalid Unicode codepoints.
    InvalidIdent,

    /// Literal types
    /// See `LiteralKind` below
    Literal{kind: LiteralKind},

    // Single-char tokens:
    /// "
    DoubleQuote,
    /// '
    SingleQuote,
    /// `
    Tick,
    /// (
    OpenPar,
    /// )
    ClosePar,
    /// [
    OpenSqb,
    /// ]
    CloseSqb,
    /// {
    OpenBrace,
    /// }
    CloseBrace,
    /// :
    Colon,
    /// ,
    Comma,
    /// ;
    Semi,
    /// +
    Plus,
    /// ++
    DoublePlus,
    /// -
    Minus,
    /// --
    DoubleMinus,
    /// *
    Star,
    /// **
    DoubleStar,
    /// /
    Slash,
    /// //
    DoubleSlash,
    /// %
    Percent,
    /// %%
    DoublePercent,
    /// !
    Bang,
    /// |
    Pipe,
    /// ||
    DoublePipe,
    /// &
    Amper,
    /// &&
    DoubleAmper,
    /// <
    Less,
    /// >
    Greater,
    /// =
    Equal,
    /// ->
    Arrow,
    /// .
    Dot,
    /// ==
    DoubleEqual,
    /// !=
    NotEqual,
    /// <=
    LessEqual,
    /// >=
    GreaterEqual,
    /// ~
    Tilde,
    /// ^
    Caret,
    /// <<
    LeftShift,
    /// >>
    RightShift,
    /// ?
    Question,
    /// $
    Dollar,
    /// @
    At,

    /// An unrecognized token
    Unknown,

    /// End of input
    Eof,
}

impl fmt::Debug for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match &self {

            TokenKind::DoubleQuote => "\"",
            TokenKind::SingleQuote => "\'",
            TokenKind::Tick => "`",
            TokenKind::OpenPar => "(",
            TokenKind::ClosePar => ")",
            TokenKind::OpenSqb => "[",
            TokenKind::CloseSqb => "]",
            TokenKind::OpenBrace => "{",
            TokenKind::CloseBrace => "}",
            TokenKind::Colon => ":",
            TokenKind::Comma => ",",
            TokenKind::Semi => ";",
            TokenKind::Plus => "+",
            TokenKind::Minus => "-",
            TokenKind::Star => "*",
            TokenKind::Slash => "/",
            TokenKind::Percent => "%",
            TokenKind::Bang => "!",
            TokenKind::Pipe => "|",
            TokenKind::Amper => "&",
            TokenKind::Less => "<",
            TokenKind::Greater => ">",
            TokenKind::Equal => "=",
            TokenKind::Dot => ".",
            TokenKind::Tilde => "~",
            TokenKind::Caret => "^",
            TokenKind::Question => "?",
            TokenKind::Dollar => "$",
            TokenKind::At => "@",
            _ => "Unkown",

        };

        write!(f, "\"{}\"", value)
    }
}

#[derive(Debug)]
pub enum LiteralKind {
    Int,
    Float,
    Bool,
    None,
    Str,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub len: u32,
}

impl Token {
}

