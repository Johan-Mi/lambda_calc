#![forbid(unsafe_code)]
#![warn(clippy::nursery, clippy::pedantic)]

mod env;
mod lexer;
mod parser;
mod statement;
mod term;

use lexer::Token;
use logos::Logos;
use std::collections::HashMap;

fn main() {
    let mut env = HashMap::new();
    let mut editor = rustyline::Editor::<(), _>::with_history(
        rustyline::Config::builder().auto_add_history(true).build(),
        rustyline::history::MemHistory::new(),
    )
    .unwrap();
    for line in editor.iter("> ").filter_map(Result::ok) {
        let lexer = Token::lexer(&line);
        let Ok(tokens) = lexer.collect::<Result<Vec<_>, _>>() else {
            println!("Syntax error!");
            continue;
        };
        match parser::statement(&tokens) {
            Some(statement::Statement::Evaluate(term)) => {
                println!("{}", term.eval(&mut env));
            }
            Some(statement::Statement::Assign { var, value }) => {
                let value = value.eval(&mut env);
                env.insert(var, value);
            }
            None => println!("Syntax error!"),
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
            let tokens = lexer.collect::<Result<Vec<_>, _>>().unwrap();
            let mut env = HashMap::new();
            assert_eq!(
                super::parser::term(&tokens)
                    .unwrap()
                    .0
                    .eval(&mut env)
                    .to_string(),
                expected
            );
        }

        case("a", "'a");
        case("long_name", "'long_name");
        case("x y z", "('x 'y 'z)");
        case(r"(\x. (\y. x))", r"(\x y. x)");
        case(r"((\x f . (f x)) b (\n . n))", "'b");
    }
}
