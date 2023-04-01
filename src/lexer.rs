use internment::Intern;
use logos::Logos;

#[derive(Logos, PartialEq, Eq)]
pub enum Token {
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("\\")]
    Backslash,
    #[token(".")]
    Dot,
    #[token("=")]
    EqualsSign,
    #[regex(r"[a-zA-Z_-][a-zA-Z0-9_-]*", |lex| Intern::from(lex.slice()))]
    Ident(Intern<str>),

    #[error]
    #[regex(r"\s+", logos::skip)]
    Error,
}
