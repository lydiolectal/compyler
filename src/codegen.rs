use program::*;
use wexp::Wexp::{self, *};

macro_rules! wasm {
    ($i:ident) => {
        ::wexp::Wexp::Atom(stringify!($i).to_string())
    };
    ($s:expr) => {{
        let s: &str = $s;
        ::wexp::Wexp::Atom(s.to_string())
    }};
}

impl Program {
    pub fn codegen(&self) -> Wexp {
        let print = List(vec![
            wasm!(func),
            Atom("$i".to_string()),
            List(vec![wasm!(import), wasm!("\"host\""), wasm!("\"print\"")]),
            List(vec![wasm!(param), wasm!(i32)]),
        ]);
        let mut module = vec![wasm!(module), print];
        let mut main = vec![wasm!(func), List(vec![wasm!(export), wasm!("\"main\"")])];
        main.extend(self.body.codegen());
        // for some other def statements in self.statements
        // module.push(List(stmt))
        module.push(List(main));
        List(module)
    }
}

impl Body {
    pub fn codegen(&self) -> Vec<Wexp> {
        self.statements
            .iter()
            .flat_map(|stmt| stmt.codegen())
            .collect()
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
            _ => unimplemented!(),
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
                let val = v.codegen();
                let expr = e.codegen();
                atoms.extend(val);
                atoms.extend(expr);
                atoms.push(Atom("i32.add".to_owned()));
            }
            Expression::Sub(v, ref e) => {
                let val = v.codegen();
                let expr = e.codegen();
                atoms.extend(val);
                atoms.extend(expr);
                atoms.push(Atom("i32.sub".to_owned()));
            }
            _ => unimplemented!(),
        }
        atoms
    }
}

impl Value {
    pub fn codegen(&self) -> Vec<Wexp> {
        let mut atoms = vec![];
        match self {
            Value::Integer(i) => {
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
    use testing::*;

    macro_rules! codegen_test {
        (name: $name:ident,text: $text:expr,wat: $expected:expr,) => {
            #[test]
            fn $name() {
                let text = $text;
                let expected = $expected;
                let wexp = codegen(text);
                assert_eq!(wexp.to_string(), expected);
            }
        };
    }

    codegen_test! {
        name: empty_program,
        text: "",
        wat: "(module (func $i (import \"host\" \"print\") (param i32)) (func (export \"main\")))",
    }

    codegen_test! {
        name: print_int,
        text: "print 24",
        wat: "(module (func $i (import \"host\" \"print\") \
         (param i32)) (func (export \"main\") i32.const 24 call $i))",
    }

    codegen_test! {
        name: add_int,
        text: "print 1 + 2",
        wat: "(module \
         (func $i (import \"host\" \"print\") (param i32)) \
         (func (export \"main\") \
         i32.const 1 \
         i32.const 2 \
         i32.add \
         call $i\
         ))",
    }

    codegen_test! {
        name: sub_int,
        text: "print 2 - 1",
        wat: "(module \
         (func $i (import \"host\" \"print\") (param i32)) \
         (func (export \"main\") \
         i32.const 2 \
         i32.const 1 \
         i32.sub \
         call $i\
         ))",
    }

    codegen_test! {
        name: add_and_sub_int,
        text: "print 2 + 2 - 3",
        wat: "(module \
         (func $i (import \"host\" \"print\") (param i32)) \
         (func (export \"main\") \
         i32.const 2 \
         i32.const 2 \
         i32.const 3 \
         i32.sub \
         i32.add \
         call $i))",
    }

    codegen_test! {
        name: test_def,
        text: "def f():\n  print 8",
        wat: "(module \
            (func $i (import \"host\" \"print\") (param i32)) \
            (func $f \
            i32.const 8 \
            call $i) \
            (func (export \"main\") \
            ))",
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
