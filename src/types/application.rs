use super::env::Env;
use super::term::Term;
use derive_more::{Constructor, Display};
use std::rc::Rc;

#[derive(Constructor, Display, Debug, Clone)]
#[display(fmt = "({func} {arg})")]
pub struct Application {
    func: Rc<Term>,
    arg: Rc<Term>,
}

impl Application {
    pub fn eval(&self, env: &mut Env) -> Rc<Term> {
        let func = self.func.eval(env);
        let arg = self.arg.eval(env);
        match &*func {
            Term::Lambda(lambda) => lambda.apply(arg, env),
            _ => Rc::new(Term::Application(Self::new(func, arg))),
        }
    }
}
