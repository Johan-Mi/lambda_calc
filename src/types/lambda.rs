use super::env::Env;
use super::symbol::Symbol;
use super::term::Term;
use derive_more::{Constructor, Display};
use std::rc::Rc;

#[derive(Constructor, Display, Debug, Clone)]
#[display(fmt = "(\\{var} . {body})")]
pub struct Lambda {
    var: Symbol,
    body: Rc<Term>,
}

impl Lambda {
    pub fn eval(&self, env: &mut Env) -> Rc<Term> {
        Rc::new(Term::Lambda(Self::new(
            self.var.clone(),
            self.body.eval(env),
        )))
    }

    pub fn apply(&self, arg: Rc<Term>, env: &mut Env) -> Rc<Term> {
        env.entry(self.var.clone()).or_default().push(arg);
        let ret = self.body.eval(env);
        env.entry(self.var.clone()).or_default().pop();
        ret
    }
}
