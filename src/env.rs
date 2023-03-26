use crate::term::Term;
use std::collections::HashMap;
use std::rc::Rc;

pub type Env = HashMap<String, Vec<Rc<Term>>>;
