use super::{lexer::Token, term::Term};
use std::rc::Rc;

fn parse_symbol(tokens: &[Token]) -> Option<(String, &[Token])> {
    match tokens.split_first() {
        Some((Token::Ident(symbol), tokens)) => {
            Some((symbol.to_string(), tokens))
        }
        _ => None,
    }
}

fn parse_lparen(tokens: &[Token]) -> Option<&[Token]> {
    tokens.strip_prefix(&[Token::LParen])
}

fn parse_rparen(tokens: &[Token]) -> Option<&[Token]> {
    tokens.strip_prefix(&[Token::RParen])
}

fn parse_dot(tokens: &[Token]) -> Option<&[Token]> {
    tokens.strip_prefix(&[Token::Dot])
}

fn parse_backslash(tokens: &[Token]) -> Option<&[Token]> {
    tokens.strip_prefix(&[Token::Backslash])
}

fn parse_lambda(tokens: &[Token]) -> Option<(Term, &[Token])> {
    fn parse_lambda_internal(tokens: &[Token]) -> Option<(Term, &[Token])> {
        let (var, tokens) = parse_symbol(tokens)?;
        if let Some(tokens) = parse_dot(tokens) {
            let (body, tokens) = parse_expression(tokens)?;
            Some((
                Term::Lambda {
                    var,
                    body: Rc::new(body),
                },
                tokens,
            ))
        } else {
            let (body, tokens) = parse_lambda_internal(tokens)?;
            Some((
                Term::Lambda {
                    var,
                    body: Rc::new(body),
                },
                tokens,
            ))
        }
    }

    let tokens = parse_lparen(tokens)?;
    let tokens = parse_backslash(tokens)?;
    let (lambda, tokens) = parse_lambda_internal(tokens)?;
    let tokens = parse_rparen(tokens)?;
    Some((lambda, tokens))
}

fn parse_application(tokens: &[Token]) -> Option<(Term, &[Token])> {
    let tokens = parse_lparen(tokens)?;
    let (func, tokens) = parse_expression(tokens)?;
    let (first_arg, mut tokens) = parse_expression(tokens)?;
    let mut application = Term::Application {
        func: Rc::new(func),
        arg: Rc::new(first_arg),
    };
    while let Some((arg, remaining_tokens)) = parse_expression(tokens) {
        tokens = remaining_tokens;
        application = Term::Application {
            func: Rc::new(application),
            arg: Rc::new(arg),
        };
    }
    let tokens = parse_rparen(tokens)?;
    Some((application, tokens))
}

pub fn parse_expression(tokens: &[Token]) -> Option<(Term, &[Token])> {
    if let Some((expr, tokens)) = parse_lambda(tokens) {
        Some((expr, tokens))
    } else if let Some((expr, tokens)) = parse_application(tokens) {
        Some((expr, tokens))
    } else if let Some((sym, tokens)) = parse_symbol(tokens) {
        Some((Term::Symbol(sym), tokens))
    } else {
        None
    }
}
