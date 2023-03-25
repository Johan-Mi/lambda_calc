use super::env::Env;
use super::object::Object;
use super::symbol::Symbol;
use derive_more::{Constructor, Display};
use std::rc::Rc;

#[derive(Constructor, Display, Debug, Clone)]
#[display(fmt = "(\\{var} . {body})")]
pub struct Lambda {
    var: Symbol,
    body: Rc<Object>,
}

impl Lambda {
    pub fn eval(&self, env: &mut Env) -> Rc<Object> {
        Rc::new(Object::Lambda(Self::new(
            self.var.clone(),
            self.body.eval(env),
        )))
    }

    pub fn apply(&self, arg: Rc<Object>, env: &mut Env) -> Rc<Object> {
        env.entry(self.var.clone()).or_default().push(arg);
        let ret = self.body.eval(env);
        env.entry(self.var.clone()).or_default().pop();
        ret
    }
}
