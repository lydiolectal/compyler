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
        match self.current.kind {
            Eof => Ok(Program { body }),
            _ => Err(Error::UnexpectedToken(self.current.clone())),
        }
    }

    fn parse_body(&mut self) -> Result<Body, Error> {
        let mut statements = Vec::new();
        loop {
            match self.current.kind {
                Eof => break,
                Dedent => {
                    self.next();
                    break;
                }
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
            Def => {
                self.next();
                self.parse_def()
            }
            If => {
                self.next();
                self.parse_if()
            }
            _ => Err(Error::UnexpectedToken(self.current.clone())),
        }
    }

    fn expect(&mut self, kind: TokenKind) -> Result<Token, Error> {
        let res = self.current.clone();
        if self.current.kind == kind {
            Ok(res)
        } else {
            Err(Error::UnexpectedToken(res))
        }
    }

    fn parse_params(&mut self) -> Vec<String> {
        let mut params = Vec::new();
        loop {
            match self.current.kind {
                TokenKind::Identifier => {
                    params.push(self.current.lexeme.clone());
                    self.next();
                }
                _ => break,
            }
            match self.current.kind {
                TokenKind::Comma => {
                    self.next();
                }
                _ => break,
            }
        }
        params
    }

    fn parse_def(&mut self) -> Result<Statement, Error> {
        let name_token = self.expect(TokenKind::Identifier)?;
        let name_string = name_token.lexeme;
        self.next();
        self.expect(TokenKind::ParenL)?;
        self.next();
        let params = self.parse_params();
        self.expect(TokenKind::ParenR)?;
        self.next();
        self.expect(TokenKind::Colon)?;
        self.next();
        self.expect(TokenKind::Newline)?;
        self.next();
        self.expect(TokenKind::Indent)?;
        self.next();
        let body = self.parse_body()?;
        Ok(Statement::Def {
            name: name_string.to_owned(),
            params: params,
            body,
        })
    }

    fn parse_if(&mut self) -> Result<Statement, Error> {
        let condition = self.parse_expression()?;
        self.expect(TokenKind::Colon)?;
        self.next();
        self.expect(TokenKind::Newline)?;
        self.next();
        self.expect(TokenKind::Indent)?;
        self.next();
        let body = self.parse_body()?;
        Ok(Statement::If { condition, body })
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
        match self.current.clone() {
            Token {
                kind: TokenKind::Integer,
                lexeme: i,
            } => {
                self.next();
                let int_i = i.chars()
                    .fold(0, |acc, c| acc * 10 + (c as u32 - '0' as u32));
                Ok(Value::Integer(int_i))
            }
            Token {
                kind: TokenKind::Identifier,
                lexeme: s,
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

    macro_rules! parse_test {
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

    macro_rules! error_test {
        (name: $name:ident,text: $text:expr,error: $expected:expr,) => {
            #[test]
            fn $name() {
                let text = $text;
                let expected = $expected;
                let error = parse(text).unwrap_err();
                assert_eq!(error, expected);
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

    parse_test! {
        name: parse_empty_program,
        text: " ",
        program: [],
    }

    parse_test! {
        name: parse_print_integer,
        text: "print 7",
        program: [Statement::Print(
            Expression::Simple(
                Value::Integer(7))
        )],
    }

    parse_test! {
        name:    parse_print_variable,
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

    parse_test! {
        name:    parse_print_add,
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

    parse_test! {
        name:    parse_print_sub,
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

    parse_test! {
        name:    parse_print_eqeq,
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

    parse_test! {
        name:    parse_print_complex_eqeq,
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

    parse_test! {
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

    parse_test! {
        name:    parse_return_complex_eqeq,
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

    parse_test! {
        name: parse_def_simple_func,
        text: "def fib():\n   print 0",
        program:
            [Statement::Def{
                name: "fib".to_owned(),
                params: vec![],
                body: Body {
                    statements: vec![Statement::Print(
                        Expression::Simple(
                            Value::Integer(0)
                        )
                    )]
                }
            }],
    }

    parse_test! {
        name: parse_def_complex_func,
        text: "def fib():\n   print 0\n   print 1",
        program:
            [Statement::Def{
                name: "fib".to_owned(),
                params: vec![],
                body: Body {
                    statements: vec![
                    Statement::Print(
                        Expression::Simple(
                            Value::Integer(0)
                        )
                    ),
                    Statement::Print(
                        Expression::Simple(
                            Value::Integer(1)
                        )
                    )]
                }
            }],
    }

    parse_test! {
        name: parse_def_simple_func_param,
        text: "def fib(a):\n   print 0",
        program:
            [Statement::Def{
                name: "fib".to_owned(),
                params: vec!["a".to_owned()],
                body: Body {
                    statements: vec![Statement::Print(
                        Expression::Simple(
                            Value::Integer(0)
                        )
                    )]
                }
            }],
    }

    parse_test! {
        name: parse_def_simple_func_params,
        text: "def fib(a, bb, ccc):\n   print 0",
        program:
            [Statement::Def{
                name: "fib".to_owned(),
                params: vec!["a".to_owned(), "bb".to_owned(), "ccc".to_owned()],
                body: Body {
                    statements: vec![Statement::Print(
                        Expression::Simple(
                            Value::Integer(0)
                        )
                    )]
                }
            }],
    }

    error_test! {
        name: parse_def_missing_paren,
        text: "def fib(a, bb, ccc:\n   print 0",
        error: Error::UnexpectedToken(Token {
            kind: Colon,
            lexeme: ":".to_owned(),
        }),
    }

    parse_test! {
        name: parse_if,
        text: "if a:\n  print 7",
        program:
            [Statement::If{
                condition: Expression::Simple(
                    Value::Variable("a".to_owned())),
                body: Body {
                    statements: vec![
                        Statement::Print(
                            Expression::Simple(
                                Value::Integer(7)
                            )
                        ),
                    ]
                }

            }],
    }

    error_test! {
        name: parse_if_error,
        text: "def fib(a, bb, ccc:\n   print 0",
        error: Error::UnexpectedToken(Token {
            kind: Colon,
            lexeme: ":".to_owned(),
        }),
    }

}
