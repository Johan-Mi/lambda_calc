use crate::env::Env;
use derive_more::Display;
use std::rc::Rc;

#[derive(Display, Debug)]
pub enum Term {
    Symbol(String),
    #[display(fmt = "({func} {arg})")]
    Application {
        func: Rc<Self>,
        arg: Rc<Self>,
    },
    #[display(fmt = "(\\{var} . {body})")]
    Lambda {
        var: String,
        body: Rc<Self>,
    },
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
