use super::env::Env;
use super::object::Object;
use derive_more::{Constructor, Display};
use std::rc::Rc;

#[derive(Constructor, Display, Debug, Clone)]
#[display(fmt = "({func} {arg})")]
pub struct Application {
    func: Rc<Object>,
    arg: Rc<Object>,
}

impl Application {
    pub fn eval(&self, env: &mut Env) -> Rc<Object> {
        let func = self.func.eval(env);
        let arg = self.arg.eval(env);
        match &*func {
            Object::Lambda(lambda) => lambda.apply(arg, env),
            _ => Rc::new(Object::Application(Self::new(func, arg))),
        }
    }
}
