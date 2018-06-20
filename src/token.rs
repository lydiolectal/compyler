#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    If,    // 4 idk how to enforce that elif and else can only show up after 'if' block
    Elif,  // 4
    Else,  // 4
    Print,
    Def,    // 3
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
