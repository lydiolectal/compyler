use program::*;
use wexp::Wexp::{self, *};

pub struct CodeGenerator {
    program: Program,
    // functions, etc.
}

macro_rules! wasm {
    ($i:ident) => {
        ::wexp::Wexp::Atom(stringify!($i).to_string())
    };
    ($s:expr) => {{
        let s: &str = $s;
        ::wexp::Wexp::Atom(s.to_string())
    }};
}

impl CodeGenerator {
    pub fn new(program: Program) -> CodeGenerator {
        CodeGenerator { program }
    }

    pub fn codegen(mut self) -> Wexp {
        let print = List(vec![
            wasm!(func),
            Atom("$i".to_string()),
            List(vec![wasm!(import), wasm!("\"host\""), wasm!("\"print\"")]),
            List(vec![wasm!(param), wasm!(i32)]),
        ]);
        let mut module = vec![wasm!(module), print];
        let mut main = vec![wasm!(func), List(vec![wasm!(export), wasm!("\"main\"")])];
        // self.functions.push(self.codegen_def(self.program.body));
        // for function in self.functions {
        //     module.push function;
        // }
        main.extend(self.codegen_body(&self.program.body));
        // for some other def statements in self.statements
        // module.push(List(stmt))
        module.push(List(main));
        List(module)
    }

    pub fn codegen_def(&self, stmt: &Statement) -> Wexp {
        let mut def_wexp: Vec<Wexp> = Vec::new();
        if let Statement::Def { name, params, body } = stmt {
            let mut n = String::from("$");
            n.push_str(name);
            def_wexp.push(wasm!("func"));
            def_wexp.push(Atom(n));
            for p in params.iter() {
                unimplemented!();
            }
            let b = self.codegen_body(body);
            def_wexp.extend(b);
            List(def_wexp)
        } else {
            wasm!("hi!")
        }
    }

    pub fn codegen_body(&self, body: &Body) -> Vec<Wexp> {
        body.statements
            .iter()
            .flat_map(|stmt| self.codegen_statement(stmt))
            .collect()
    }

    pub fn codegen_statement(&self, stmt: &Statement) -> Vec<Wexp> {
        let mut atoms = vec![];
        match stmt {
            Statement::Print(e) => {
                let expr = self.codegen_expression(e);
                atoms.extend(expr);
                atoms.extend(vec![wasm!(call), Atom("$i".to_owned())]);
            }
            _ => unimplemented!(),
        }
        atoms
    }

    pub fn codegen_expression(&self, expr: &Expression) -> Vec<Wexp> {
        let mut atoms = vec![];
        match expr {
            Expression::Simple(v) => {
                let val = self.codegen_value(v);
                atoms.extend(val);
            }
            Expression::Add(ref v, ref e) => {
                let expr_l = self.codegen_expression(v);
                let expr_r = self.codegen_expression(e);
                atoms.extend(expr_l);
                atoms.extend(expr_r);
                atoms.push(Atom("i32.add".to_owned()));
            }
            Expression::Sub(ref v, ref e) => {
                let expr_l = self.codegen_expression(v);
                let expr_r = self.codegen_expression(e);
                atoms.extend(expr_l);
                atoms.extend(expr_r);
                atoms.push(Atom("i32.sub".to_owned()));
            }
            _ => unimplemented!(),
        }
        atoms
    }

    pub fn codegen_value(&self, value: &Value) -> Vec<Wexp> {
        let mut atoms = vec![];
        match value {
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
    use testing::*;
    // delete and burn this
    use super::*;

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

    // codegen_test! {
    //     name: test_def,
    //     text: "def f():\n  print 8",
    //     wat: "(module \
    //         (func $i (import \"host\" \"print\") (param i32)) \
    //         (func $f \
    //         i32.const 8 \
    //         call $i) \
    //         (func (export \"main\") \
    //         ))",
    // }

    // delete and burn this
    #[test]
    fn test_def() {
        let text = "def f():\n  print 8";
        let program = parse(text).unwrap();
        let def = &program.body.statements[0];
        let codegenerator = CodeGenerator::new(program.clone());
        assert_eq!(
            codegenerator.codegen_def(def).to_string(),
            "(func $f \
             i32.const 8 \
             call $i)"
        );
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
