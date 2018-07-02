use error::Error;
use token::{Token, TokenKind};

pub struct Lexer {
    chars: Vec<char>,
    current: Option<char>,
    column: u64,
    indent_stack: Vec<u64>,
    // false until we've encountered a nonblank character on a line.
    // |  asdfb
    //  ffftttt
    seen_nonblank: bool,
    cur_token: String,
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
        let indent_stack = vec![0];
        let cur_token = "".to_owned();

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
            cur_token: cur_token,
        }
    }

    fn make_token(&mut self, kind: TokenKind) -> Token {
        let token = Token {
            kind,
            lexeme: self.cur_token.clone(),
        };
        self.cur_token.clear();
        token
    }

    fn make_dent_token(&mut self, kind: TokenKind) -> Token {
        let mut lexeme = "".to_owned();
        match self.current {
            None => {}
            _ => {
                for _ in 0..self.column {
                    lexeme.push(' ');
                }
            }
        }
        Token { kind, lexeme }
    }

    fn next(&mut self) -> Option<char> {
        if let Some(c) = self.current {
            self.cur_token.push(c);
        }
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
            if !self.seen_nonblank && c != ' ' && c != '#' && c != '\n' {
                // unwrap_or works on option and result types: will return the Some/Ok but wrapped
                // value for err/none.
                // cloned() clones the inner value of option, because last() returns option(&u64).
                let cur_indentation = self.indent_stack.last().cloned().unwrap();
                if self.column == cur_indentation {
                } else if self.column > cur_indentation {
                    let t = self.make_dent_token(TokenKind::Indent);
                    tokens.push(t);
                    self.indent_stack.push(self.column);
                } else if self.column < cur_indentation {
                    let mut indentation_level = self.indent_stack.pop().unwrap();
                    while indentation_level > self.column {
                        indentation_level = self.indent_stack.pop().unwrap();
                        let t = self.make_dent_token(TokenKind::Dedent);
                        tokens.push(t);
                    }
                    self.indent_stack.push(indentation_level);
                    if indentation_level < self.column {
                        return Err(Error::UnmatchedIndentationLevel(self.column));
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
                '(' => {
                    self.next();
                    let t = self.make_token(TokenKind::ParenL);
                    tokens.push(t);
                }
                ')' => {
                    self.next();
                    let t = self.make_token(TokenKind::ParenR);
                    tokens.push(t);
                }
                '+' => {
                    self.next();
                    let t = self.make_token(TokenKind::Plus);
                    tokens.push(t);
                }
                '-' => {
                    self.next();
                    let t = self.make_token(TokenKind::Minus);
                    tokens.push(t);
                }
                ':' => {
                    self.next();
                    let t = self.make_token(TokenKind::Colon);
                    tokens.push(t);
                }
                '=' => tokens.push(self.lex_equals()?),
                _ => return Err(Error::UnexpectedStartOfToken(c)),
            }
        }

        // - count number of indents on indent_stack
        // - add same # of dedent tokens to token vector. :)
        for _ in 0..self.indent_stack.len() - 1 {
            let t = self.make_dent_token(TokenKind::Dedent);
            tokens.push(t);
        }
        let t = self.make_token(TokenKind::Eof);
        tokens.push(t);
        Ok(tokens)
    }

    fn lex_identifier_or_keyword(&mut self) -> Token {
        let mut text = String::new();
        while let Some(c) = self.current {
            match c {
                'a'...'z' | '0'...'9' => text.push(c),
                _ => break,
            }
            self.next();
        }

        match text.as_str() {
            "if" => self.make_token(TokenKind::If),
            "elif" => self.make_token(TokenKind::Elif),
            "else" => self.make_token(TokenKind::Else),
            "print" => self.make_token(TokenKind::Print),
            "def" => self.make_token(TokenKind::Def),
            "return" => self.make_token(TokenKind::Return),
            _ => self.make_token(TokenKind::Identifier),
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
        self.make_token(TokenKind::Integer)
    }

    fn lex_comment(&mut self) {
        while let Some(c) = self.current {
            if c == '\n' {
                break;
            }
            self.next();
        }
        self.cur_token.clear();
    }

    fn lex_newline(&mut self) -> Token {
        self.next();
        self.column = 0;
        self.seen_nonblank = false;
        self.make_token(TokenKind::Newline)
    }

    fn lex_backslash(&mut self) -> Result<(), Error> {
        if self.next() == Some('\n') {
            self.next();
            self.cur_token.clear();
            Ok(())
        } else {
            Err(Error::UnpairedBackslash(self.current))
        }
    }

    fn lex_whitespace(&mut self) {
        while self.next() == Some(' ') {}
        self.cur_token.clear();
    }

    fn lex_equals(&mut self) -> Result<Token, Error> {
        if self.next() == Some('=') {
            self.next();
            Ok(self.make_token(TokenKind::EqEq))
        } else {
            Err(Error::UnexpectedCharacter(self.current))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use error::Error::*;
    use token::{Token, TokenKind};

    macro_rules! token_test {
        (name: $name:ident,text: $text:expr,token: $expected:expr,) => {
            #[test]
            fn $name() {
                let text = $text;
                let mut expected = $expected.to_vec();
                let lexer = Lexer::new(text);
                let tokens = lexer.lex().unwrap();
                expected.push(Token {
                    kind: TokenKind::Eof,
                    lexeme: "".to_owned(),
                });
                assert_eq!(tokens, expected);
            }
        };
    }

    macro_rules! error_test {
        (name: $name:ident,text: $text:expr,error: $expected:expr,) => {
            #[test]
            fn $name() {
                let text = $text;
                let expected = $expected;
                let lexer = Lexer::new(text);
                let error = lexer.lex().unwrap_err();
                assert_eq!(error, expected);
            }
        };
    }

    token_test! {
        name: empty_string,
        text: "",
        token: [],
    }

    error_test! {
        name: illegal_char,
        text: "😎",
        error: Error::UnexpectedStartOfToken('😎'),
    }

    token_test! {
        name: lowercase_identifier,
        text: "hi",
        token: [Token{
            kind: TokenKind::Identifier,
            lexeme: "hi".to_owned(),
        }],
    }

    token_test! {
        name: if_keyword,
        text: "if",
        token: [Token{
            kind: TokenKind::If,
            lexeme: "if".to_owned(),
        }],
    }

    /*
    vec:     Vec::new(), vec![a,b,c]
    arrays:  [1,2,3] (size is part of the type)
    slices:  &[1,2,3]

    fn foo(a: [u8; 4]) {

}
    */

    token_test! {
        name: decimal_integer,
        text: "1234",
        token: [Token{
            kind: TokenKind::Integer,
            lexeme: "1234".to_owned(),
        }
            ],
    }

    token_test! {
        name: comment,
        text: "# this is a comment",
        token: [],
    }

    token_test! {
        name: newline,
        text: "\n",
        token: [Token{
            kind: TokenKind::Newline,
            lexeme: "\n".to_owned(),
        }],
    }

    token_test! {
        name: escaped_newline,
        text: "\\\n",
        token: [],
    }

    error_test! {
        name: escaped_newline_fail,
        text: "\\h",
        error: Error::UnpairedBackslash(Some('h')),
    }

    token_test! {
        name: blank_line,
        text: "  ",
        token: [],
    }

    token_test! {
        name: indent,
        text: "   39",
        token: [
            Token {
                kind: TokenKind::Indent,
                lexeme: "   ".to_owned(),
            }, Token {
                kind: TokenKind::Integer,
                lexeme: "39".to_owned(),
            }, Token {
                kind: TokenKind::Dedent,
                lexeme: "".to_owned(),
            }
        ],
    }

    token_test! {
        name: blank_line_comments,
        text: "   #this is a comment",
        token: [],
    }

    token_test! {
        name: dedent,
        text: "  39\nhmm",
        token: [Token {
            kind: TokenKind::Indent,
            lexeme: "  ".to_owned(),
        },
        Token {
            kind: TokenKind::Integer,
            lexeme: "39".to_owned(),
        },
        Token {
            kind: TokenKind::Newline,
            lexeme: "\n".to_owned(),
        },
        Token {
            kind: TokenKind::Dedent,
            lexeme: "".to_owned(),
        },
        Token {
            kind: TokenKind::Identifier,
            lexeme: "hmm".to_owned(),
        }],
    }

    token_test! {
        name: multiple_indent,
        text: "  39\n   hmm\n  1",
        token: [
            Token {
                kind: TokenKind::Indent,
                lexeme: "  ".to_owned(),
            },
            Token {
                kind: TokenKind::Integer,
                lexeme: "39".to_owned(),
            },
            Token {
                kind: TokenKind::Newline,
                lexeme: "\n".to_owned(),
            },
            Token {
                kind: TokenKind::Indent,
                lexeme: "   ".to_owned(),
            },
            Token {
                kind: TokenKind::Identifier,
                lexeme: "hmm".to_owned(),
            },
            Token {
                kind: TokenKind::Newline,
                lexeme: "\n".to_owned(),
            },
            Token {
                kind: TokenKind::Dedent,
                lexeme: "  ".to_owned(),
            },
            Token {
                kind: TokenKind::Integer,
                lexeme: "1".to_owned(),
            },
            Token {
                kind: TokenKind::Dedent,
                lexeme: "".to_owned(),
            }
        ],
    }

    error_test! {
        name: illegal_indent,
        text: "  39\n   hmm\n 1",
        error: Error::UnmatchedIndentationLevel(1),
    }

    token_test! {
        name: print,
        text: "print",
        token: [
            Token {
                kind: TokenKind::Print,
                lexeme: "print".to_owned(),
            }
        ],
    }

    token_test! {
        name: def,
        text: "def",
        token: [
            Token {
                kind: TokenKind::Def,
                lexeme: "def".to_owned(),
            }
        ],
    }

    token_test! {
        name: elif,
        text: "elif",
        token: [
            Token {
                kind: TokenKind::Elif,
                lexeme: "elif".to_owned(),
            }
        ],
    }

    token_test! {
        name: else_token,
        text: "else",
        token: [
            Token {
                kind: TokenKind::Else,
                lexeme: "else".to_owned(),
            }
        ],
    }

    token_test! {
        name: parenl,
        text: "(",
        token: [
            Token {
                kind: TokenKind::ParenL,
                lexeme: "(".to_owned(),
            }
        ],
    }

    token_test! {
        name: parenr,
        text: ")",
        token: [
            Token {
                kind: TokenKind::ParenR,
                lexeme: ")".to_owned(),
            }
        ],
    }

    token_test! {
        name: return_token,
        text: "return",
        token: [
            Token {
                kind: TokenKind::Return,
                lexeme: "return".to_owned(),
            }
        ],
    }

    token_test! {
        name: colon,
        text: ":",
        token: [
            Token {
                kind: TokenKind::Colon,
                lexeme: ":".to_owned(),
            }
        ],
    }

    token_test! {
        name:  minus,
        text:  "-",
        token: [
            Token {
                kind: TokenKind::Minus,
                lexeme: "-".to_owned(),
            }
        ],
    }

    token_test! {
        name:  plus,
        text:  "+",
        token: [
            Token {
                kind: TokenKind::Plus,
                lexeme: "+".to_owned(),
            }
        ],
    }

    token_test! {
        name: eqeq,
        text: "==",
        token: [
            Token {
                kind: TokenKind::EqEq,
                lexeme: "==".to_owned(),
            }
        ],
    }

    error_test! {
        name:  eqeq_error,
        text:  "=",
        error: UnexpectedCharacter(None),
    }
}
