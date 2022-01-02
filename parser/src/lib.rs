use errors::*;
use lexer::*;
use tokens::*;

#[derive(Clone, Debug, PartialEq)]
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
    peeked_expr: Option<Option<Expr>>,
    lexer: LexerSource<'source>,
}

impl<'source> ParserSource<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            peeked_expr: None,
            lexer: LexerSource::new(source),
        }
    }

    pub fn parse_binexpr(&mut self, lhs: Expr) -> Result<Expr, Error> {
        Err(Error {
            str_error: "error".to_string(),
        })
    }
    pub fn parse_literal(&mut self) -> Result<Expr, Error> {
        let take = self.lexer.next().unwrap();
        let peek = self.lexer.peek();
        let peek_some = peek.expect_some()?;
        if peek_some.is_rh() {
            let result = self.parse_expr()?;
        }
        Ok(Expr::new_literal(take))
    }
    pub fn parse_expr(&mut self) -> Result<Expr, Error> {
        let peek = self.lexer.peek();
        let peek_some = peek.expect_some()?;
        match peek_some {
            peek_some if peek_some.is_literal() => Ok(self.parse_literal()?),
            _ => Err(Error {
                str_error: "error".to_string(),
            }),
        }
    }
}
impl<'source> Iterator for ParserSource<'source> {
    type Item = Expr;

    fn next(&mut self) -> Option<Expr> {
        if let Some(peeked) = self.peeked_expr.take() {
            peeked
        } else {
            match self.parse_expr() {
                Ok(val) => Some(val),
                Err(err) => None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn parse_literal() {
        let data = Expr::new_literal(Token::Num(5));
        let mut parser = ParserSource::new("5 22");
        assert_eq!(parser.next().unwrap(), data);
    }
    #[test]
    fn parse_binexpr() {
        let data = Expr::new_binexpr(
            Token::Plus,
            Expr::new_literal(Token::Num(5)),
            Expr::new_literal(Token::Num(6)),
        );
        let mut parser = ParserSource::new("5 + 6");
        assert_eq!(parser.next().unwrap(), data);
    }
}
