use crate::term::Term;
use internment::Intern;

pub enum Statement {
    Evaluate(Term),
    Assign { var: Intern<str>, value: Term },
}
