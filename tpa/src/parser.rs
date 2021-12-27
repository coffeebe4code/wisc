use crate::lexer::*;
use crate::token::*;
use crate::tracker::*;

#[derive(Debug, PartialEq)]
pub enum SigKind {
    Name(TOKEN, Span),
    Body(Vec<(TOKEN, Span, Box<SigKind>)>),
    Func(Vec<(TOKEN, Span, Box<SigKind>)>, Box<SigKind>)
}
impl SigKind {
    pub fn new_name(tok: TOKEN, span: Span) -> Self {
        Self::Name(tok,span)
    }
    pub fn new_func(props: Vec<(TOKEN, Span, Box<SigKind>)>, ret: Box<SigKind>) -> Self {
        Self::Func(props, ret)
    }
    pub fn new_body(props: Vec<(TOKEN, Span, Box<SigKind>)>) -> Self {
        Self::Body(props)
    }
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    NumLiteral(TOKEN, Span),
    CharLiteral(TOKEN, Span),
    StringLiteral(TOKEN, Span),
    TypeExpr(Option<(TOKEN, Span)>, Box<Expr>),
    Body(Vec<Box<Expr>>),
    Signature(SigKind),
    Declaration(TOKEN, Span, Vec<(TOKEN, Span)>, Option<Box<Expr>>),
    BinOp(TOKEN, Span, Box<Expr>, Box<Expr>),
    UnOp(TOKEN, Span, Box<Expr>, Box<Expr>),
    AsOp(TOKEN, Span, Box<Expr>, Box<Expr>),
    Call(TOKEN, Span, Vec<Box<Expr>>),
    Error(TOKEN, Span),
    Import(TOKEN, Span),
    Macro(TOKEN, Span, Box<Expr>)
}

pub fn parse_preproc(tracker: &mut Tracker) -> Result<Expr, (TOKEN, Span)> {
    let (keywords, span) = expect(tracker, &lex_preproc_keywords)?;
    match keywords {
        TOKEN::Pre(PREPROC::IMPORT) => {
            tracker.skip_empty();
            let (path, pathspan) = expect(tracker, &lex_quoted)?;
            return Ok(Expr::Import(path, pathspan));
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
                tracker.skip_empty();
            },
            TOKEN::Words(_) => {
                name = (word, word_span);
                tracker.skip_empty();
                break;
            }
            _ => return Err((word, word_span))
        }
    }
    let next = tracker.get_next();
    return Ok(Expr::Declaration(name.0, name.1, mods, None));
}

pub fn parse_signame(tracker: &mut Tracker) -> Result<Expr, (TOKEN, Span)> {
    let quoted = expect(tracker, &lex_word);
    match quoted {
        Ok(q) => { return Ok(Expr::Signature(SigKind::new_name(q.0, q.1))) }
        Err(e) => { 
            return Err(e);
            }
        } 
    }

pub fn parse_signature(tracker: &mut Tracker) -> Result<Expr, (TOKEN, Span)> {
    let mut expr: Expr;
    let mut result = parse_signame(tracker);
    result = if_error_parse(tracker, &parse_sigbody, result);
    result = if_error_parse(tracker, &parse_sigfunc, result);
    result = if_error_parse(tracker, &parse_sigarray, result);
    return result;
}

////pub fn parse_body(tracker: &mut Tracker) -> Result<Expr, (TOKEN, Span)> {
//    
//}

pub fn if_error_parse(tracker: &mut Tracker, parse: &dyn Fn(&str), result: Result<(TOKEN, Span), (TOKEN, Span)>) -> Result<(TOKEN, Span), (TOKEN, Span)> {
    match result {
       Err(_) => {
           tracker.reset();
           return parse(tracker);
       }
       Ok(_) => result
    }
}
pub fn expect_token(token: (TOKEN, Span), expected: TOKEN) -> Result<(TOKEN, Span), (TOKEN, Span)> {
    match token.0 {
        expected => Ok(token),
        _ => Err(token)
    }
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
