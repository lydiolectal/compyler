use common::*;

pub fn lex(text: &str) -> Vec<Token> {
    let lexer = Lexer::new(text);
    lexer.lex().unwrap()
}

pub fn parse(text: &str) -> Result<Program, Error> {
    let parser = Parser::new(lex(text));
    parser.parse_program()
}

pub fn codegen(text: &str) -> String {
    let program = parse(text).unwrap();
    let codegenerator = CodeGenerator::new(program);
    codegenerator.codegen().to_string()
}
