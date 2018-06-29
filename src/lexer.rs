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
            if !self.seen_nonblank && c != ' ' && c != '#' && c != '\n' {
                // unwrap_or works on option and result types: will return the Some/Ok but wrapped
                // value for err/none.
                // cloned() clones the inner value of option, because last() returns option(&u64).
                let cur_indentation = self.indent_stack.last().cloned().unwrap();
                if self.column == cur_indentation {
                } else if self.column > cur_indentation {
                    tokens.push(Token {
                        kind: TokenKind::Indent,
                    });
                    self.indent_stack.push(self.column);
                } else if self.column < cur_indentation {
                    let mut indentation_level = self.indent_stack.pop().unwrap();
                    while indentation_level > self.column {
                        indentation_level = self.indent_stack.pop().unwrap();
                        tokens.push(Token {
                            kind: TokenKind::Dedent,
                        });
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
                    tokens.push(Token {
                        kind: TokenKind::ParenL,
                    });
                    self.next();
                }
                ')' => {
                    tokens.push(Token {
                        kind: TokenKind::ParenR,
                    });
                    self.next();
                }
                '+' => {
                    tokens.push(Token {
                        kind: TokenKind::Plus,
                    });
                    self.next();
                }
                '-' => {
                    tokens.push(Token {
                        kind: TokenKind::Minus,
                    });
                    self.next();
                }
                ':' => {
                    tokens.push(Token {
                        kind: TokenKind::Colon,
                    });
                    self.next();
                }
                '=' => tokens.push(self.lex_equals()?),
                _ => return Err(Error::UnexpectedStartOfToken(c)),
            }
        }

        // - count number of indents on indent_stack
        // - add same # of dedent tokens to token vector. :)
        for _ in 0..self.indent_stack.len() - 1 {
            tokens.push(Token {
                kind: TokenKind::Dedent,
            });
        }
        tokens.push(Token {
            kind: TokenKind::Eof,
        });

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
            "if" => Token {
                kind: TokenKind::If,
            },
            "elif" => Token {
                kind: TokenKind::Elif,
            },
            "else" => Token {
                kind: TokenKind::Else,
            },
            "print" => Token {
                kind: TokenKind::Print,
            },
            "def" => Token {
                kind: TokenKind::Def,
            },
            "return" => Token {
                kind: TokenKind::Return,
            },
            _ => Token {
                kind: TokenKind::Identifier(text),
            },
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
        Token {
            kind: TokenKind::Integer(integer),
        }
    }

    fn lex_comment(&mut self) {
        while let Some(c) = self.current {
            if c == '\n' {
                break;
            }
            self.next();
        }
    }

    fn lex_newline(&mut self) -> Token {
        self.next();
        self.column = 0;
        self.seen_nonblank = false;
        Token {
            kind: TokenKind::Newline,
        }
    }

    fn lex_backslash(&mut self) -> Result<(), Error> {
        if self.next() == Some('\n') {
            self.next();
            Ok(())
        } else {
            Err(Error::UnpairedBackslash(self.current))
        }
    }

    fn lex_whitespace(&mut self) {
        while self.next() == Some(' ') {}
    }

    fn lex_equals(&mut self) -> Result<Token, Error> {
        if self.next() == Some('=') {
            self.next();
            Ok(Token {
                kind: TokenKind::EqEq,
            })
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
            kind: TokenKind::Identifier("hi".to_owned()),
        }],
    }

    token_test! {
        name: if_keyword,
        text: "if",
        token: [Token{
            kind: TokenKind::If,
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
            kind: TokenKind::Integer(1234)
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
            }, Token {
                kind: TokenKind::Integer(39),
            }, Token {
                kind: TokenKind::Dedent,
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
        },
        Token {
            kind: TokenKind::Integer(39),
        },
        Token {
            kind: TokenKind::Newline,
        },
        Token {
            kind: TokenKind::Dedent,
        },
        Token {
            kind: TokenKind::Identifier("hmm".into()),
        }],
    }

    token_test! {
        name: multiple_indent,
        text: "  39\n   hmm\n  1",
        token: [
            Token {
                kind: TokenKind::Indent,
            },
            Token {
                kind: TokenKind::Integer(39),
            },
            Token {
                kind: TokenKind::Newline,
            },
            Token {
                kind: TokenKind::Indent,
            },
            Token {
                kind: TokenKind::Identifier("hmm".into()),
            },
            Token {
                kind: TokenKind::Newline,
            },
            Token {
                kind: TokenKind::Dedent,
            },
            Token {
                kind: TokenKind::Integer(1),
            },
            Token {
                kind: TokenKind::Dedent,
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
            }
        ],
    }

    token_test! {
        name: def,
        text: "def",
        token: [
            Token {
                kind: TokenKind::Def,
            }
        ],
    }

    token_test! {
        name: elif,
        text: "elif",
        token: [
            Token {
                kind: TokenKind::Elif,
            }
        ],
    }

    token_test! {
        name: else_token,
        text: "else",
        token: [
            Token {
                kind: TokenKind::Else,
            }
        ],
    }

    token_test! {
        name: parenl,
        text: "(",
        token: [
            Token {
                kind: TokenKind::ParenL,
            }
        ],
    }

    token_test! {
        name: parenr,
        text: ")",
        token: [
            Token {
                kind: TokenKind::ParenR,
            }
        ],
    }

    token_test! {
        name: return_token,
        text: "return",
        token: [
            Token {
                kind: TokenKind::Return,
            }
        ],
    }

    token_test! {
        name: colon,
        text: ":",
        token: [
            Token {
                kind: TokenKind::Colon,
            }
        ],
    }

    token_test! {
        name:  minus,
        text:  "-",
        token: [
            Token {
                kind: TokenKind::Minus,
            }
        ],
    }

    token_test! {
        name:  plus,
        text:  "+",
        token: [
            Token {
                kind: TokenKind::Plus,
            }
        ],
    }

    token_test! {
        name: eqeq,
        text: "==",
        token: [
            Token {
                kind: TokenKind::EqEq,
            }
        ],
    }

    error_test! {
        name:  eqeq_error,
        text:  "=",
        error: UnexpectedCharacter(None),
    }
}
