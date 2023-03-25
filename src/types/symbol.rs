use super::env::Env;
use super::object::Object;
use derive_more::{Constructor, Display};
use std::rc::Rc;

#[derive(Constructor, Display, PartialEq, Eq, Hash, Clone, Debug)]
pub struct Symbol {
    name: String,
}

impl Symbol {
    pub fn eval(&self, env: &Env) -> Rc<Object> {
        env.get(self)
            .and_then(|stack| stack.last().cloned())
            .unwrap_or_else(|| Rc::new(Object::Symbol(self.clone())))
    }
}
