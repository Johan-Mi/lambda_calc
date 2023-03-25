use super::object::Object;
use super::symbol::Symbol;
use std::collections::HashMap;
use std::rc::Rc;

pub type Env = HashMap<Symbol, Vec<Rc<Object>>>;
