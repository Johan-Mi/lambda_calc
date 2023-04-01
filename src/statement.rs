use crate::term::Term;

pub enum Statement {
    Evaluate(Term),
    Assign { var: String, value: Term },
}
