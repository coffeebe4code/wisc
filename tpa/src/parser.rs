use crate::lexer::*;
use crate::token::*;
use crate::tracker::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
    pub fn len(self) -> usize {
        return self.end - self.start + 1;
    }
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    NumLiteral((TOKEN, Span)),
    CharLiteral((TOKEN, Span)),
    StringLiteral((TOKEN, Span)),
    Declaration((TOKEN, Span), Vec<(TOKEN, Span)>),
    BinOp((TOKEN, Span), Box<Expr>, Box<Expr>),
    UnOp((TOKEN, Span), Box<Expr>, Box<Expr>),
    AsOp((TOKEN, Span), Box<Expr>, Box<Expr>),
    Call((TOKEN, Span), Vec<Box<Expr>>),
    Error((TOKEN, Span)),
    PreExpr((TOKEN, Span), Box<Expr>),
}

pub fn parse_preproc(tracker: &mut Tracker) -> Result<Expr, (TOKEN, Span)> {
    let (keywords, span) = expect(tracker, &lex_preproc_keywords)?;
    match keywords {
        TOKEN::Pre(PREPROC::IMPORT) => {
            tracker.skip_empty();
            let (path, pathspan) = expect(tracker, &lex_quoted)?;
            return Ok(Expr::PreExpr((keywords, span), Box::new(Expr::StringLiteral((path, pathspan))),
            ));
        }
        _ => return Err((keywords, span)),
    }
}

pub fn parse_declaration(tracker: &mut Tracker) -> Result<Expr, (TOKEN, Span)> {
    let mut mods:Vec<(TOKEN, Span)> = Vec::new();
    let name: (TOKEN, Span);
    loop {
        let (word, word_span) = expect(tracker, &lex_word)?; 
        match word {
            TOKEN::Keywords(KEYWORDS::PUB) => {
                mods.push((word, word_span));    
                seek_past_whitespace(tracker.get_slice());
            },
            TOKEN::Words(_) => {
                name = (word, word_span);
                seek_past_whitespace(tracker.get_slice());
                break;
            }
            _ => return Err((word, word_span))
        }
    }
    let next = tracker.get_next();
    return Ok(Expr::Declaration(name, mods));
}

pub fn expect(
    tracker: &mut Tracker,
    lex: &dyn Fn(&str) -> (TOKEN, usize),
) -> Result<(TOKEN, Span), (TOKEN, Span)> {
    let outcome = lex(&tracker.get_slice());
    tracker.adv(outcome.1);
    let span = Span::new(tracker.prev(), tracker.current());
    match outcome.0 {
        TOKEN::Error(_) => {
            return Err((outcome.0, span));
        }
        _ => {
            return Ok((outcome.0, span));
        }
    }
}
