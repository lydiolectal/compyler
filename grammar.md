program    : statement* EOF

statement  : NEWLINE
           | PRINT expression

expression : INTEGER

struct Program {
    statements: Vec<Statement>,
}

enum Statement {
    Print(Expression),
    If(Expression, Suite),
}

enum Expression {
    Integer(u64),
    String(String),
    Variable(String),
}

fn program(&mut self) -> Result<Program, Error> {
    let statements = Vec::new();
    loop {
        match current {
            Newline => {}
            Print => statements.push(self.parse_print_statement()),
        }
    }

    Ok(Program{statements})
}

fn parse_print_statement(&self) -> Result<Statement, Error> {

}
