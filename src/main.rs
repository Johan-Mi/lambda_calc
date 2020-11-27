mod types;

use std::collections::HashMap;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser);

fn main() {
    let mut env = HashMap::new();

    let src = r"((\x f . (f x)) b (\n . n))";
    let parsed = match parser::ObjectsParser::new().parse(src) {
        Ok(ok) => ok,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };
    for i in parsed {
        println!("{}", i);
        println!("=> {}", i.eval(&mut env));
    }
}
