use tokens::Token;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedEOF,
}

pub struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, pos: 0 }
    }
    
    fn current_token(&self) -> &Token {
        &self.tokens[self.pos]
    }
    
    fn advance(&mut self) {
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
    }
    
    fn expect(&mut self, token: Token) -> Result<(), ParseError> {
        if self.current_token() == &token {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken(self.current_token().clone()))
        }
    }

    pub fn parse_e(&mut self) -> Result<(), ParseError> {
        self.parse_t()?;
        self.parse_e_prime()?;
        Ok(())
    }

    fn parse_e_prime(&mut self) -> Result<(), ParseError> {
        match self.current_token() {
            Token::Plus => {
                self.advance(); // consume '+'
                self.parse_t()?;
                self.parse_e_prime()?;
                Ok(())
            }
            Token::EOF | Token::RParen => Ok(()), // ε-production
            _ => Err(ParseError::UnexpectedToken(self.current_token().clone())),
        }
    }

    fn parse_t(&mut self) -> Result<(), ParseError> {
        self.parse_f()?;
        self.parse_t_prime()?;
        Ok(())
    }

    fn parse_t_prime(&mut self) -> Result<(), ParseError> {
        match self.current_token() {
            Token::Mul => {
                self.advance(); // consume '*'
                self.parse_f()?;
                self.parse_t_prime()?;
                Ok(())
            }
            Token::Plus | Token::RParen | Token::EOF => Ok(()), // ε-production
            _ => Err(ParseError::UnexpectedToken(self.current_token().clone())),
        }
    }

    fn parse_f(&mut self) -> Result<(), ParseError> {
        match self.current_token() {
            Token::LParen => {
                self.advance(); // consume '('
                self.parse_e()?;
                self.expect(Token::RParen) // consume ')'
            }
            Token::Id => {
                self.advance(); // consume 'id'
                Ok(())
            }
            Token::Literal => {
                self.advance(); // consume 'literal'
                Ok(())
            }
            _ => Err(ParseError::UnexpectedToken(self.current_token().clone())),
        }
    }
}
