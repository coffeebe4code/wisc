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
    pub fn new_binexpr(op: Token<'source>, lhs: Expr<'source>, rhs: Expr<'source>) -> Self {
        Self::BinExpr {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }
}

pub struct ParserSource<'source> {
    prev: Option<Expr<'source>>,
    lexer: LexerSource<'source>,
}

impl<'source> ParserSource<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            prev: None,
            lexer: LexerSource::new(source),
        }
    }

    pub fn expect_token(&mut self, token: Token) -> Result<Token<'source>, Error> {
        let peek = self.lexer.peek();
        let result = peek.expect_some()?;
        if variant_comp(result, &token) {
            return Ok(self.lexer.next().unwrap());
        }
        Err(Error {
            str_error: "error".to_string(),
        })
    }
    pub fn parse_binexpr(&mut self) -> Result<Expr<'source>, Error> {
        let peek = self.lexer.peek();
        let result = peek.expect_some().expect_kind(&bin_kind)?;
        Ok(Expr::new_binexpr(
            self.lexer.next().unwrap(),
            Expr::new_literal(Token::Num),
            Expr::new_literal(Token::Num),
        ))
    }
    pub fn parse_any_expr(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
}
