use super::symbol::Symbol;
use super::term::Term;
use std::collections::HashMap;
use std::rc::Rc;

pub type Env = HashMap<Symbol, Vec<Rc<Term>>>;
