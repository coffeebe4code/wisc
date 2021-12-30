use logos::*;
use tokens::*;
use lexer::*;

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
    pub fn parse_declaration(&mut self) -> Result<(),_> {
        
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
