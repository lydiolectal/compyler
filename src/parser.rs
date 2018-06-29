use error::Error;
use program::*;
use token::{Token, TokenKind::{self, *}};

pub struct Parser {
    tokens: Vec<Token>,
    current: Token,
}

impl Parser {
    pub fn new(mut tokens: Vec<Token>) -> Parser {
        let current = tokens.remove(0);
        Parser { tokens, current }
    }

    fn next(&mut self) {
        if self.tokens.is_empty() {
            panic!("Token stream empty.");
        }
        self.current = self.tokens.remove(0);
    }

    pub fn parse_program(mut self) -> Result<Program, Error> {
        let body = self.parse_body()?;
        Ok(Program { body })
    }

    fn parse_body(&mut self) -> Result<Body, Error> {
        let mut statements = Vec::new();
        loop {
            match self.current.kind {
                Eof => break,
                Newline => self.next(),
                _ => statements.push(self.parse_statement()?),
            }
        }
        if !self.tokens.is_empty() {
            panic!("Did not consume token stream.");
        }
        Ok(Body { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement, Error> {
        match self.current.kind {
            Print => {
                self.next();
                Ok(Statement::Print(self.parse_expression()?))
            }
            Return => {
                self.next();
                Ok(Statement::Return(self.parse_expression()?))
            }
            Def => self.parse_def(),
            _ => Err(Error::UnexpectedToken(self.current.clone())),
        }
    }

    fn parse_def(&mut self) -> Result<Statement, Error> {
        self.next();

        unimplemented!();

        /*

        struct Token {
            kind:   TokenKind,
            lexeme: String,
        }

        let name_token = self.expect(TokenKind::Identifier)?;
        let name_string = name_token.lexeme;



        if let Token::Identifier(name) = self.current {

        } else {
            return Err(Error::UnexpectedToken(self.current.clone()));
        }
        */
    }

    fn parse_expression(&mut self) -> Result<Expression, Error> {
        let t = self.parse_term()?;
        match self.current.kind {
            EqEq => {
                self.next();
                let e = self.parse_term()?;
                Ok(Expression::EqEq(Box::new(t), Box::new(e)))
            }
            _ => Ok(t),
        }
    }

    fn parse_term(&mut self) -> Result<Expression, Error> {
        let v = self.parse_value()?;
        match self.current.kind {
            Plus => {
                self.next();
                let e = self.parse_term()?;
                Ok(Expression::Add(v, Box::new(e)))
            }
            Minus => {
                self.next();
                let e = self.parse_term()?;
                Ok(Expression::Sub(v, Box::new(e)))
            }
            _ => Ok(Expression::Simple(v)),
        }
    }

    fn parse_value(&mut self) -> Result<Value, Error> {
        match self.current {
            Token {
                kind: TokenKind::Integer(i),
            } => {
                self.next();
                Ok(Value::Integer(i))
            }
            Token {
                kind: TokenKind::Identifier(s),
            } => {
                self.next();
                Ok(Value::Variable(s))
            }
            _ => Err(Error::UnexpectedToken(self.current.clone())),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use lexer::Lexer;

    macro_rules! test {
        (name: $name:ident,text: $text:expr,program: $expected:expr,) => {
            #[test]
            fn $name() {
                let text = $text;
                let expected = $expected.to_vec();
                let program = parse(text).unwrap();
                assert_eq!(program.body.statements, expected);
            }
        };
    }

    fn lex(text: &str) -> Vec<Token> {
        let lexer = Lexer::new(text);
        lexer.lex().unwrap()
    }

    fn parse(text: &str) -> Result<Program, Error> {
        let parser = Parser::new(lex(text));
        parser.parse_program()
    }

    test! {
        name: empty_program,
        text: " ",
        program: [],
    }

    test! {
        name: print_integer,
        text: "print 7",
        program: [Statement::Print(
            Expression::Simple(
                Value::Integer(7))
        )],
    }

    test! {
        name:    print_variable,
        text:    "print name",
        program: [
            Statement::Print(
                Expression::Simple(
                    Value::Variable(
                    "name".to_owned()
                    )
                )
            )
        ],
    }

    test! {
        name:    print_add,
        text:    "print 1 + 1",
        program: [
            Statement::Print(
                Expression::Add(
                    Value::Integer(1),
                    Box::new(Expression::Simple(
                        Value::Integer(1)
                        )
                    )
                )
            )
        ],
    }

    test! {
        name:    print_sub,
        text:    "print 2- 1",
        program: [
            Statement::Print(
                Expression::Sub(
                    Value::Integer(2),
                    Box::new(Expression::Simple(
                        Value::Integer(1)
                        )
                    )
                )
            )
        ],
    }

    test! {
        name:    print_eqeq,
        text:    "print 0 == 1",
        program: [
            Statement::Print(
                Expression::EqEq(
                    Box::new(Expression::Simple(
                        Value::Integer(0))
                    ),
                    Box::new(Expression::Simple(
                        Value::Integer(1))
                    )
                )
            )
        ],
    }

    test! {
        name:    print_complex_eqeq,
        text:    "print 0 + 1 == 1",
        program: [
            Statement::Print(
                Expression::EqEq(
                    Box::new(Expression::Add(
                        Value::Integer(0),
                        Box::new(Expression::Simple(
                            Value::Integer(1)
                        ))
                    )),
                    Box::new(Expression::Simple(
                        Value::Integer(1)
                    ))
                )
            )
        ],
    }

    test! {
        name: parse_return,
        text: "return 9",
        program: [
            Statement::Return(
                Expression::Simple(
                    Value::Integer(9)
                )
            )
        ],
    }

    test! {
        name:    return_complex_eqeq,
        text:    "return 0 + 1 == 1",
        program: [
            Statement::Return(
                Expression::EqEq(
                    Box::new(
                        Expression::Add(
                            Value::Integer(0),
                            Box::new(Expression::Simple(
                                Value::Integer(1)
                                )
                            )
                        )
                    ),
                    Box::new(
                        Expression::Simple(
                            Value::Integer(1)
                        )
                    )
                )
            )
        ],
    }

    // test! {
    //     name: def_simple_func,
    //     text: "def fib():\n   print 0",
    //     program:
    //         [],
    // }

    // add fib test
}
