use super::env::*;
use super::object::*;
use derive_more::{Constructor, Display};
use std::rc::Rc;

#[derive(Constructor, Display, PartialEq, Eq, Hash, Clone, Debug)]
pub struct Symbol {
    name: String,
}

impl Symbol {
    pub fn eval(&self, env: &Env) -> Rc<Object> {
        match env.get(self) {
            Some(stack) => match stack.last() {
                Some(val) => val.clone(),
                None => Rc::new(Object::Symbol(self.clone())),
            },
            None => Rc::new(Object::Symbol(self.clone())),
        }
    }
}
