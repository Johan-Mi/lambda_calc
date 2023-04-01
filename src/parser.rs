use super::{lexer::Token, term::Term};
use std::rc::Rc;

fn symbol(tokens: &[Token]) -> Option<(String, &[Token])> {
    if let [Token::Ident(symbol), tokens @ ..] = tokens {
        Some((symbol.to_string(), tokens))
    } else {
        None
    }
}

fn lparen(tokens: &[Token]) -> Option<&[Token]> {
    tokens.strip_prefix(&[Token::LParen])
}

fn rparen(tokens: &[Token]) -> Option<&[Token]> {
    tokens.strip_prefix(&[Token::RParen])
}

fn dot(tokens: &[Token]) -> Option<&[Token]> {
    tokens.strip_prefix(&[Token::Dot])
}

fn backslash(tokens: &[Token]) -> Option<&[Token]> {
    tokens.strip_prefix(&[Token::Backslash])
}

fn lambda(tokens: &[Token]) -> Option<(Term, &[Token])> {
    fn lambda_internal(tokens: &[Token]) -> Option<(Term, &[Token])> {
        let (var, tokens) = symbol(tokens)?;
        if let Some(tokens) = dot(tokens) {
            let (body, tokens) = term(tokens)?;
            Some((
                Term::Lambda {
                    var,
                    body: Rc::new(body),
                },
                tokens,
            ))
        } else {
            let (body, tokens) = lambda_internal(tokens)?;
            Some((
                Term::Lambda {
                    var,
                    body: Rc::new(body),
                },
                tokens,
            ))
        }
    }

    let tokens = lparen(tokens)?;
    let tokens = backslash(tokens)?;
    let (lambda, tokens) = lambda_internal(tokens)?;
    let tokens = rparen(tokens)?;
    Some((lambda, tokens))
}

fn application(tokens: &[Token]) -> Option<(Term, &[Token])> {
    let tokens = lparen(tokens)?;
    let (func, tokens) = term(tokens)?;
    let (first_arg, mut tokens) = term(tokens)?;
    let mut application = Term::Application {
        func: Rc::new(func),
        arg: Rc::new(first_arg),
    };
    while let Some((arg, remaining_tokens)) = term(tokens) {
        tokens = remaining_tokens;
        application = Term::Application {
            func: Rc::new(application),
            arg: Rc::new(arg),
        };
    }
    let tokens = rparen(tokens)?;
    Some((application, tokens))
}

pub fn term(tokens: &[Token]) -> Option<(Term, &[Token])> {
    if let Some((expr, tokens)) = lambda(tokens) {
        Some((expr, tokens))
    } else if let Some((expr, tokens)) = application(tokens) {
        Some((expr, tokens))
    } else if let Some((sym, tokens)) = symbol(tokens) {
        Some((Term::Symbol(sym), tokens))
    } else {
        None
    }
}
