// struct instance variables private by default
pub struct Program {
    pub statements: Vec<Statement>,
    // pub body: Body,
}

// pub struct Body {
//     pub statements: Vec<Statement>,
// }

// enum variants and their fields public by default
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Print(Expression),
    // If {
    //     condition: Expression,
    //     body: Body,
    //     elifs: Vec<(Expression, Body)>,
    //     else_body: Option<Body>,
    // },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    EqEq(Term, Term),
    Simple(Term),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Term {
    Add(Value, Box<Term>),
    Sub(Value, Box<Term>),
    Simple(Value),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Integer(u32),
    Variable(String),
    // Complex(Box<Expression>), () precedence
}
