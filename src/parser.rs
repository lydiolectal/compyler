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
        if !self.tokens.is_empty() {
            panic!(
                "Did not consume token stream at {}.",
                self.current.lexeme.clone()
            );
        }
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
        self.next();
        if res.kind == kind {
            Ok(res)
        } else {
            Err(Error::UnexpectedToken(res))
        }
    }

    fn parse_def_params(&mut self) -> Vec<String> {
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
        self.expect(TokenKind::ParenL)?;
        let params = self.parse_def_params();
        self.expect(TokenKind::ParenR)?;
        self.expect(TokenKind::Colon)?;
        self.expect(TokenKind::Newline)?;
        self.expect(TokenKind::Indent)?;
        let body = self.parse_body()?;
        Ok(Statement::Def {
            name: name_string.to_owned(),
            params: params,
            body,
        })
    }

    fn parse_if(&mut self) -> Result<Statement, Error> {
        let condition = self.parse_comparison()?;
        self.expect(TokenKind::Colon)?;
        self.expect(TokenKind::Newline)?;
        self.expect(TokenKind::Indent)?;
        let body = self.parse_body()?;
        let elif = self.parse_elif()?;
        let else_body = self.parse_else()?;
        Ok(Statement::If {
            condition,
            body,
            elif,
            else_body,
        })
    }

    fn parse_elif(&mut self) -> Result<Vec<(Expression, Body)>, Error> {
        let mut elif = Vec::new();
        loop {
            if self.current.kind == TokenKind::Elif {
                self.next();
                let condition = self.parse_comparison()?;
                self.expect(TokenKind::Colon)?;
                self.expect(TokenKind::Newline)?;
                self.expect(TokenKind::Indent)?;
                let body = self.parse_body()?;
                elif.push((condition, body));
            } else {
                break;
            }
        }
        Ok(elif)
    }

    fn parse_else(&mut self) -> Result<Option<Body>, Error> {
        if self.current.kind == TokenKind::Else {
            self.next();
            self.expect(TokenKind::Colon)?;
            self.expect(TokenKind::Newline)?;
            self.expect(TokenKind::Indent)?;
            let body = self.parse_body()?;
            Ok(Some(body))
        } else {
            Ok(None)
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, Error> {
        let c = self.parse_comparison()?;
        match self.current.kind {
            And => {
                self.next();
                let e = self.parse_expression()?;
                Ok(Expression::And(Box::new(c), Box::new(e)))
            }
            Or => {
                self.next();
                let e = self.parse_expression()?;
                Ok(Expression::Or(Box::new(c), Box::new(e)))
            }
            _ => Ok(c),
        }
    }

    fn parse_comparison(&mut self) -> Result<Expression, Error> {
        let t = self.parse_term()?;
        match self.current.kind {
            EqEq => {
                self.next();
                let e = self.parse_comparison()?;
                Ok(Expression::EqEq(Box::new(t), Box::new(e)))
            }
            Ne => {
                self.next();
                let e = self.parse_comparison()?;
                Ok(Expression::Ne(Box::new(t), Box::new(e)))
            }
            Lt => {
                self.next();
                let e = self.parse_comparison()?;
                Ok(Expression::Lt(Box::new(t), Box::new(e)))
            }
            Gt => {
                self.next();
                let e = self.parse_comparison()?;
                Ok(Expression::Gt(Box::new(t), Box::new(e)))
            }
            Leq => {
                self.next();
                let e = self.parse_comparison()?;
                Ok(Expression::Leq(Box::new(t), Box::new(e)))
            }
            Geq => {
                self.next();
                let e = self.parse_comparison()?;
                Ok(Expression::Geq(Box::new(t), Box::new(e)))
            }
            _ => Ok(t),
        }
    }

    fn parse_term(&mut self) -> Result<Expression, Error> {
        let c = self.parse_product()?;
        match self.current.kind {
            Plus => {
                self.next();
                let e = self.parse_term()?;
                Ok(Expression::Add(Box::new(c), Box::new(e)))
            }
            Minus => {
                self.next();
                let e = self.parse_term()?;
                Ok(Expression::Sub(Box::new(c), Box::new(e)))
            }
            _ => Ok(c),
        }
    }

    fn parse_product(&mut self) -> Result<Expression, Error> {
        let c = self.parse_primary()?;
        match self.current.kind {
            Mult => {
                self.next();
                let e = self.parse_product()?;
                Ok(Expression::Mult(Box::new(c), Box::new(e)))
            }
            Div => {
                self.next();
                let e = self.parse_product()?;
                Ok(Expression::Div(Box::new(c), Box::new(e)))
            }
            Mod => {
                self.next();
                let e = self.parse_product()?;
                Ok(Expression::Mod(Box::new(c), Box::new(e)))
            }
            _ => Ok(c),
        }
    }

    fn parse_primary(&mut self) -> Result<Expression, Error> {
        let v = self.parse_value()?;
        match self.current.kind {
            ParenL => match v {
                Value::Variable(s) => self.parse_call(s),
                _ => Err(Error::UnexpectedToken(self.current.clone())),
            },
            _ => Ok(Expression::Simple(v)),
        }
    }

    fn parse_call(&mut self, name: String) -> Result<Expression, Error> {
        self.next();
        let mut params = Vec::new();
        while self.current.kind != ParenR {
            let e = self.parse_comparison()?;
            params.push(e);
            match self.current.kind {
                Comma => self.next(),
                _ => break,
            }
        }
        self.expect(TokenKind::ParenR)?;
        Ok(Expression::Call { name, params })
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
            Token {
                kind: TokenKind::ParenL,
                ..
            } => {
                self.next();
                let expr = self.parse_expression()?;
                self.expect(TokenKind::ParenR);
                let complex_atom = Value::Complex(Box::new(expr));
                Ok(complex_atom)
            }
            _ => Err(Error::UnexpectedToken(self.current.clone())),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use common::*;
    use testing::*;

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

    parse_test! {
        name: parse_empty_program,
        text: " ",
        program: [],
    }

    parse_test! {
        name: print_integer,
        text: "print 7",
        program: [Statement::Print(
            Expression::Simple(
                Value::Integer(7))
        )],
    }

    parse_test! {
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

    parse_test! {
        name:    print_add,
        text:    "print 1 + 1",
        program: [
            Statement::Print(
                Expression::Add(
                    Box::new(Expression::Simple(
                        Value::Integer(1))),
                    Box::new(Expression::Simple(
                        Value::Integer(1)
                        )
                    )
                )
            )
        ],
    }

    parse_test! {
        name:    print_sub,
        text:    "print 2- 1",
        program: [
            Statement::Print(
                Expression::Sub(
                    Box::new(Expression::Simple(
                        Value::Integer(2))),
                    Box::new(Expression::Simple(
                        Value::Integer(1)
                        )
                    )
                )
            )
        ],
    }

    parse_test! {
        name:    print_mult,
        text:    "print 2*2 - 1",
        program: [
            Statement::Print(
                Expression::Sub(
                    Box::new(Expression::Mult(
                        Box::new(Expression::Simple(
                            Value::Integer(2)
                        )),
                        Box::new(Expression::Simple(
                            Value::Integer(2)
                        ))
                    )),
                    Box::new(Expression::Simple(
                        Value::Integer(1)
                        )
                    )
                )
            )
        ],
    }

    parse_test! {
        name:    print_mod_sub,
        text:    "print 2%2 - 1",
        program: [
            Statement::Print(
                Expression::Sub(
                    Box::new(Expression::Mod(
                        Box::new(Expression::Simple(
                            Value::Integer(2)
                        )),
                        Box::new(Expression::Simple(
                            Value::Integer(2)
                        ))
                    )),
                    Box::new(Expression::Simple(
                        Value::Integer(1)
                        )
                    )
                )
            )
        ],
    }

    parse_test! {
        name:    print_mod_mult,
        text:    "print 5 % 2*3",
        program: [
            Statement::Print(
                Expression::Mod(
                    Box::new(Expression::Simple(
                        Value::Integer(5)
                    )),
                    Box::new(Expression::Mult(
                        Box::new(Expression::Simple(
                            Value::Integer(2)
                        )),
                        Box::new(Expression::Simple(
                            Value::Integer(3)
                        ))
                    ))
                )
            )
        ],
    }

    parse_test! {
        name:    print_paren_expression,
        text:    "print (2 + 1) * 7",
        program: [
            Statement::Print(
                Expression::Mult(
                    Box::new(Expression::Simple(
                        Value::Complex(
                            Box::new(Expression::Add(
                                Box::new(Expression::Simple(
                                    Value::Integer(2)
                                )),
                                Box::new(Expression::Simple(
                                    Value::Integer(1)
                                ))
                            ))
                        )
                    )),

                    Box::new(Expression::Simple(
                        Value::Integer(7)
                    ))
                )
            )
        ],
    }

    parse_test! {
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

    parse_test! {
        name:    print_complex_eqeq,
        text:    "print 0 + 1 == 1",
        program: [
            Statement::Print(
                Expression::EqEq(
                    Box::new(Expression::Add(
                        Box::new(Expression::Simple(
                            Value::Integer(0))),
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
        name: return_int,
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
        name:    return_complex_eqeq,
        text:    "return 0 + 1 == 1",
        program: [
            Statement::Return(
                Expression::EqEq(
                    Box::new(
                        Expression::Add(
                            Box::new(Expression::Simple(
                                Value::Integer(0))),
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
        name: print_lt,
        text: "print 1 < 2",
        program: [Statement::Print(
            Expression::Lt(
                Box::new(Expression::Simple(
                    Value::Integer(1)
                )),
                Box::new(Expression::Simple(
                    Value::Integer(2)
                ))
            )
        )],
    }

    parse_test! {
        name: print_geq,
        text: "print 1>=2",
        program: [Statement::Print(
            Expression::Geq(
                Box::new(Expression::Simple(
                    Value::Integer(1)
                )),
                Box::new(Expression::Simple(
                    Value::Integer(2)
                ))
            )
        )],
    }

    parse_test! {
        name: print_ne,
        text: "print 1 !=2",
        program: [Statement::Print(
            Expression::Ne(
                Box::new(Expression::Simple(
                    Value::Integer(1)
                )),
                Box::new(Expression::Simple(
                    Value::Integer(2)
                ))
            )
        )],
    }

    parse_test! {
        name: print_and,
        text: "print 1>=2 and 2<7",
        program: [Statement::Print(
            Expression::And(
                Box::new(Expression::Geq(
                    Box::new(Expression::Simple(
                        Value::Integer(1)
                    )),
                    Box::new(Expression::Simple(
                        Value::Integer(2)
                    ))
                )),
                Box::new(Expression::Lt(
                    Box::new(Expression::Simple(
                        Value::Integer(2)
                    )),
                    Box::new(Expression::Simple(
                        Value::Integer(7)
                    ))
                ))
            )
        )],
    }

    parse_test! {
        name: print_complex_gt,
        text: "print 1 + 3 > 2 - 1",
        program: [Statement::Print(
            Expression::Gt(
                Box::new(Expression::Add(
                    Box::new(Expression::Simple(
                        Value::Integer(1))),
                    Box::new(Expression::Simple(
                        Value::Integer(3)
                    ))
                )),
                Box::new(Expression::Sub(
                    Box::new(Expression::Simple(
                        Value::Integer(2))),
                    Box::new(Expression::Simple(
                        Value::Integer(1)
                    ))
                ))
            )
        )],
    }

    parse_test! {
        name: def_simple_func,
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
        name: def_complex_func,
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
        name: def_simple_func_param,
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
        name: def_simple_func_params,
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
        name: def_missing_paren,
        text: "def fib(a, bb, ccc:\n   print 0",
        error: Error::UnexpectedToken(Token {
            kind: Colon,
            lexeme: ":".to_owned(),
        }),
    }

    parse_test! {
        name: function_call,
        text: "print foo(n, 7+ 9)",
        program: [Statement::Print(
            Expression::Call {
                name: "foo".to_owned(),
                params: vec![
                    Expression::Simple(
                        Value::Variable("n".to_owned())
                    ),
                    Expression::Add(
                        Box::new(Expression::Simple(
                            Value::Integer(7))),
                        Box::new(Expression::Simple(
                            Value::Integer(9)
                        ))
                    )
                ],
            }
        )],
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
                },
                elif: vec![],
                else_body: None,
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

    parse_test! {
        name: parse_if_elif,
        text: "if a:\n  print 7\nelif b:\n  print 8",
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
                },
                elif: vec![(Expression::Simple(
                    Value::Variable("b".to_owned())),
                    Body {
                        statements: vec![
                            Statement::Print(
                                Expression::Simple(
                                    Value::Integer(8)
                                )
                            ),
                        ]
                    })],
                else_body: None,
            }],
    }

    parse_test! {
        name: parse_if_else,
        text: "if a:\n  print 7\nelse:\n  print 8",
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
                },
                elif: vec![],
                else_body: Some(
                    Body {
                        statements: vec![
                            Statement::Print(
                                Expression::Simple(
                                    Value::Integer(8)
                                )
                            ),
                        ]
                    }),
            }],
    }

    parse_test! {
        name: parse_if_elif_else,
        text: "if a:\n  print 7\nelif b:\n  print 8\nelse:\n  print 9",
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
                },
                elif: vec![(Expression::Simple(
                    Value::Variable("b".to_owned())),
                    Body {
                        statements: vec![
                            Statement::Print(
                                Expression::Simple(
                                    Value::Integer(8)
                                )
                            ),
                        ]
                    })],
                else_body: Some(
                    Body {
                        statements: vec![
                            Statement::Print(
                                Expression::Simple(
                                    Value::Integer(9)
                                )
                            ),
                        ]
                    }
                ),
            }],
    }

    parse_test! {
        name: parse_fib,
        text: "def fib(n):\n  if n < 2:\n   return n\n  else:\n   return fib(n - 2) + fib(n - 1)",
        program:
            [Statement::Def{
                name: "fib".to_owned(),
                params: vec!["n".to_owned()],
                body: Body {
                    statements: vec![
                        Statement::If {
                            condition: Expression::Lt(
                                Box::new(Expression::Simple(
                                    Value::Variable("n".to_owned())
                                )),
                                Box::new(Expression::Simple(
                                    Value::Integer(2)
                                ))
                            ),
                            body: Body {
                                statements: vec![Statement::Return(
                                    Expression::Simple(
                                        Value::Variable("n".to_owned())
                                    )
                                )]
                            },
                            elif: vec![],
                            else_body: Some(
                                Body {
                                    statements: vec![Statement::Return(
                                        Expression::Add(
                                            Box::new(Expression::Call{
                                                name: "fib".to_owned(),
                                                params: vec![
                                                    Expression::Sub(
                                                        Box::new(Expression::Simple(
                                                            Value::Variable("n".to_owned())
                                                        )),
                                                        Box::new(Expression::Simple(
                                                            Value::Integer(2)
                                                        ))
                                                    )
                                                ],
                                            }),
                                            Box::new(Expression::Call{
                                                name:"fib".to_owned(),
                                                params: vec![
                                                    Expression::Sub(
                                                        Box::new(Expression::Simple(
                                                            Value::Variable("n".to_owned())
                                                        )),
                                                        Box::new(Expression::Simple(
                                                            Value::Integer(1)
                                                        ))
                                                    )
                                                ],
                                            })
                                        )
                                        )
                                    ]
                                }
                            )
                        }
                    ]
                },
            }],
    }

}
