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
            Atom("$print".to_string()),
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
            let mut n = Self::prepend_dollar(name);
            def_wexp.push(Atom(n));
            for param in params.iter() {
                let mut p = Self::prepend_dollar(param);
                let mut param_wexp = vec![wasm!("param")];
                param_wexp.push(Atom(p));
                param_wexp.push(wasm!(i32));
                def_wexp.push(List(param_wexp));
            }
            // TODO: don't assume that all functions return integers.
            let return_type = List(vec![wasm!("result"), wasm!("i32")]);
            def_wexp.push(return_type);
            // TODO: resolve this.
            // this disallows functions inside functions I suppose, since
            // codegen_body skips function definitions.
            let b = self.codegen_body(body);
            def_wexp.extend(b);
        }
        List(def_wexp)
    }

    pub fn codegen_statement(&self, stmt: &Statement) -> Vec<Wexp> {
        let mut atoms = vec![];
        match stmt {
            Statement::Print(e) => {
                let expr = self.codegen_expression(e);
                atoms.extend(expr);
                atoms.extend(vec![wasm!(call), Atom("$print".to_owned())]);
            }
            Statement::Return(e) => {
                let expr = self.codegen_expression(e);
                atoms.extend(expr);
            }
            Statement::If { .. } => {
                let if_wexp = self.codegen_if(stmt);
                atoms.extend(if_wexp);
            }
            _ => unimplemented!(),
        }
        atoms
    }

    pub fn codegen_if(&self, stmt: &Statement) -> Vec<Wexp> {
        let mut if_wexp = Vec::new();
        if let Statement::If {
            condition,
            body,
            elif,
            else_body,
        } = stmt
        {
            let cond_wexp = self.codegen_expression(condition);
            if_wexp.extend(cond_wexp);
            if_wexp.push(wasm!("if"));
            // TODO: don't assume that all if/else return integers.
            let return_type = List(vec![wasm!("result"), wasm!("i32")]);
            if_wexp.push(return_type);
            let body_wexp = self.codegen_body(body);
            if_wexp.extend(body_wexp);
            if !elif.is_empty() {
                let mut elif_clone = elif.clone();
                let else_clone = else_body.clone();
                let (elif_condition, elif_body) = elif_clone.remove(0);
                let elif_stmt = Statement::If {
                    condition: elif_condition,
                    body: elif_body,
                    elif: elif_clone,
                    else_body: else_clone,
                };
                let mut elif_wexp = vec![wasm!("else")];
                elif_wexp.extend(self.codegen_if(&elif_stmt));
                if_wexp.extend(elif_wexp);
            } else {
                if let Some(b) = else_body {
                    if_wexp.push(wasm!("else"));
                    let else_wexp = self.codegen_body(b);
                    if_wexp.extend(else_wexp);
                }
            }
            if_wexp.push(wasm!("end"));
        }
        if_wexp
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
            Expression::Mult(ref v, ref e) => {
                let expr_l = self.codegen_expression(v);
                let expr_r = self.codegen_expression(e);
                atoms.extend(expr_l);
                atoms.extend(expr_r);
                atoms.push(Atom("i32.mul".to_owned()));
            }
            Expression::Div(ref v, ref e) => {
                let expr_l = self.codegen_expression(v);
                let expr_r = self.codegen_expression(e);
                atoms.extend(expr_l);
                atoms.extend(expr_r);
                atoms.push(Atom("i32.div_s".to_owned()));
            }
            Expression::Mod(ref v, ref e) => {
                let expr_l = self.codegen_expression(v);
                let expr_r = self.codegen_expression(e);
                atoms.extend(expr_l);
                atoms.extend(expr_r);
                atoms.push(Atom("i32.rem_s".to_owned()));
            }
            Expression::Lt(ref v, ref e) => {
                let expr_l = self.codegen_expression(v);
                let expr_r = self.codegen_expression(e);
                atoms.extend(expr_l);
                atoms.extend(expr_r);
                atoms.push(Atom("i32.lt_s".to_owned()));
            }
            Expression::Gt(ref v, ref e) => {
                let expr_l = self.codegen_expression(v);
                let expr_r = self.codegen_expression(e);
                atoms.extend(expr_l);
                atoms.extend(expr_r);
                atoms.push(Atom("i32.gt_s".to_owned()));
            }
            Expression::Leq(ref v, ref e) => {
                let expr_l = self.codegen_expression(v);
                let expr_r = self.codegen_expression(e);
                atoms.extend(expr_l);
                atoms.extend(expr_r);
                atoms.push(Atom("i32.le_s".to_owned()));
            }
            Expression::Geq(ref v, ref e) => {
                let expr_l = self.codegen_expression(v);
                let expr_r = self.codegen_expression(e);
                atoms.extend(expr_l);
                atoms.extend(expr_r);
                atoms.push(Atom("i32.ge_s".to_owned()));
            }
            Expression::EqEq(ref v, ref e) => {
                let expr_l = self.codegen_expression(v);
                let expr_r = self.codegen_expression(e);
                atoms.extend(expr_l);
                atoms.extend(expr_r);
                atoms.push(Atom("i32.eq".to_owned()));
            }
            Expression::Ne(ref v, ref e) => {
                let expr_l = self.codegen_expression(v);
                let expr_r = self.codegen_expression(e);
                atoms.extend(expr_l);
                atoms.extend(expr_r);
                atoms.push(Atom("i32.ne".to_owned()));
            }
            Expression::And(ref v, ref e) => {
                let expr_l = self.codegen_expression(v);
                let expr_r = self.codegen_expression(e);
                atoms.extend(expr_l);
                atoms.extend(expr_r);
                atoms.push(Atom("i32.and".to_owned()));
            }
            Expression::Or(ref v, ref e) => {
                let expr_l = self.codegen_expression(v);
                let expr_r = self.codegen_expression(e);
                atoms.extend(expr_l);
                atoms.extend(expr_r);
                atoms.push(Atom("i32.or".to_owned()));
            }
            Expression::Call { name, params } => {
                for param in params {
                    atoms.extend(self.codegen_expression(param));
                }
                atoms.push(wasm!("call"));
                atoms.push(wasm!(&Self::prepend_dollar(name)));
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
                let value = Self::prepend_dollar(v);
                atoms.push(Atom(value));
            }
            _ => {
                unimplemented!();
            }
        }
        atoms
    }

    fn prepend_dollar(name: &str) -> String {
        let mut s = String::from("$");
        s.push_str(name);
        s
    }
}

