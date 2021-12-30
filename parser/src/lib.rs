use logos::*;
use tokens::*;
use lexer::*;
use errors:*;

pub enum Expr<'source> {
    Literal{ val: Token<'source> },
    BinExpr{ op: Token<'source>, lhs: Box<Expr<'source>>, rhs: Box<Expr<'source>>}
}
impl<'source> Expr<'source> {
    pub fn new_literal(val: Token<'source>) -> Self {
        Self::Literal{val}
    }
}


pub struct ParserSource<'source> {
    prev: Option<Expr<'source>>,
    lexer: LexerSource<'source>,
    lineno: usize
}

impl<'source> ParserSource<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            prev: None,
            lexer: LexerSource::new(source),
            lineno: 0,
        }
    }

    pub fn expect_token(&mut self, token: Token) -> Result<Token<'source>, Error> {
        let result = self.lexer.peek().expect_some()?;
        match self.lexer.peek() {
            Some(t) => {
                if variant_comp(t, &Token::NewLine) {
                    self.lineno += 1;
                    return self.expect_token(token);
                }
                else if variant_comp(t, &token) {
                    return Ok(self.lexer.next().unwrap());
                }
                Err(Error{str_error: "error".to_string()})
            },
            None => Err(Error{str_error: "error".to_string()})
        }
    }
    pub fn parse_binexpr(&mut self) -> Result<(), Error> {
        match self.lexer.peek() {
            Some(t) => {
                if t.bin_kind() {
                    Ok(())
                }
                else {
                    Err(Error { lineno: self.lineno, error_string: "error".to_string() })
                }
            },
            _ => Err(Error { lineno: self.lineno, error_string: "error".to_string()})
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
}
