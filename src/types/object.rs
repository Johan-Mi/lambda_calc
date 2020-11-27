use super::application::*;
use super::env::*;
use super::lambda::*;
use super::symbol::*;
use derive_more::Display;
use std::rc::Rc;

#[derive(Display, Debug)]
pub enum Object {
    Symbol(Symbol),
    Application(Application),
    Lambda(Lambda),
}

impl Object {
    pub fn eval(&self, env: &mut Env) -> Rc<Object> {
        match self {
            Object::Symbol(contained) => contained.eval(env),
            Object::Application(contained) => contained.eval(env),
            Object::Lambda(contained) => contained.eval(env),
        }
    }
}
