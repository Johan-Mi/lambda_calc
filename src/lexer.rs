use logos::Logos;

#[derive(Logos)]
pub enum Token {
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("\\")]
    Backslash,
    #[token(".")]
    Dot,
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),

    #[error]
    #[regex(r"\s+", logos::skip)]
    Error,
}