#[cfg(test)]
mod test {
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
        wat: "(module (func $print (import \"host\" \"print\") (param i32)) \
        (func (export \"main\")))",
    }

    codegen_test! {
        name: print_int,
        text: "print 24",
        wat: "(module (func $print (import \"host\" \"print\") \
         (param i32)) (func (export \"main\") i32.const 24 call $print))",
    }

    codegen_test! {
        name: print_leq,
        text: "print 8 <= 8",
        wat: "(module (func $print (import \"host\" \"print\") \
         (param i32)) (func (export \"main\") i32.const 8 i32.const 8 \
         i32.le_s call $print))",
    }

    codegen_test! {
        name: print_ne,
        text: "print 8 != 8",
        wat: "(module (func $print (import \"host\" \"print\") \
         (param i32)) (func (export \"main\") i32.const 8 i32.const 8 \
         i32.ne call $print))",
    }

    codegen_test! {
        name: add_int,
        text: "print 1 + 2",
        wat: "(module \
         (func $print (import \"host\" \"print\") (param i32)) \
         (func (export \"main\") \
         i32.const 1 \
         i32.const 2 \
         i32.add \
         call $print\
         ))",
    }

    codegen_test! {
        name: sub_int,
        text: "print 2 - 1",
        wat: "(module \
         (func $print (import \"host\" \"print\") (param i32)) \
         (func (export \"main\") \
         i32.const 2 \
         i32.const 1 \
         i32.sub \
         call $print\
         ))",
    }

    codegen_test! {
        name: div_int,
        text: "print 9 / 3",
        wat: "(module \
         (func $print (import \"host\" \"print\") (param i32)) \
         (func (export \"main\") \
         i32.const 9 \
         i32.const 3 \
         i32.div_s \
         call $print\
         ))",
    }

    codegen_test! {
        name: mod_int,
        text: "print 13 % 7",
        wat: "(module \
         (func $print (import \"host\" \"print\") (param i32)) \
         (func (export \"main\") \
         i32.const 13 \
         i32.const 7 \
         i32.rem_s \
         call $print\
         ))",
    }

    codegen_test! {
        name: print_and,
        text: "print 1>=2 and 2<7",
        wat: "(module \
         (func $print (import \"host\" \"print\") (param i32)) \
         (func (export \"main\") \
         i32.const 1 \
         i32.const 2 \
         i32.ge_s \
         i32.const 2 \
         i32.const 7 \
         i32.lt_s \
         i32.and \
         call $print\
         ))",
    }

    codegen_test! {
        name: add_and_sub_int,
        text: "print 2 + 2 - 3",
        wat: "(module \
         (func $print (import \"host\" \"print\") (param i32)) \
         (func (export \"main\") \
         i32.const 2 \
         i32.const 2 \
         i32.const 3 \
         i32.sub \
         i32.add \
         call $print))",
    }

    codegen_test! {
        name: codegen_def,
        text: "def f():\n  return 8",
        wat: "(module \
            (func $print (import \"host\" \"print\") (param i32)) \
            (func $f \
            (result i32) \
            i32.const 8) \
            (func (export \"main\")\
            ))",
    }

    codegen_test! {
        name: def_param,
        text: "def f(n):\n  return n",
        wat: "(module \
            (func $print (import \"host\" \"print\") (param i32)) \
            (func $f \
            (param $n i32) \
            (result i32) \
            get_local $n) \
            (func (export \"main\")\
            ))",
    }

    codegen_test! {
        name: def_params,
        text: "def f(m, n, o, p):\n  return p",
        wat: "(module \
            (func $print (import \"host\" \"print\") (param i32)) \
            (func $f \
            (param $m i32) \
            (param $n i32) \
            (param $o i32) \
            (param $p i32) \
            (result i32) \
            get_local $p) \
            (func (export \"main\")\
            ))",
    }

    codegen_test! {
        name: function_call,
        text: "def f(a, b):\n  return a + b\nprint f(2, 3)",
        wat: "(module \
            (func $print (import \"host\" \"print\") (param i32)) \
            (func $f (param $a i32) (param $b i32) (result i32) \
            get_local $a \
            get_local $b \
            i32.add) \
            (func (export \"main\") \
            i32.const 2 \
            i32.const 3 \
            call $f \
            call $print))",
    }

    codegen_test! {
        name: if_else,
        text: "def f(a):\n  if a < 5:\n    return 0\n  else:\n    return 1\nprint f(1)",
        wat: "(module \
            (func $print (import \"host\" \"print\") (param i32)) \
            (func $f (param $a i32) (result i32) \
            get_local $a \
            i32.const 5 \
            i32.lt_s \
            if (result i32) \
            i32.const 0 \
            else \
            i32.const 1 \
            end) \
            (func (export \"main\") \
            i32.const 1 \
            call $f \
            call $print))",
    }

    codegen_test! {
        name: elif,
        text: "def f(n):\n if n < 5:\n  return 0\n elif n < 10:\n  return 1\n else:\
        \n  return 2\nprint f(4)\nprint f(8)\nprint f(11)",
        wat: "(module \
            (func $print (import \"host\" \"print\") (param i32)) \
            (func $f (param $n i32) (result i32) \
                get_local $n \
                i32.const 5 \
                i32.lt_s \
                if (result i32) \
                    i32.const 0 \
                else \
                    get_local $n \
                    i32.const 10 \
                    i32.lt_s \
                    if (result i32) \
                        i32.const 1 \
                    else \
                        i32.const 2 \
                    end \
                end) \
            (func (export \"main\") \
                i32.const 4 \
                call $f \
                call $print \
                i32.const 8 \
                call $f \
                call $print \
                i32.const 11 \
                call $f \
                call $print))",
    }

    codegen_test! {
        name: elif_multiple,
        text: "def f(n):\n if n < 5:\n  return 0\n elif n < 10:\n  return 1\n elif n < 15:\
        \n  return 2  \n else:\n  return 3\nprint f(4)\nprint f(8)\nprint f(81)",
        wat: "(module \
            (func $print (import \"host\" \"print\") (param i32)) \
            (func $f (param $n i32) (result i32) \
                get_local $n \
                i32.const 5 \
                i32.lt_s \
                if (result i32) \
                    i32.const 0 \
                else \
                    get_local $n \
                    i32.const 10 \
                    i32.lt_s \
                    if (result i32) \
                        i32.const 1 \
                    else \
                        get_local $n \
                        i32.const 15 \
                        i32.lt_s \
                        if (result i32) \
                            i32.const 2 \
                        else \
                            i32.const 3 \
                        end \
                    end \
                end) \
            (func (export \"main\") \
                i32.const 4 \
                call $f \
                call $print \
                i32.const 8 \
                call $f \
                call $print \
                i32.const 81 \
                call $f \
                call $print))",
    }

    codegen_test! {
        name: fib,
        text: "def fib(n):\n  if n < 2:\n    return n\n  else:\n    return fib(n - 2) + fib(n - 1)\
        \nprint fib(4)",
        wat: "(module \
            (func $print (import \"host\" \"print\") (param i32)) \
            (func $fib (param $n i32) (result i32) \
            get_local $n \
            i32.const 2 \
            i32.lt_s \
            if (result i32) \
            get_local $n \
            else \
            get_local $n \
            i32.const 2 \
            i32.sub \
            call $fib \
            get_local $n \
            i32.const 1 \
            i32.sub \
            call $fib \
            i32.add \
            end) \
            (func (export \"main\") \
            i32.const 4 \
            call $fib \
            call $print))",
    }
}
