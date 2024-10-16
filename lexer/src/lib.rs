use tokens::Token;

pub fn lexer(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    
    while let Some(&ch) = chars.peek() {
        match ch {
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Sub);
                chars.next();
            }
            '*' => {
                tokens.push(Token::Mul);
                chars.next();
            }
            '/' => {
                tokens.push(Token::Div);
                chars.next();
            }
            '=' => {
                tokens.push(Token::Equ);
                chars.next();
            }
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            'a'..='z' | 'A'..='Z' => {
                tokens.push(Token::Id);
                chars.next();
            }
            '0'..='9' => {
                tokens.push(Token::Literal);
                chars.next();
            }
            _ if ch.is_whitespace() => {
                chars.next(); // Skip whitespace
            }
            _ => panic!("Unexpected character: {}", ch),
        }
    }
    
    tokens.push(Token::EOF); // End of input
    println!("{:?}", tokens);
    tokens
}
