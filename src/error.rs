use token::Token;

#[derive(Debug, PartialEq)]
pub enum Error {
    UnexpectedStartOfToken(char),
    UnpairedBackslash(Option<char>),
    UnmatchedIndentationLevel(u64),

    UnexpectedToken(Token),
}
