use logos::*;
use tokens::*;
use lexer::*;

pub struct ParserError {
    lineno: usize,
    error_string: String
}

pub struct ParserSource<'source> {
    lexer: LexerSource<'source>,
    lineno: usize
}

impl<'source> ParserSource<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            lexer: LexerSource::new(source),
            lineno: 0,
        }
    }

    pub fn expect_token(&mut self, token: Token) -> Result<Token<'source>, ParserError> {
        match self.lexer.peek() {
            Some(t) => {
                if variant_comp(t, &Token::NewLine) {
                    self.lineno += 1;
                    return self.expect_token(token);
                }
                else if variant_comp(t, &token) {
                    return Ok(self.lexer.next().unwrap());
                }
                Err(ParserError { lineno: self.lineno, error_string: "error".to_string() })
            },
            None => Err(ParserError { lineno: self.lineno, error_string: "error".to_string()})
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
}
