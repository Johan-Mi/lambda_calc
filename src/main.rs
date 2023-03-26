mod env;
mod lexer;
mod parser;
mod term;

use lexer::Token;
use logos::Logos;
use parser::parse_expressions;
use std::collections::HashMap;

fn main() {
    let mut env = HashMap::new();

    let src = r"((\x f . (f x)) b (\n . n))";
    let lex = Token::lexer(src);
    let lexed = lex.collect::<Vec<_>>();
    let Some((parsed, _)) = parse_expressions(&lexed) else {
        eprintln!("Parser failed");
        return;
    };
    for i in parsed {
        println!("{i}");
        println!("=> {}", i.eval(&mut env));
    }
}
