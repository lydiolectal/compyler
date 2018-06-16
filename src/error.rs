use token::Token;

#[derive(Debug, PartialEq)]
pub enum Error {
    UnexpectedStartOfToken(char),
    UnexpectedCharacter(Option<char>),
    UnpairedBackslash(Option<char>),
    UnmatchedIndentationLevel(u64),

    UnexpectedToken(Token),
}
