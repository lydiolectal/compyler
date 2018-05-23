use token::Token;
use error::Error;

pub struct Lexer {

}

impl Lexer {
    pub fn new(text: &str) -> Lexer {
        Lexer{}
    }

    pub fn lex(self) -> Result<Vec<Token>, Error> {
        Ok(Vec::new())
    }
}
