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
        let mut module = vec![wasm!(module)];
        // for loop that adds statements to module vec
        module.push(List(vec![wasm!(func), List(vec![wasm!(export), wasm!("\"main\"")])]));
        List(module)
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
        assert_eq!(wexp.to_string(), "(module (func (export \"main\")))");
    }
}
