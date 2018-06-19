// struct instance variables private by default
pub struct Program {
    pub statements: Vec<Statement>,
}

// enum variants and their fields public by default
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Print(Expression),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Add(Value, Box<Expression>),
    Sub(Value, Box<Expression>),
    Simple(Value),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Integer(u32),
    Variable(String),
}
