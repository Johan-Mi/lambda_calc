use super::lexer::*;
use super::types::*;
use std::rc::Rc;

fn parse_symbol(tokens: &[Token]) -> Option<(Symbol, &[Token])> {
    match tokens.split_first() {
        Some((Token::Ident(symbol), tokens)) => {
            Some((Symbol::new(symbol.to_string()), tokens))
        }
        _ => None,
    }
}

fn parse_lparen(tokens: &[Token]) -> Option<&[Token]> {
    match tokens.split_first() {
        Some((Token::LParen, tokens)) => Some(tokens),
        _ => None,
    }
}

fn parse_rparen(tokens: &[Token]) -> Option<&[Token]> {
    match tokens.split_first() {
        Some((Token::RParen, tokens)) => Some(tokens),
        _ => None,
    }
}

fn parse_dot(tokens: &[Token]) -> Option<&[Token]> {
    match tokens.split_first() {
        Some((Token::Dot, tokens)) => Some(tokens),
        _ => None,
    }
}

fn parse_backslash(tokens: &[Token]) -> Option<&[Token]> {
    match tokens.split_first() {
        Some((Token::Backslash, tokens)) => Some(tokens),
        _ => None,
    }
}

fn parse_lambda(tokens: &[Token]) -> Option<(Lambda, &[Token])> {
    fn parse_lambda_internal(tokens: &[Token]) -> Option<(Lambda, &[Token])> {
        let (var, tokens) = parse_symbol(tokens)?;
        if let Some(tokens) = parse_dot(tokens) {
            let (body, tokens) = parse_expression(tokens)?;
            Some((Lambda::new(var, Rc::new(body)), tokens))
        } else {
            let (body, tokens) = parse_lambda_internal(tokens)?;
            Some((Lambda::new(var, Rc::new(Object::Lambda(body))), tokens))
        }
    }

    let tokens = parse_lparen(tokens)?;
    let tokens = parse_backslash(tokens)?;
    let (lambda, tokens) = parse_lambda_internal(tokens)?;
    let tokens = parse_rparen(tokens)?;
    Some((lambda, tokens))
}

fn parse_application(tokens: &[Token]) -> Option<(Application, &[Token])> {
    let tokens = parse_lparen(tokens)?;
    let (func, tokens) = parse_expression(tokens)?;
    let (first_arg, mut tokens) = parse_expression(tokens)?;
    let mut application = Application::new(Rc::new(func), Rc::new(first_arg));
    while let Some((arg, remaining_tokens)) = parse_expression(tokens) {
        tokens = remaining_tokens;
        application = Application::new(
            Rc::new(Object::Application(application)),
            Rc::new(arg),
        );
    }
    let tokens = parse_rparen(tokens)?;
    Some((application, tokens))
}

fn parse_expression(tokens: &[Token]) -> Option<(Object, &[Token])> {
    if let Some((expr, tokens)) = parse_lambda(tokens) {
        Some((Object::Lambda(expr), tokens))
    } else if let Some((expr, tokens)) = parse_application(tokens) {
        Some((Object::Application(expr), tokens))
    } else if let Some((expr, tokens)) = parse_symbol(tokens) {
        Some((Object::Symbol(expr), tokens))
    } else {
        None
    }
}

pub fn parse_expressions(
    mut tokens: &[Token],
) -> Option<(Vec<Object>, &[Token])> {
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
