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
    curr: Option<Expr>,
    peeked_expr: Option<Option<Expr>>,
    lexer: LexerSource<'source>,
}

impl<'source> ParserSource<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            curr: None,
            peeked_expr: None,
            lexer: LexerSource::new(source),
        }
    }
    pub fn peek(&mut self) -> Option<&Expr> {
        if self.peeked_expr.is_none() {
            let peek = self.parse_expr();
            match peek {
                Ok(_) => self.peeked_expr = Some(Some(self.curr().unwrap().to_owned())),
                Err(err) => self.peeked_expr = Some(None),
            }
        }
        self.peeked_expr.as_ref().unwrap().as_ref()
    }
    pub fn curr(&mut self) -> Option<&Expr> {
        return self.curr.as_ref()
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

    pub fn parse_binexpr(&mut self) -> Result<(), Error> {
        let lhs = self.curr.to_owned();
        let peek = self.lexer.peek();
        let peek_some = peek.expect_some();
        peek_some.expect_kind(&bin_kind)?;
        let bin_op = self.lexer.next().unwrap();
        self.parse_expr()?;
        let peek_rh = self.lexer.peek();
        let peek_rh_some = peek_rh.expect_some();
        let rh_op = peek_rh_some.expect_kind(&rh_assoc_kind); 
        match rh_op {
            Ok(new_token) => {
                self.lexer.next().unwrap();
                let inner_rhs = self.parse_expr()?;
                self.curr = Some(Expr::new_binexpr(bin_op, lhs.unwrap(), self.curr.to_owned().unwrap().to_owned()));
            }
            Err(err) => {
                self.curr = Some(Expr::new_binexpr(bin_op, lhs.unwrap(), self.curr.to_owned().unwrap().to_owned()));
            }
        }
        Ok(())
    }
    pub fn parse_literal(&mut self) -> Result<(), Error> {
        let peek = self.lexer.peek();
        let some_peek = peek.expect_some();
        let result = some_peek.expect_kind(&literal_kind);
        match result {
            Ok(lit) => {
                let new_lit = Expr::new_literal(self.lexer.next().unwrap());
                self.curr = Some(new_lit);
                Ok(())
            },
            Err(err) => {
                Err(Error {
                    str_error: "error".to_string(),
                })
            }
        }
        
    }
    pub fn parse_expr(&mut self) -> Result<(), Error> {
        let peek = self.lexer.peek();
        println!("peek => {:?}",peek);
        let some_peek = peek.expect_some();
        println!("some_peek => {:?}",some_peek);
        some_peek.expect_kind(&expr_starter)?;
        let mut result = self.parse_literal();
        match result {
            Err(_) => result = self.parse_binexpr(),
            Ok(_) => {
                // looks like I need to check for a rhs of the literal.
            }
        }
        return result;
    }
}
impl<'source> Iterator for ParserSource<'source> {
    type Item = Expr;

    fn next(&mut self) -> Option<Expr> {
        if let Some(peeked) = self.peeked_expr.take() {
            peeked
        } else {
            match self.parse_expr() {
                Ok(_) => Some(self.curr().to_owned().unwrap().to_owned()),
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
        let data = Expr::new_binexpr(Token::Plus, Expr::new_literal(Token::Num(5)), Expr::new_literal(Token::Num(6)));
        let mut parser = ParserSource::new("5 + 6");
        assert_eq!(parser.next().unwrap(), data);
    }
}
