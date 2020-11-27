use super::object::*;
use super::symbol::*;
use std::collections::HashMap;
use std::rc::Rc;

pub type Env = HashMap<Symbol, Vec<Rc<Object>>>;
