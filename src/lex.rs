use token::Token;
use error::Error;

pub struct Lexer {
    chars:   Vec<char>,
    current: Option<char>,
}

impl Lexer {
    pub fn new(text: &str) -> Lexer {
        // type of chars is dictated by the above
        // type annotation. since text has chars method,
        // which returns an iterator of characters, `chars`
        // variable is iterator of characters.
        //
        // when types flow this way, i think of them as
        // flowing from the inputs downstream
        let mut i = text.chars();

        let current = i.next();

        // all iterators define a .collect method, but the
        // type that .collect returns is flexible and
        // ambiguous. without further annotation, this
        // would be a type error, since collect doesn't
        // know what kind of collection to construct
        let chars = i.collect();

        Lexer {
            // here, however, the ambiguity is resolved!
            // we have declared that Lexer.chars is a vector
            // of characters, so even though the assignment
            // comes later in the program, it informs the type
            // of collection that .collect returns
            //
            // when types flow this way, i think of them
            // as flowing from the outputs to the inputs
            chars: chars,
            current: current,
        }
    }

    fn next(&mut self) -> Option<char> {
        if self.chars.is_empty() {
            self.current = None;
        } else {
            self.current = Some(self.chars.remove(0));
        }

        self.current
    }

    pub fn lex(mut self) -> Result<Vec<Token>, Error> {
        let mut tokens = Vec::new();

        while let Some(c) = self.current {
            match c {
                'a'...'z' => tokens.push(self.lex_identifier_or_keyword()),
                '0'...'9' => tokens.push(self.lex_integer()),
                '#' => self.lex_comment(),
                '\n' => tokens.push(self.lex_newline()),
                '\\' => self.lex_backslash()?,
                // desugars into:
                // '\\' => match self.lex_backslash() {
                //     Err(error) => return Err(error.into()),
                //     Ok(unit) => unit, // in this case, would be the unit value of the unit type ()
                // }
                _ => return Err(Error::UnexpectedStartOfToken(c)),
            }
        }

        Ok(tokens)
    }

    fn lex_identifier_or_keyword(&mut self) -> Token {
        let mut text = String::new();
        while let Some(c) = self.current {
            match c {
                'a'...'z' => text.push(c),
                _ => break,
            }
            self.next();
        }

        match text.as_str() {
            "if" => Token::If,
            _ => Token::Identifier(text),
        }
    }

    fn lex_integer(&mut self) -> Token {
        let mut integer = 0;
        while let Some(c) = self.current {
            match c {
                '0'...'9' => {
                    integer = integer * 10 + (c as u32 - '0' as u32);
                }
                _ => break,
            }
            self.next();
        }
        Token::Integer(integer)
    }

    fn lex_comment(&mut self) {
        while let Some(c) = self.current {
            if c == '\n' {
                break
            }
            self.next();
        }
    }

    fn lex_newline(&mut self) -> Token {
        self.next();
        Token::Newline
    }

    fn lex_backslash(&mut self) -> Result<(), Error> {
        if self.next() == Some('\n') {
            self.next();
            Ok(())
        } else {
            Err(Error::UnpairedBackslash(self.current))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_string() {
        let lexer  = Lexer::new("");
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens, vec![]);
    }

    #[test]
    fn illegal_char() {
        let lexer = Lexer::new("😎");
        let error = lexer.lex().unwrap_err();
        assert_eq!(error, Error::UnexpectedStartOfToken('😎'));
    }

    #[test]
    fn lowercase_identifier() {
        let lexer = Lexer::new("hi");
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens, vec![Token::Identifier("hi".to_owned())]);
    }

    #[test]
    fn if_keyword() {
        let lexer = Lexer::new("if");
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens, vec![Token::If]);
    }

    #[test]
    fn decimal_integer() {
        let lexer = Lexer::new("1234");
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens, vec![Token::Integer(1234)]);
    }

    #[test]
    fn comment() {
        let lexer = Lexer::new("#this is a comment");
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens, vec![]);
    }

    #[test]
    fn newline() {
        let lexer = Lexer::new("\n");
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens, vec![Token::Newline]);
    }

    #[test]
    fn escaped_newline() {
        let lexer = Lexer::new("\\\n");
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens, vec![]);
    }

    #[test]
    fn escaped_newline_fail() {
        let lexer = Lexer::new("\\h");
        let error = lexer.lex().unwrap_err();
        assert_eq!(error, Error::UnpairedBackslash(Some('h')));
    }

}
