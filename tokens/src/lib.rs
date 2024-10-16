pub struct Tokens {
    token: Token,
    start: usize,
    end: usize,
    value: TokenValue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Plus,     // '+'
    Sub,      // '-'
    Mul,      // '*'
    Div,      // '/'
    Equ,      // '='
    LParen,   // '('
    RParen,   // ')'
    Id,       // identifier
    Literal,  // literals
    EOF,      // End of input
    Keywords(Keywords),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keywords {
    Int,
    Char,
    String,
    Bool,
    Float,
    Const,
    Break,
    If,
    Else,
    Ifelse,
    EOL,
}

#[derive(Debug, Clone, PartialEq)]

pub enum TokenValue {
    String(String),
    Literal(f64),
}
