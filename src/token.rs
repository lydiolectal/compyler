#[derive(Debug, PartialEq, Clone)]
pub enum Token {
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
