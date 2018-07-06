// struct instance variables private by default
#[derive(Debug)]
pub struct Program {
    //pub statements: Vec<Statement>,
    pub body: Body,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Body {
    pub statements: Vec<Statement>,
}

// enum variants and their fields public by default
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Print(Expression),
    Return(Expression),
    If {
        condition: Expression,
        body: Body,
    },
    Elif {
        condition: Expression,
        body: Body,
    },
    Else(Body),
    Def {
        name: String,
        params: Vec<String>,
        body: Body,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    EqEq(Box<Expression>, Box<Expression>),
    Lt(Box<Expression>, Box<Expression>),
    Gt(Box<Expression>, Box<Expression>),
    Add(Value, Box<Expression>),
    Sub(Value, Box<Expression>),
    Simple(Value),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Integer(u32),
    Variable(String),
    // Complex(Box<Expression>), () precedence
}
