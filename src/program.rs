// struct instance variables private by default
#[derive(Debug, Clone)]
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
        elif: Vec<(Expression, Body)>,
        else_body: Option<Body>,
    },
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
    Leq(Box<Expression>, Box<Expression>),
    Geq(Box<Expression>, Box<Expression>),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mult(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Mod(Box<Expression>, Box<Expression>),
    Call {
        name: String,
        params: Vec<Expression>,
    },
    Simple(Value),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Integer(u32),
    Variable(String),
    // Complex(Box<Expression>), () precedence
}
