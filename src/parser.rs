// brings Token and variants of Token into scope
use token::Token::{self, *};
use error::Error;
use program::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: Token,
}

impl Parser {
    pub fn new(mut tokens: Vec<Token>) -> Parser {
        let current = tokens.remove(0);
        Parser {
            tokens,
            current,
        }
    }

    fn next(&mut self) {
        if self.tokens.is_empty() {
            panic!("Token stream empty.");
        }
        self.current = self.tokens.remove(0);
    }

    pub fn parse_program(mut self) -> Result<Program, Error> {
        let mut statements = Vec::new();
        loop {
            match self.current {
                Eof => break,
                Newline => self.next(),
                _ => statements.push(self.parse_statement()?),
            }
        }
        if !self.tokens.is_empty() {
            panic!("Did not consume token stream.");
        }
        Ok(Program{statements})
    }

    fn parse_statement(&mut self) -> Result<Statement, Error> {
        match self.current {
            Print => {
                self.next();
                Ok(Statement::Print(self.parse_expression()?))
            }
            _ => Err(Error::UnexpectedToken(self.current.clone())),
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, Error> {
        let v = self.parse_value()?;
        match self.current {
            Plus => {
                self.next();
                let e = self.parse_expression()?;
                Ok(Expression::Add(v, Box::new(e)))
            }
            Minus => {
                self.next();
                let e = self.parse_expression()?;
                Ok(Expression::Sub(v, Box::new(e)))
            }
            _ => Ok(Expression::Simple(v)),
        }
    }

    fn parse_value(&mut self) -> Result<Value, Error> {
        match self.current.clone() {
            Integer(i) => {
                self.next();
                Ok(Value::Integer(i))
            }
            Identifier(s) => {
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

    fn lex(text: &str) -> Vec<Token> {
        let lexer = Lexer::new(text);
        lexer.lex().unwrap()
    }

    fn parse(text: &str) -> Result<Program, Error> {
        let parser = Parser::new(lex(text));
        parser.parse_program()
    }

    #[test]
    fn empty_program() {
        let program = parse(" ").unwrap();
        assert_eq!(program.statements, vec![]);
        // assert_eq!(program, Program{statements: vec![]})
    }

    #[test]
    fn print_integer() {
        let program = parse("print 7").unwrap();
        assert_eq!(program.statements, vec![Statement::Print(Expression::Simple(Value::Integer(7)))]);
    }

    #[test]
    fn print_variable() {
        let program = parse("print name").unwrap();
        assert_eq!(program.statements, vec![Statement::Print(Expression::Simple(Value::Variable("name".to_owned())))]);
    }

    #[test]
    fn print_add() {
        let program = parse("print 1 + 1").unwrap();
        assert_eq!(program.statements, vec![
            Statement::Print(
                Expression::Add(
                    Value::Integer(1),
                    Box::new(Expression::Simple(
                        Value::Integer(1)
                    ))
                )
            )
        ]);
    }

    #[test]
    fn print_sub() {
        let program = parse("print 2- 1").unwrap();
        assert_eq!(program.statements, vec![
            Statement::Print(
                Expression::Sub(
                    Value::Integer(2),
                    Box::new(Expression::Simple(
                        Value::Integer(1)
                    ))
                )
            )
        ]);
    }


    macro_rules! test {
        () => {

        }
    }

    test! {
       //  name:    print_variable,
       //  text:    "print name",
       //  program: Program{statements: vec![
       //      Statement::Print(Expression::Simple(Value::Variable("name".to_owned())))
       // ]},
    }
}
