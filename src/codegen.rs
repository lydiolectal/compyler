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
                atoms.extend(val);
            }
            Expression::Add(v, ref e) => {
                // TODO: fix this grodiness
                let val = v.codegen();
                let expr = e.codegen();
                atoms.extend(val);
                atoms.extend(expr);
                atoms.push(Atom("i32.add".to_owned()));
            }
            _         => {

            }
        }
        atoms
    }
}

impl Value {
    pub fn codegen(&self) -> Vec<Wexp> {
        let mut atoms = vec![];
        match self {
            Value::Integer(i)  => {
                atoms.push(Atom("i32.const".to_owned()));
                atoms.push(Atom(i.to_string()));
            }
            Value::Variable(v) => {
                Atom(v.to_string());
            }
        }
        atoms
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
        "(module (func $i (import \"host\" \"print\") \
        (param i32)) (func (export \"main\") i32.const 24 call $i))");
    }

    #[test]
    fn test_add_int() {
        let p = Program {
            statements: vec![Statement::Print(
                Expression::Add(
                    Value::Integer(1),
                    Box::new(
                        Expression::Simple(
                            Value::Integer(2)))
                )
            )]
        };
        let wexp = p.codegen();
        assert_eq!(wexp.to_string(),
        "(module \
        (func $i (import \"host\" \"print\") (param i32)) \
        (func (export \"main\") \
        i32.const 1 \
        i32.const 2 \
        i32.add \
        call $i\
        ))");

    }
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
