use errors::*;
use logos::*;
use tokens::*;

pub struct LexerSource<'source> {
    pub lexer: Lexer<'source, Token>,
    peeked: Option<Option<Token>>,
}

impl<'source> LexerSource<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            lexer: Token::lexer(source),
            peeked: None,
        }
    }

    pub fn peek(&mut self) -> Option<&Token> {
        if self.peeked.is_none() {
            self.peeked = Some(self.lexer.next());
        }
        self.peeked.as_ref().unwrap().as_ref()
    }
    pub fn reset(&mut self) -> () {
        self.peeked = None;
    }
}

pub trait TokenExpects {
    fn expect_some(&self) -> Result<&Token, Error>;
}

impl TokenExpects for Option<&Token> {
    fn expect_some(&self) -> Result<&Token, Error> {
        match self {
            Some(t) => {
                if variant_comp(t, &&Token::Error) {
                    return Err(Error {
                        str_error: "error".to_string(),
                    });
                } else {
                    return Ok(t);
                }
            }
            None => Err(Error {
                str_error: "error".to_string(),
            }),
        }
    }
}

pub trait ResultExpects {
    fn expect_kind(&self, kind: &dyn Fn(&Token) -> bool) -> Result<&Token, Error>;
}

impl ResultExpects for Result<&Token, Error> {
    fn expect_kind(&self, kind: &dyn Fn(&Token) -> bool) -> Result<&Token, Error> {
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
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if let Some(peeked) = self.peeked.take() {
            peeked
        } else {
            self.lexer.next()
        }
    }
}
pub fn literal_kind(tok: &Token) -> bool {
    match tok {
        Token::Hex(_) => true,
        Token::Num(_) => true,
        Token::Bin(_) => true,
        Token::SQuote(_)=> true,
        Token::DQuote(_) => true,
        _ => false,
    }
}
pub fn rh_assoc_kind(tok: &Token) -> bool {
    match tok {
        Token::Mul => true,
        Token::Div => true,
        Token::Mod => true,
        Token::Not => true,
        Token::Xor => true,
        Token::Or => true,
        Token::And => true,
        Token::LShift => true,
        Token::RShift => true,
        Token::OParen => true,
        _ => false,
    }
}
pub fn bin_kind(tok: &Token) -> bool {
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

pub fn expr_starter(tok: &Token) -> bool {
    match tok {
        Token::Symbol => true,
        Token::Struct => true,
        Token::Pound => true,
        Token::Dot => true,
        Token::Mut => true,
        Token::Const => true,
        Token::Type => true,
        Token::Async => true,
        Token::Await => true,
        Token::Break => true,
        Token::True => true,
        Token::False => true,
        Token::Dollar => true,
        Token::Static => true,
        Token::IFace => true,
        Token::Inline => true,
        Token::OParen => true,
        Token::OBrace => true,
        Token::OArray => true,
        Token::If => true,
        Token::Match => true,
        Token::For => true,
        Token::Dec => true,
        Token::Inc => true,
        Token::Pub => true,
        Token::Return => true,
        Token::Enum => true,
        Token::Trait => true,
        Token::Vol => true,
        Token::At => true,
        Token::Data => true,
        Token::As => true,
        Token::AddAs => true,
        Token::OrAs => true,
        Token::XorAs => true,
        Token::ModAs => true,
        Token::SubAs => true,
        Token::DivAs => true,
        Token::LShiftAs => true,
        Token::RShiftAs => true,
        Token::DQuote(_) => true,
        Token::SQuote(_) => true,
        Token::Hex(_) => true,
        Token::Bin(_) => true,
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
