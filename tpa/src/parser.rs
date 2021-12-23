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
pub struct Node<T> {
    inner: Box<T>,
    span: Span,
}

impl<T> Node<T> {
    pub fn new(inner: T, span: Span) -> Self {
        Node {
            inner: Box::new(inner),
            span,
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum Expr {
    NumLiteral((TOKEN, Span)),
    CharLiteral((TOKEN, Span)),
    StringLiteral((TOKEN, Span)),
    MutDec((TOKEN, Span)),
    ConstDec((TOKEN, Span)),
    Identifier((TOKEN, Span)),
    Symbol((TOKEN, Span)),
    BinOp((TOKEN, Span), Box<Expr>, Box<Expr>),
    UnOp((TOKEN, Span), Box<Expr>, Box<Expr>),
    AsOp((TOKEN, Span), Box<Expr>, Box<Expr>),
    Call((TOKEN, Span), Vec<Box<Expr>>),
    Statement((TOKEN, Span), Box<Expr>),
    Error((TOKEN, Span)),
    PreExpr((TOKEN, Span), Box<Expr>),
}

pub fn parse_preproc(tracker: &mut Tracker) -> Result<Expr, (TOKEN, Span)> {
    let (keywords, span) = expect(tracker, &lex_preproc_keywords)?;
    match keywords {
        TOKEN::Pre(PREPROC::IMPORT) => {
            tracker.adv(seek_past_whitespace(tracker.get_slice()));
            let (path, pathspan) = expect(tracker, &lex_quoted)?;
            return Ok(Expr::PreExpr((keywords, span), Box::new(Expr::StringLiteral((path, pathspan))),
            ));
        }
        _ => return Err((keywords, span)),
    }
}

pub fn parse_declaration(tracker: &mut Tracker) -> Result<Expr, (TOKEN, Span)> {
    let (word, span) = expect(tracker, &lex_word)?;
    match word {
        TOKEN::Keywords(PREPROC::IMPORT) => {
            tracker.adv(seek_past_whitespace(tracker.get_slice()));
            let (path, pathspan) = expect(tracker, &lex_quoted)?;
            return Ok(Expr::PreExpr((keywords, span), Box::new(Expr::StringLiteral((path, pathspan))),
            ));
        }
        _ => return Err((keywords, span)),
    }
}

pub fn expect(
    tracker: &mut Tracker,
    lex: &dyn Fn(&str) -> (TOKEN, usize),
) -> Result<(TOKEN, Span), (TOKEN, Span)> {
    let outcome = lex(&tracker.get_slice());
    println!("tracker slice {}", tracker.get_slice());
    println!("tracker prev {}, curr {}", tracker.prev(), tracker.current());
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
