use crate::term::Term;
use std::{collections::HashMap, rc::Rc};

pub type Env = HashMap<String, Vec<Rc<Term>>>;
