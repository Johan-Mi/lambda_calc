use crate::term::Term;
use internment::Intern;
use std::{collections::HashMap, rc::Rc};

pub type Env = HashMap<Intern<str>, Rc<Term>>;
