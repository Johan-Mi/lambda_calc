use super::application::Application;
use super::env::Env;
use super::lambda::Lambda;
use super::symbol::Symbol;
use derive_more::Display;
use std::rc::Rc;

#[derive(Display, Debug)]
pub enum Object {
    Symbol(Symbol),
    Application(Application),
    Lambda(Lambda),
}

impl Object {
    pub fn eval(&self, env: &mut Env) -> Rc<Self> {
        match self {
            Self::Symbol(contained) => contained.eval(env),
            Self::Application(contained) => contained.eval(env),
            Self::Lambda(contained) => contained.eval(env),
        }
    }
}
