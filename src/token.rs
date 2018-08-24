#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Identifier,
    If,
    Elif,
    Else,
    Print,
    Def,
    Return,
    Integer,
    Newline,
    Indent,
    Dedent,
    Eof,
    ParenL,
    ParenR,
    Colon,
    Comma,
    EqEq,
    Ne,
    Lt,
    Gt,
    Leq,
    Geq,
    Plus,
    Minus,
    Mult,
    Div,
    Mod,
    And,
    Or,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
}
