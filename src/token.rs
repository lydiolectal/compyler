#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    If,
    Print,
    Integer(u32),
    Newline,
    Indent,
    Dedent,
}
