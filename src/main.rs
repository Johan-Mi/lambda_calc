mod lexer;
mod parser;
mod types;

use lexer::*;
use logos::Logos;
use parser::*;
use std::collections::HashMap;

fn main() {
    let mut env = HashMap::new();

    let src = r"((\x f . (f x)) b (\n . n))";
    let lex = Token::lexer(src);
    let lexed = lex.collect::<Vec<_>>();
    let parsed = match parse_expressions(&lexed) {
        Some((ok, _)) => ok,
        None => {
            eprintln!("Parser failed");
            return;
        }
    };
    for i in parsed {
        println!("{}", i);
        println!("=> {}", i.eval(&mut env));
    }
}
