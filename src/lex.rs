use token::Token;
use error::Error;

pub struct Lexer {
    chars:   Vec<char>,
    current: Option<char>,
    column: u64,
    indent_stack: Vec<u64>,
    // false until we've encountered a nonblank character on a line.
    // |  asdfb
    //  ffftttt
    seen_nonblank: bool,
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
        let mut indent_stack = vec![0];

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
            column: 0,
            // indent_stack
            // - push current level of indentation onto stack whenever I add indent token.
            // - ex: [2, 4, 7, 9] means that indent, respectively, were: [2, 2, 3, 2].
            indent_stack: indent_stack,
            seen_nonblank: false,
        }
    }

    fn next(&mut self) -> Option<char> {
        if self.current != Some(' ') {
            self.seen_nonblank = true;
        }
        if self.chars.is_empty() {
            self.current = None;
        } else {
            self.current = Some(self.chars.remove(0));
        }
        self.column += 1;
        self.current
    }

    pub fn lex(mut self) -> Result<Vec<Token>, Error> {
        let mut tokens = Vec::new();

        while let Some(c) = self.current {
            // - skip characters until reach non-blank. (self.column tracks # traversed.)
            // - if comment or newline, don't do anything; let match handle them.
            if !self.seen_nonblank && c != ' ' && c != '#' && c != '\n' {
                // check self.column against indent stack;
                let cur_indentation = match self.indent_stack.pop() {
                    Some(cur_indentation) => cur_indentation,
                    None => 0,
                };
                match self.column {
                    col if col == cur_indentation => {
                        // if == indent_stack.pop,
                        // no new indent or dedent needed.
                        self.indent_stack.push(cur_indentation);
                    }
                    col if col > cur_indentation => {
                        // elif self.column > indent_stack.pop, add indent token. Push current
                        // level of indentation to indent_stack.
                        tokens.push(Token::Indent);
                        self.indent_stack.push(cur_indentation);
                        self.indent_stack.push(col);
                    }
                    _ => {
                        let mut dedents = 0;
                        let indentation_level = match self.indent_stack.pop() {
                            Some(indentation_level) => indentation_level,
                            None => 0,
                        };
                        // else self.column < indent_stack.pop, keep popping off indent stack until column
                        // and thing popped are equal OR thing popped is less than self.column.
                        while indentation_level > self.column {
                            dedents += 1;
                            let indentation_level = match self.indent_stack.pop() {
                                Some(indentation_level) => indentation_level,
                                None => 0,
                            };
                        }
                        if indentation_level == self.column {
                            // issue # of dedents eq to # items popped.
                            for _ in 0..dedents {
                                tokens.push(Token::Dedent);
                            }
                            self.indent_stack.push(indentation_level);
                        }
                        else if indentation_level < self.column {
                            return Err(Error::UnmatchedIndentationLevel(self.column));
                        }
                    }
                }
            }

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
                // ' ' if self.column == 0 => tokens.push(self.lex_indent()),
                ' ' => self.lex_whitespace(),
                _ => return Err(Error::UnexpectedStartOfToken(c)),
            }
        }

        // - count number of indents on indent_stack
        // - add same # of dedent tokens to token vector. :)
        for _ in 0..self.indent_stack.len()-1 {
            tokens.push(Token::Dedent);
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
        self.column = 0;
        self.seen_nonblank = false;
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

    fn lex_indent(&mut self) -> Token {
        self.next();
        Token::Indent
    }

    fn lex_whitespace(&mut self) {
        while self.next() == Some(' ') {
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use token::Token::*;

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
        assert_eq!(tokens, vec![Identifier("hi".to_owned())]);
    }

    #[test]
    fn if_keyword() {
        let lexer = Lexer::new("if");
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens, vec![If]);
    }

    #[test]
    fn decimal_integer() {
        let lexer = Lexer::new("1234");
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens, vec![Integer(1234)]);
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
        assert_eq!(tokens, vec![Newline]);
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

    #[test]
    fn blank_line() {
        let lexer = Lexer::new("  ");
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens, vec![]);
    }

    #[test]
    fn indent() {
        let lexer = Lexer::new("  39");
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens, vec![Indent, Integer(39), Dedent]);
    }

    #[test]
    fn blank_line_comments() {
        let lexer = Lexer::new("  #this is a comment");
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens, vec![]);
    }

    #[test]
    fn dedent() {
        let lexer = Lexer::new("  39\nhmm");
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens, vec![Indent, Integer(39), Newline, Dedent, Identifier("hmm".into())]);
    }
}
