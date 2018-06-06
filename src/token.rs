#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    If,
    Integer(u32),
    Newline,
}
