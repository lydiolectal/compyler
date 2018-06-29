#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Identifier(String),
    If,
    Elif,
    Else,
    Print,
    Def,
    Return,
    Integer(u32),
    Newline,
    Indent,
    Dedent,
    Eof,
    ParenL,
    ParenR,
    Colon,
    EqEq,
    Plus,
    Minus,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
}
