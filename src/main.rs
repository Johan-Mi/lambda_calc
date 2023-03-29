mod env;
mod lexer;
mod parser;
mod term;

use lexer::Token;
use logos::Logos;
use std::{collections::HashMap, io::Write};

fn main() {
    let mut env = HashMap::new();

    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let lexer = Token::lexer(&input);
        let tokens = lexer.collect::<Vec<_>>();
        match parser::parse_expression(&tokens) {
            Some((parsed, &[])) => println!("{}", parsed.eval(&mut env)),
            _ => println!("Syntax error!"),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        #[track_caller]
        fn case(input: &str, expected: &str) {
            use logos::Logos;
            use std::collections::HashMap;

            let lexer = super::lexer::Token::lexer(input);
            let tokens = lexer.collect::<Vec<_>>();
            let mut env = HashMap::new();
            assert_eq!(
                super::parser::parse_expression(&tokens)
                    .unwrap()
                    .0
                    .eval(&mut env)
                    .to_string(),
                expected
            );
        }

        case("a", "a");
        case("long_name", "long_name");
        case(r"(\x. (\y. x))", r"(\x y. x)");
        case(r"((\x f . (f x)) b (\n . n))", "b");
    }
}
