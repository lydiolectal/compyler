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
        module.extend(self.codegen_defs(&self.program.body));
        let mut main = vec![wasm!(func), List(vec![wasm!(export), wasm!("\"main\"")])];
        main.extend(self.codegen_body(&self.program.body));
        module.push(List(main));
        List(module)
    }

    pub fn codegen_defs(&self, body: &Body) -> Vec<Wexp> {
        body.statements
            .iter()
            .filter_map(|stmt| {
                if let Statement::Def { .. } = stmt {
                    Some(self.codegen_def(stmt))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn codegen_body(&self, body: &Body) -> Vec<Wexp> {
        body.statements
            .iter()
            .filter(|stmt| {
                if let Statement::Def { .. } = stmt {
                    false
                } else {
                    true
                }
            })
            .flat_map(|stmt| self.codegen_statement(stmt))
            .collect()
    }

    pub fn codegen_def(&self, stmt: &Statement) -> Wexp {
        let mut def_wexp: Vec<Wexp> = vec![wasm!("func")];
        // TODO: is there a better way to destructure Def variant?
        if let Statement::Def { name, params, body } = stmt {
            let mut n = Self::prepend_dollar(name.to_owned());
            def_wexp.push(Atom(n));
            for param in params.iter() {
                let mut p = Self::prepend_dollar(param.to_owned());
                let mut param_wexp = vec![wasm!("param")];
                param_wexp.push(Atom(p));
                param_wexp.push(wasm!(i32));
                def_wexp.push(List(param_wexp));
            }
            // TODO: resolve this.
            // this disallows functions inside functions I suppose, since
            // codegen_body skips function definitions.
            let b = self.codegen_body(body);
            def_wexp.extend(b);
        } else {
        }
        List(def_wexp)
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
                atoms.push(Atom("get_local".to_owned()));
                let value = Self::prepend_dollar(v.to_owned());
                atoms.push(Atom(value));
                //
                // Atom(v.to_string());
            }
        }
        atoms
    }

    // TODO: change this from taking String to &str
    fn prepend_dollar(name: String) -> String {
        let mut s = String::from("$");
        s.push_str(&name);
        s
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

    codegen_test! {
        name: test_def,
        text: "def f():\n  print 8",
        wat: "(module \
            (func $i (import \"host\" \"print\") (param i32)) \
            (func $f \
            i32.const 8 \
            call $i) \
            (func (export \"main\")\
            ))",
    }

    // delete and burn this
    #[test]
    fn test_def_no_params() {
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

    #[test]
    fn test_def_params() {
        let text = "def f(n):\n  print n";
        let program = parse(text).unwrap();
        let def = &program.body.statements[0];
        let codegenerator = CodeGenerator::new(program.clone());
        assert_eq!(
            codegenerator.codegen_def(def).to_string(),
            "(func $f \
             (param $n i32) \
             get_local $n \
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
