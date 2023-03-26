use super::lexer::Token;
use super::term::Term;
use std::rc::Rc;

fn parse_symbol(tokens: &[Token]) -> Option<(String, &[Token])> {
    match tokens.split_first() {
        Some((Token::Ident(symbol), tokens)) => {
            Some((symbol.to_string(), tokens))
        }
        _ => None,
    }
}

const fn parse_lparen(tokens: &[Token]) -> Option<&[Token]> {
    match tokens.split_first() {
        Some((Token::LParen, tokens)) => Some(tokens),
        _ => None,
    }
}

const fn parse_rparen(tokens: &[Token]) -> Option<&[Token]> {
    match tokens.split_first() {
        Some((Token::RParen, tokens)) => Some(tokens),
        _ => None,
    }
}

const fn parse_dot(tokens: &[Token]) -> Option<&[Token]> {
    match tokens.split_first() {
        Some((Token::Dot, tokens)) => Some(tokens),
        _ => None,
    }
}

const fn parse_backslash(tokens: &[Token]) -> Option<&[Token]> {
    match tokens.split_first() {
        Some((Token::Backslash, tokens)) => Some(tokens),
        _ => None,
    }
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

fn parse_expression(tokens: &[Token]) -> Option<(Term, &[Token])> {
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

pub fn parse_expressions(
    mut tokens: &[Token],
) -> Option<(Vec<Term>, &[Token])> {
    let mut ret = Vec::new();

    while let Some((expr, remaining_tokens)) = parse_expression(tokens) {
        ret.push(expr);
        tokens = remaining_tokens;
    }

    if ret.is_empty() {
        None
    } else {
        Some((ret, tokens))
    }
}
