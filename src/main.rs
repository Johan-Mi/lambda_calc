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
