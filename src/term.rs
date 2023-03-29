use crate::env::Env;
use std::{fmt, rc::Rc};

#[derive(Debug)]
pub enum Term {
    Symbol(String),
    Application { func: Rc<Self>, arg: Rc<Self> },
    Lambda { var: String, body: Rc<Self> },
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Symbol(sym) => f.write_str(sym),
            Term::Application { func, arg } => write!(f, "({func} {arg})"),
            Term::Lambda { var, body } => write!(f, "(\\{var}. {body})"),
        }
    }
}

impl Term {
    pub fn eval(&self, env: &mut Env) -> Rc<Self> {
        match self {
            Self::Symbol(sym) => env
                .get(sym)
                .cloned()
                .unwrap_or_else(|| Rc::new(Self::Symbol(sym.clone()))),
            Self::Application { func, arg } => {
                let func = func.eval(env);
                let arg = arg.eval(env);
                match &*func {
                    Self::Lambda { var, body } => {
                        let old = env.insert(var.clone(), arg);
                        let ret = body.eval(env);
                        if let Some(old) = old {
                            env.insert(var.clone(), old);
                        }
                        ret
                    }
                    _ => Rc::new(Self::Application { func, arg }),
                }
            }
            Self::Lambda { var, body } => Rc::new(Self::Lambda {
                var: var.clone(),
                body: body.eval(env),
            }),
        }
    }
}
