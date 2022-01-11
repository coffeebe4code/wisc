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
    lexer: LexerSource<'source>,
    operator: Option<Token>
}

impl<'source> ParserSource<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            lexer: LexerSource::new(source),
            operator: None
        }
    }

    pub fn parse_binexpr(&mut self, lhs: Option<Expr>) -> Result<Expr, Error> {
        // 1 + 2 * 3
        let lhs = lhs.expect_some()?; 
        let operator = self.lexer.next().unwrap(); 
        let peek = self.lexer.peek();
        let peek_some = peek.expect_some()?; 
        if peek_some.is_expr() {
            if self.operator.is_some() {
               if self.operator.take().unwrap().get_precedence() > operator.get_precedence() {
// if 1 * 2 + 3 i need to return 1 * 2, but + has already been taken;
//                    return Ok(Expr::new_binexpr(self.operator.take().unwrap(), lhs, rhs)
               }
            }
            self.operator = Some(operator);
            let result = self.parse_expr(Some(lhs))?;
            return Ok(result);
        }
        Err(Error {
            str_error: "error".to_string(),
        })
    }
    pub fn parse_literal(&mut self) -> Result<Expr, Error> {
        let take = self.lexer.next().unwrap();
        let lit = Expr::new_literal(take);
        let peek = self.lexer.peek();
        match peek {
            Some(val) => {
                match val {
                    val if val.is_rh() => Ok(self.parse_expr(Some(lit))?),
                    _ => Ok(lit)
                }
            },
            None => Ok(lit)
        }
    }
    pub fn parse_expr(&mut self, lhs: Option<Expr>) -> Result<Expr, Error> {
        let peek = self.lexer.peek();
        let peek_some = peek.expect_some()?;
        match peek_some {
            peek_some if peek_some.is_literal() => Ok(self.parse_literal()?),
            peek_some if peek_some.is_bin() => Ok(self.parse_binexpr(lhs)?),
            _ => Err(Error {
                str_error: "error".to_string(),
            }),
        }
    }
}
impl<'source> Iterator for ParserSource<'source> {
    type Item = Expr;

    fn next(&mut self) -> Option<Expr> {
            match self.parse_expr(None) {
                Ok(val) => Some(val),
                Err(err) => None,
        }
    }
}

pub trait ExprExpect {
    fn expect_some(&self) -> Result<Expr, Error>;
}

impl ExprExpect for Option<Expr> {
    fn expect_some(&self) -> Result<Expr, Error> {
        match self {
            Some(t) => {
                Ok(t.clone())
            }
            None => Err(Error {
                str_error: "error".to_string(),
            }),
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
