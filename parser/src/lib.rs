use errors::*;
use lexer::*;
use tokens::*;

pub enum Expr<'source> {
    Literal {
        val: Token<'source>,
    },
    BinExpr {
        op: Token<'source>,
        lhs: Box<Expr<'source>>,
        rhs: Box<Expr<'source>>,
    },
}
impl<'source> Expr<'source> {
    pub fn new_literal(val: Token<'source>) -> Self {
        Self::Literal { val }
    }
}

pub struct ParserSource<'source> {
    prev: Option<Expr<'source>>,
    lexer: LexerSource<'source>,
    lineno: usize,
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
        let peek = self.lexer.peek();
        let result = peek.expect_some()?;
        if variant_comp(result, &Token::NewLine) {
            self.lineno += 1;
            return self.expect_token(token);
        } else if variant_comp(result, &token) {
            return Ok(self.lexer.next().unwrap());
        }
        Err(Error {
            str_error: "error".to_string(),
        })
    }
    pub fn parse_binexpr(&mut self) -> Result<(), Error> {
        let peek = self.lexer.peek();
        let result = peek.expect_some()?;
        if result.bin_kind() {
            return Ok(());
        }
        Err(Error {
            str_error: "error".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
}
