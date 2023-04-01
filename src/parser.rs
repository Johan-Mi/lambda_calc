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

pub fn parenthesized(tokens: &[Token]) -> Option<(Term, &[Token])> {
    let tokens = lparen(tokens)?;
    let (inner, tokens) = term(tokens)?;
    let tokens = rparen(tokens)?;
    Some((inner, tokens))
}

pub fn atom(tokens: &[Token]) -> Option<(Term, &[Token])> {
    if let Some((expr, tokens)) = lambda(tokens) {
        Some((expr, tokens))
    } else if let Some((expr, tokens)) = parenthesized(tokens) {
        Some((expr, tokens))
    } else if let Some((sym, tokens)) = symbol(tokens) {
        Some((Term::Symbol(sym), tokens))
    } else {
        None
    }
}

pub fn term(tokens: &[Token]) -> Option<(Term, &[Token])> {
    let (mut term, mut tokens) = atom(tokens)?;
    while let Some((arg, remaining_tokens)) = atom(tokens) {
        tokens = remaining_tokens;
        term = Term::Application {
            func: Rc::new(term),
            arg: Rc::new(arg),
        };
    }
    Some((term, tokens))
}
