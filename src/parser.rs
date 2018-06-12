// brings Token and variants of Token into scope
use token::Token::{self, *};
use error::Error;

pub struct Parser {
    tokens: Vec<Token>,
    current: Token,
}

pub struct Program {
    statements: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Print(Expression),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Simple(Value),
    // Addition(Value, Expression),
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Integer(u32),
    // String(String),
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
        // match self.current {
        //
        // }
        Ok(Expression::Simple(self.parse_value()?))
    }

    fn parse_value(&mut self) -> Result<Value, Error> {
        match self.current {
            Integer(i) => {
                self.next();
                Ok(Value::Integer(i))
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
    fn print() {
        let program = parse("print 7").unwrap();
        assert_eq!(program.statements, vec![Statement::Print(Expression::Simple(Value::Integer(7)))]);
    }

}
