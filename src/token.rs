#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    If,
    Print,
    Def,
    Integer(u32),
    Newline,
    Indent,
    Dedent,
    Eof,
}
