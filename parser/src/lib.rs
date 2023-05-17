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
}

impl<'source> ParserSource<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            lexer: LexerSource::new(source),
        }
    }

    pub fn parse_binexpr(&mut self) -> Result<Expr, Error> {
        let lhs = begin().parse_literal(self)?;
        let peek = self.lexer.peek();
        let peek_some = peek.expect_some()?;
        if peek_some.is_bin() {
            let result = self.parse_expr(Some(lhs))?;
            return Ok(result);
        }
        Err(Error {
            str_error: "error".to_string(),
        })
    }

    pub fn parse_literal(&mut self) -> Result<Expr, Error> {
        let peek = self.lexer.peek();
        let peek_some = peek.expect_some()?;
        match peek_some {
            peek_some if peek_some.is_literal() => {
                Ok(Expr::new_literal(self.lexer.next().unwrap()))
            }
            _ => Err(Error {
                str_error: "error".to_string(),
            }),
        }
    }

    pub fn parse_expr(&mut self, lhs: Option<Expr>) -> Result<Expr, Error> {
        let peek = self.lexer.peek();
        let peek_some = peek.expect_some()?;
        match peek_some {
            peek_some if peek_some.is_literal() => Ok(self.parse_literal()?),
            peek_some if peek_some.is_bin() => Ok(self.parse_binexpr()?),
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

pub trait ParseChainer<'source> {
    fn parse_literal(&self, parser: &mut ParserSource<'source>) -> Result<Expr, Error>;
    fn parse_bin(&self, parser: &mut ParserSource<'source>) -> Result<Expr, Error>;
    fn parse_mul(&self, parser: &mut ParserSource<'source>) -> Result<Expr, Error>;
    fn parse_add(&self, parser: &mut ParserSource<'source>) -> Result<Expr, Error>;
}

impl<'source> ParseChainer<'source> for Result<Expr, Error> {
    fn parse_literal(&self, parser: &mut ParserSource<'source>) -> Result<Expr, Error> {
        match self {
            Ok(val) => Ok(val.to_owned()),
            Err(_) => {
                let peek = parser.lexer.peek();
                let peek_some = peek.expect_some()?;
                match peek_some {
                    peek_some if peek_some.is_literal() => {
                        Ok(Expr::new_literal(parser.lexer.next().unwrap()))
                    }
                    _ => Err(Error {
                        str_error: "error".to_string(),
                    }),
                }
            }
        }
    }
    fn parse_add(&self, parser: &mut ParserSource<'source>) -> Result<Expr, Error> {
        Err(Error {
            str_error: "error".to_string(),
        })
    }
    fn parse_mul(&self, parser: &mut ParserSource<'source>) -> Result<Expr, Error> {
        Err(Error {
            str_error: "error".to_string(),
        })
    }
    fn parse_bin(&self, parser: &mut ParserSource<'source>) -> Result<Expr, Error> {
        Err(Error {
            str_error: "error".to_string(),
        })
    }
}

fn begin() -> Result<Expr, Error> {
        Err(Error {
            str_error: "error".to_string(),
        })
}

pub trait ExprExpect {
    fn expect_some(&self) -> Result<Expr, Error>;
}

impl ExprExpect for Option<Expr> {
    fn expect_some(&self) -> Result<Expr, Error> {
        match self {
            Some(t) => Ok(t.to_owned()),
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
