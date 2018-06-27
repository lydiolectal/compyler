use std::fmt::{self, Display, Formatter};

pub enum Wexp {
    List(Vec<Wexp>),
    Atom(String),
}

impl Display for Wexp {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use self::Wexp::*;
        match self {
            List(contents) => {
                write!(f, "(")?;
                for (i, wexp) in contents.iter().enumerate() {
                    write!(f, "{}", wexp)?;
                    if i < contents.len() - 1 {
                        write!(f, " ")?;
                    }
                }
                write!(f, ")")
            }
            Atom(value) => write!(f, "{}", value),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn display_atom() {
        let w = Wexp::Atom("hallo".to_owned());
        assert_eq!(w.to_string(), "hallo");
    }

    #[test]
    fn display_empty_vec() {
        let w = Wexp::List(vec![]);
        assert_eq!(w.to_string(), "()");
    }

    #[test]
    fn display_list() {
        let w = Wexp::List(vec![
            Wexp::Atom("hmm".to_owned()),
            Wexp::List(vec![Wexp::Atom("??!".to_owned())]),
        ]);
        assert_eq!(w.to_string(), "(hmm (??!))");
    }
}
