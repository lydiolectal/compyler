use program::*;
use wexp::Wexp::{self, *};

macro_rules! wasm {
    ($i:ident) => {::wexp::Wexp::Atom(stringify!($i).to_string())};
    ($s:expr) => {{
        let s: &str = $s;
        ::wexp::Wexp::Atom(s.to_string())
    }}
    /*
    (
        (
            $($item:expr)*
        )
    ) => {{
        let mut v = Vec::new();
        $(
        {
            let i = wasm!($item);
            v.push(i);
        }
        )*
        ::exp::Wexp::List(v);
    }}
    */
}

impl Program {
    pub fn codegen(&self) -> Wexp {
        let print = List(
            vec![wasm!(func), Atom("$i".to_string()), List(
                vec![wasm!(import), wasm!("\"host\""), wasm!("\"print\"")]
            ),
            List(
                vec![wasm!(param), wasm!(i32)]
            )]
        );
        let mut module = vec![wasm!(module), print];
        let mut main = vec![wasm!(func), List(vec![wasm!(export), wasm!("\"main\"")])];
        for stmt in &self.statements {
            main.extend(stmt.codegen());
        }
        // for some other def statements in self.statements
            // module.push(List(stmt))
        module.push(List(main));
        List(module)
    }
}

impl Statement {
    pub fn codegen(&self) -> Vec<Wexp> {
        let mut atoms = vec![];
        match self {
            Statement::Print(e) => {
                let expr = e.codegen();
                atoms.push(Atom("i32.const".to_owned()));
                atoms.extend(expr);
                atoms.extend(vec![wasm!(call), Atom("$i".to_owned())]);
            }
            _        => {

            }
        }
        atoms
    }
}


impl Expression {
    pub fn codegen(&self) -> Vec<Wexp> {
        let mut atoms = vec![];
        match self {
            Expression::Simple(v) => {
                let val = v.codegen();
                atoms.push(val);
            }
            _         => {

            }
        }
        atoms
    }
}

impl Value {
    pub fn codegen(&self) -> Wexp {
        match self {
            Value::Integer(i)  => {
                Atom(i.to_string())
            }
            Value::Variable(v) => {
                Atom(v.to_string())
            }
        }
    }
}


#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_empty_program() {
        let p = Program {
            statements: vec![],
        };
        let wexp = p.codegen();
        assert_eq!(wexp.to_string(),
        "(module (func $i (import \"host\" \"print\") (param i32)) (func (export \"main\")))");
    }

    #[test]
    fn test_print_int() {
        let p = Program {
            statements: vec![Statement::Print(
                Expression::Simple(
                    Value::Integer(24)))]
        };
        let wexp = p.codegen();
        assert_eq!(wexp.to_string(),
        "(module (func $i (import \"host\" \"print\") (param i32)) (func (export \"main\") i32.const 24 call $i))")

        /*
        (module
            (func $i
                (import "host" "print")
                (param i32))
            // everything not inside a "def" block is in main.
            // everything in "def" has its own func (export)
            (func (export "main")
                i32.const 42
                call $i)
        )
        */
    }
}
