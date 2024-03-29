use crate::env::Env;
use internment::Intern;
use std::{
    fmt::{self, Write},
    rc::Rc,
};

pub enum Term {
    Var(Intern<str>),
    Free(Intern<str>),
    Application { func: Rc<Self>, arg: Rc<Self> },
    Lambda { var: Intern<str>, body: Rc<Self> },
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn display(
            term: &Term,
            is_application_head: bool,
            is_lambda_body: bool,
            f: &mut fmt::Formatter,
        ) -> fmt::Result {
            if is_lambda_body && !matches!(term, Term::Lambda { .. }) {
                f.write_str(". ")?;
            }

            match term {
                Term::Var(sym) => f.write_str(sym),
                Term::Free(sym) => write!(f, "'{sym}"),
                Term::Application { func, arg } => {
                    if !is_application_head {
                        f.write_char('(')?;
                    }
                    display(func, true, false, f)?;
                    f.write_char(' ')?;
                    display(arg, false, false, f)?;
                    if !is_application_head {
                        f.write_char(')')?;
                    }
                    Ok(())
                }
                Term::Lambda { var, body } => {
                    f.write_str(if is_lambda_body { " " } else { "(\\" })?;
                    f.write_str(var)?;
                    display(body, false, true, f)?;
                    if !is_lambda_body {
                        f.write_char(')')?;
                    }
                    Ok(())
                }
            }
        }

        display(self, false, false, f)
    }
}

impl Term {
    pub fn eval(&self, env: &mut Env) -> Rc<Self> {
        match self {
            Self::Var(sym) => env
                .get(sym)
                .cloned()
                .unwrap_or_else(|| Rc::new(Self::Free(*sym))),
            Self::Free(sym) => Rc::new(Self::Free(*sym)),
            Self::Application { func, arg } => {
                let func = func.eval(env);
                let arg = arg.eval(env);
                match &*func {
                    Self::Lambda { var, body } => {
                        let old = env.insert(*var, arg);
                        let ret = body.eval(env);
                        if let Some(old) = old {
                            env.insert(*var, old);
                        } else {
                            env.remove(var);
                        }
                        ret
                    }
                    _ => Rc::new(Self::Application { func, arg }),
                }
            }
            Self::Lambda { var, body } => {
                let old = env.insert(*var, Rc::new(Self::Var(*var)));
                let ret = Rc::new(Self::Lambda {
                    var: *var,
                    body: body.eval(env),
                });
                if let Some(old) = old {
                    env.insert(*var, old);
                } else {
                    env.remove(var);
                }
                ret
            }
        }
    }
}
