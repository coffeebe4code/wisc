use errors::*;
use logos::*;
use tokens::*;

pub struct LexerSource<'source> {
    pub lexer: Lexer<'source, Token<'source>>,
    peeked: Option<Option<Token<'source>>>,
}

impl<'source> LexerSource<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            lexer: Token::lexer(source),
            peeked: None,
        }
    }

    pub fn peek(&mut self) -> Option<&Token<'source>> {
        if self.peeked.is_none() {
            self.peeked = Some(self.lexer.next());
        }
        self.peeked.as_ref().unwrap().as_ref()
    }
    pub fn reset(&mut self) -> () {
        self.peeked = None;
    }
}

pub trait TokenExpects<'source> {
    fn expect_some(&self) -> Result<&Token<'source>, Error>;
}

impl<'source> TokenExpects<'source> for Option<&Token<'source>> {
    fn expect_some(&self) -> Result<&Token<'source>, Error> {
        match self {
            Some(t) => Ok(t),
            None => Err(Error {
                str_error: "error".to_string(),
            }),
        }
    }
}

pub trait ResultExpects<'source> {
    fn expect_kind(&self, kind: &dyn Fn(&Token<'source>) -> bool)
        -> Result<&Token<'source>, Error>;
}

impl<'source> ResultExpects<'source> for Result<&Token<'source>, Error> {
    fn expect_kind(
        &self,
        kind: &dyn Fn(&Token<'source>) -> bool,
    ) -> Result<&Token<'source>, Error> {
        match self {
            Ok(t) => {
                if kind(*t) {
                    return Ok(t);
                }
                Err(Error {
                    str_error: "error".to_string(),
                })
            }
            _ => Err(Error {
                str_error: "error".to_string(),
            }),
        }
    }
}

impl<'source> Iterator for LexerSource<'source> {
    type Item = Token<'source>;

    fn next(&mut self) -> Option<Token<'source>> {
        if let Some(peeked) = self.peeked.take() {
            peeked
        } else {
            self.lexer.next()
        }
    }
}
pub fn bin_kind<'source>(tok: &Token<'source>) -> bool {
    match tok {
        Token::Plus => true,
        Token::Sub => true,
        Token::Mul => true,
        Token::Div => true,
        Token::Mod => true,
        Token::Not => true,
        Token::Xor => true,
        Token::Or => true,
        Token::And => true,
        Token::LShift => true,
        Token::RShift => true,
        _ => false,
    }
}
#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn lex_peek() {
        let mut lex = LexerSource::new("this is a test");
        assert_eq!(lex.peek().unwrap(), &Token::This);
        assert_eq!(lex.next().unwrap(), Token::This);
        assert_eq!(lex.peek().unwrap(), &Token::Symbol);
        assert_eq!(lex.lexer.slice(), "is");
        assert_eq!(lex.peek().unwrap(), &Token::Symbol);
        assert_eq!(lex.lexer.slice(), "is");
    }
}
