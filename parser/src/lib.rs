use errors::*;
use lexer::*;
use tokens::*;

pub enum Expr {
    Literal {
        val: Token,
    },
    BinExpr {
        op: Token,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
}

impl Expr {
    pub fn new_literal(val: Token) -> Self {
        Self::Literal { val }
    }
    pub fn new_binexpr(op: Token, lhs: Expr, rhs: Expr) -> Self {
        Self::BinExpr {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }
}

pub struct ParserSource<'source> {
    prev: Option<Expr>,
    peeked: Option<Option<Expr>>,
    lexer: LexerSource<'source>,
}

impl<'source> ParserSource<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            prev: None,
            peeked: None,
            lexer: LexerSource::new(source),
        }
    }
    pub fn peek(&mut self) -> Option<&Expr> {
        if self.peeked.is_none() {
            let peek = self.parse_expr();
            match peek {
                Ok(expr) => self.peeked = Some(Some(expr)),
                Err(err) => self.peeked = Some(None),
            }
        }
        self.peeked.as_ref().unwrap().as_ref()
    }

    pub fn expect_token(&mut self, token: Token) -> Result<Token, Error> {
        let peek = self.lexer.peek();
        let result = peek.expect_some()?;
        if variant_comp(result, &token) {
            return Ok(self.lexer.next().unwrap());
        }
        Err(Error {
            str_error: "error".to_string(),
        })
    }

    pub fn parse_binexpr(&mut self) -> Result<Expr, Error> {
        let lhs = self.prev.unwrap();
        let peek = self.lexer.peek();
        peek.expect_some().expect_kind(&bin_kind)?;
        let actual = self.lexer.next().unwrap();
        let mut rhs = self.next();
        match rhs {
            Some(expr) => {
                self.prev = Some(expr);
                let assoc_check = self.lexer.peek().expect_some().expect_kind(&rh_assoc_kind);
                match assoc_check {
                    Ok(new_token) => {
                        let inner_rhs = self.parse_expr()?;
                        return Ok(Expr::new_binexpr(actual, lhs, inner_rhs));
                    }
                    Err(err) => {
                        return Ok(Expr::new_binexpr(actual, lhs, expr));
                    }
                }
            }
            None => Err(Error {
                str_error: "error".to_string(),
            }),
        }
    }
    pub fn parse_expr(&mut self) -> Result<Expr, Error> {
        let peek = self.lexer.peek();
        let result = peek.expect_some()?;


        Ok(())
    }
}
impl<'source> Iterator for ParserSource<'source> {
    type Item = Expr;

    fn next(&mut self) -> Option<Expr> {
        if let Some(peeked) = self.peeked.take() {
            peeked
        } else {
            match self.parse_expr() {
                Ok(expr) => Some(expr),
                Err(err) => None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
}
