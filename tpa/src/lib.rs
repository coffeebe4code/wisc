#![feature(iter_advance_by)]

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
#[derive(Debug,PartialEq)]
pub enum Expr {
    Literal(Node<TOKEN>),
    Identifier(Node<TOKEN>),
    Symbol(Node<TOKEN>),
    BinOp(Node<TOKEN>, Node<Expr>, Node<Expr>),
    UnOp(Node<TOKEN>, Node<Expr>, Node<Expr>),
    AsOp(Node<TOKEN>, Node<Expr>),
    Call(Node<TOKEN>, Vec<Node<Expr>>),
    Statement(Node<TOKEN>, Node<Expr>),
    Error(Node<TOKEN>),
    PreExpr(Node<TOKEN>, Node<Expr>),
}

const PREPROC_L: &'static [&'static str] = &["import", "define", "macro", "test"];
const PREPROC_SIZE_L: &'static [usize] = &[6, 6, 5, 4];
const KEYWORDS_L: &'static [&'static str] = &[
    "mut", "const", "i32", "u32", "i64", "i16", "u16", "u8", "i8", "bit", "f64", "f32", "fn", "if",
    "else", "type", "this", "null", "undef", "char", "string", "inline", "static", "switch", "for",
    "in", "of", "break", "enum", "pub", "return", "async", "await", "box", "trait", "ptr", "match",
    "addr", "list", "vol", "true", "false", "func", "function", "void",
];

const KEYWORDS_SIZE_L: &'static [usize] = &[
    3, 5, 3, 3, // u32
    3, 3, 3, 2, 2, 3, // bit
    3, 3, 2, 2, // if
    4, 4, 4, 4, 5, // undef
    4, 6, // string
    6, 6, 6, // switch
    3, 2, 2, 5, // break
    4, 3, 6, 5, 5, 3, 5, 3, 5, // match
    4, 4, 3, 4, 5, 4, 8, 4, // void
];
#[derive(Debug, PartialEq)]
pub enum PREPROC {
    IMPORT,
    DEFINE,
    MACRO,
    TEST,
}

impl PREPROC {
    fn from_usize(value: usize) -> PREPROC {
        match value {
            0 => PREPROC::IMPORT,
            1 => PREPROC::DEFINE,
            2 => PREPROC::MACRO,
            3 => PREPROC::TEST,
            _ => {
                panic!("no enum for usize");
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum KEYWORDS {
    MUT,
    CONST,
    I32,
    U32,
    I64,
    I16,
    U16,
    U8,
    I8,
    BIT,
    F64,
    F32,
    FN,
    IF,
    ELSE,
    TYPE,
    THIS,
    NULL,
    UNDEF,
    CHAR,
    STRING,
    INLINE,
    STATIC,
    SWITCH,
    FOR,
    IN,
    OF,
    BREAK,
    ENUM,
    PUB,
    RETURN,
    ASYNC,
    AWAIT,
    BOX,
    TRAIT,
    PTR,
    MATCH,
    ADDR,
    VOL,
    LIST,
    TRUE,
    FALSE,
    VOID,
}

impl KEYWORDS {
    fn from_usize(value: usize) -> KEYWORDS {
        match value {
            0 => KEYWORDS::MUT,
            1 => KEYWORDS::CONST,
            2 => KEYWORDS::I32,
            3 => KEYWORDS::U32,
            4 => KEYWORDS::I64,
            5 => KEYWORDS::I16,
            6 => KEYWORDS::U16,
            7 => KEYWORDS::U8,
            8 => KEYWORDS::I8,
            9 => KEYWORDS::BIT,
            10 => KEYWORDS::F64,
            11 => KEYWORDS::F32,
            12 => KEYWORDS::FN,
            13 => KEYWORDS::IF,
            14 => KEYWORDS::ELSE,
            15 => KEYWORDS::TYPE,
            16 => KEYWORDS::THIS,
            17 => KEYWORDS::NULL,
            18 => KEYWORDS::UNDEF,
            19 => KEYWORDS::CHAR,
            20 => KEYWORDS::STRING,
            21 => KEYWORDS::INLINE,
            22 => KEYWORDS::STATIC,
            23 => KEYWORDS::SWITCH,
            24 => KEYWORDS::FOR,
            25 => KEYWORDS::IN,
            26 => KEYWORDS::OF,
            27 => KEYWORDS::BREAK,
            28 => KEYWORDS::ENUM,
            29 => KEYWORDS::PUB,
            30 => KEYWORDS::RETURN,
            31 => KEYWORDS::ASYNC,
            32 => KEYWORDS::AWAIT,
            33 => KEYWORDS::BOX,
            34 => KEYWORDS::TRAIT,
            35 => KEYWORDS::PTR,
            36 => KEYWORDS::MATCH,
            37 => KEYWORDS::ADDR,
            38 => KEYWORDS::LIST,
            39 => KEYWORDS::VOL,
            40 => KEYWORDS::TRUE,
            41 => KEYWORDS::FALSE,
            42 => KEYWORDS::FN,
            43 => KEYWORDS::FN,
            44 => KEYWORDS::VOID,
            _ => {
                panic!("no enum for usize");
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum OPS {
    And,    // &
    Or,     // |
    Xor,    // ^
    LShift, // <<
    RShift, // >>
    Not,    // ~
    // Assignment
    As,       // =
    NotAs,    // ~=
    AndAs,    // &=
    OrAs,     // |=
    XorAs,    // ^=
    LShiftAs, // <<=
    RShiftAs, // >>=
    // Logical
    AndLog,      // &&
    OrLog,       // ||
    NotEquality, // !=
    Equality,    // ==
    NotLog,      // !
    // Relational
    Lt,   // <
    LtEq, // <=
    Gt,   // >
    GtEq, // >=
    // Operators
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    Mod, // %
    Inc, // ++
    Dec, // --
    // Operator and Assign
    AddAs, // +=
    SubAs, // -=
    MulAs, // *=
    DivAs, // /=
    ModAs, // %=
}

#[derive(Debug, PartialEq)]
pub enum TOKEN {
    Dot,
    Dollar,
    Pound,
    Digit,
    Alpha,
    Operator(OPS),
    Number([u8; 8]),
    Words(String),
    Keywords(KEYWORDS),
    Pre(PREPROC),
    TOpen(String),
    TClose(String),
    Char(char),
    Error(String),
    OArray,
    CArray,
    Comma,
    Colon,
    SColon,
    DQuote,
    SQuote,
    Backtick,
    At,
    OCaret,
    CCaret,
    CParen,
    OParen,
    CBrace,
    OBrace,
    Empty,
    EOF,
    FSlash,
    BSlash,
    Plus,
    Minus,
    Gt,
    Lt,
    Asterisk,
    Amp,
    Pipe,
    Carrot,
    Mod,
    Exclam,
    Tilde,
    Equals,
}

pub struct Tracker<'a> {
    slice: &'a str,
    prev: usize,
    index: usize,
}

pub fn get_token(c: char) -> TOKEN {
                match c {
                    ';' => TOKEN::SColon,
                    ':' => TOKEN::Colon,
                    ',' => TOKEN::Comma,
                    ')' => TOKEN::CParen,
                    '(' => TOKEN::OParen,
                    '$' => TOKEN::Dollar,
                    '{' => TOKEN::OBrace,
                    '}' => TOKEN::CBrace,
                    '"' => TOKEN::DQuote,
                    '@' => TOKEN::At,
                    '#' => TOKEN::Pound,
                    '[' => TOKEN::OArray,
                    ']' => TOKEN::CArray,
                    '\'' => TOKEN::SQuote,
                    '/' => TOKEN::FSlash,
                    '\\' => TOKEN::BSlash,
                    '+' => TOKEN::Plus, 
                    '>' => TOKEN::Gt,
                    '<' => TOKEN::Lt,
                    '-' => TOKEN::Minus,
                    '&' => TOKEN::Amp,
                    '|' => TOKEN::Pipe,
                    '^' => TOKEN::Carrot,
                    '%' => TOKEN::Mod,
                    '*' => TOKEN::Asterisk,
                    '!' => TOKEN::Exclam,
                    '~' => TOKEN::Tilde,
                    '=' => TOKEN::Equals, 
                    '\t' => TOKEN::Empty,
                    '\n' => TOKEN::Empty,
                    ' ' => TOKEN::Empty,
                    c if c.is_alphabetic() => TOKEN::Alpha,
                    c if c.is_digit(10) => TOKEN::Digit,
                    _ => TOKEN::Error("Invalid token found".to_string())
                }
}

impl<'a> Tracker<'a> {
    pub fn new(slice: &'a str) -> Self {
        Tracker {
            slice,
            prev: 0,
            index: 0
        }
    }
    pub fn adv(&mut self, inc: usize) -> () {
            self.prev = self.index;
            self.index += inc;
    }
    pub fn get_next(&mut self) -> TOKEN {
        let mut local_token = TOKEN::EOF;
        match self.slice[self.index..].chars().next() {
            Some(c) => {
                self.index += 1;
                local_token = get_token(c);
            }
            _ => ()
        }
        return local_token;
    }
    pub fn get_slice(&self) -> &'a str {
        return &self.slice[self.index..];
    }
}

pub fn parse_start(data: &str) -> Vec<Expr> {
    let mut vec: Vec<Expr> = Vec::new();
    let mut tracker = Tracker::new(data);
    loop {
    let token = tracker.get_next();
        match token {
            TOKEN::Pound => {
                vec.push(parse_preproc(&mut tracker));
            }
            TOKEN::EOF => {
                break;
            }
            _ => vec.push(Expr::Error(Node::new(TOKEN::Error("error".to_string()),Span::new(1,1))))
        }
    }
    if tracker.index == 0 {
        vec.push(Expr::Error(Node::new(TOKEN::EOF, Span::new(0,0))));
        return vec;
    }
    return vec;
}

pub fn parse_preproc(tracker: &mut Tracker) -> Expr {
    let token = lex_preproc_keywords(tracker.get_slice());
    tracker.adv(token.1);
    let span = Span::new(tracker.prev, tracker.index);
    match token.0 {
        TOKEN::Pre(PREPROC::IMPORT) => {
            let seek_amount = seek_past_whitespace(tracker.get_slice());
            tracker.adv(seek_amount);
            let quoted = lex_quoted(tracker.get_slice());
            tracker.adv(quoted.1);
            match quoted.0 {
                TOKEN::Words(_) => { 
                    let token_node = Node::new(token.0, span);
                    let quoted_node = Node::new(quoted.0, Span::new(tracker.prev, tracker.index));
                    return Expr::PreExpr(token_node, Node::new(Expr::Literal(quoted_node), Span::new(tracker.prev, tracker.index)));
                }
                _ => {
                    return Expr::Error(Node::new(token.0, span));
                }
            }
        }
        _ => {
            return Expr::Error(Node::new(token.0, span));
        }
    }
}

fn lex_number(data: &str) -> (TOKEN, usize) {
    let mut index = 0;
    let mut found = false;
    for c in data.chars() {
        match c {
            'x' | 'b' => {
                if index != 1 {
                    found = true;
                    index -= 1;
                }
            }
            'f' => {}
            'l' => {}
            c if c.is_digit(10) => {}
            _ => {
                found = true;
            }
        }
        index += 1;
        if found == true {
            break;
        }
    }
    // return (TOKEN::Number((&data[..index - 1]).to), index);
    return (TOKEN::Number(usize::to_be_bytes(64)), index);
}

fn seek_past_whitespace(data: &str) -> usize {
    let mut index = 0;
    for c in data.chars() {
        match c {
            ' ' | '\n' | '\t' => {
                index += 1;
            }
            _ => {
                break;
            }
        }
    }
    return index;
}

pub fn has_escaped_error(c: char) -> (bool, char) {
    let mut error = false;
    let mut d = '\0';
    match c {
        '\\' => d = '\\',
        't' => d = '\t',
        'n' => d = '\n',
        'r' => d = '\r',
        '\'' => d = '\'',
        '\"' => d = '\"',
        '0' => d = '\0',
        _ => error = true,
    }

    return (error, d);
}

pub fn lex_char(data: &str) -> (TOKEN, usize) {
    let mut is_escaped = false;
    let mut new_data: char = '\0';
    let mut index = 1;
    for c in data.chars() {
        match c {
            '\\' => {
                is_escaped = true;
            }
            '\'' => {
                if !is_escaped {
                    break;
                }
                new_data = '\'';
            }
            _ => {
                if is_escaped {
                    match has_escaped_error(c) {
                        (false, n) => {
                            new_data = n;
                            index += 1;
                            break;
                        }
                        (true, _) => {
                            return (TOKEN::Error("invalid escaped character".to_string()), index);
                        }
                    }
                } else {
                    new_data = c;
                }
            }
        }
        index += 1;
    }
    return (TOKEN::Char(new_data), index);
}

pub fn lex_preproc_keywords(data: &str) -> (TOKEN, usize) {
    let mut index = 0;
    let mut found = false;
    let mut tok: (TOKEN, usize) = (TOKEN::EOF, 0);
    for c in data.chars() {
        match c {
            c if c.is_alphabetic() => {
                index += 1;
            }
            c if c.is_digit(10) => {
                index += 1;
            }
            '-' | '_' => {
                index += 1;
            }
            _ => {
                break;
            }
        }
    }
    for (i, _x) in PREPROC_L.iter().enumerate() {
        if index == PREPROC_SIZE_L[i] {
            if PREPROC_L[i].eq(&data[..index]) {
                found = true;
                tok = (TOKEN::Pre(PREPROC::from_usize(i)), PREPROC_SIZE_L[i]);
            }
        }
    }
    if !found {
        return (
            TOKEN::Error(String::from("expected valid preprocessor keywords")),
            index,
        );
    }
    return tok;
}
pub fn lex_word(data: &str) -> (TOKEN, usize) {
    let mut index = 0;
    let mut found = false;
    let mut tok: (TOKEN, usize) = (TOKEN::Empty, 0);
    for c in data.chars() {
        match c {
            c if c.is_alphabetic() => {
                index += 1;
            }
            c if c.is_digit(10) => {
                index += 1;
            }
            '-' | '_' => {
                index += 1;
            }
            _ => {
                break;
            }
        }
    }
    for (i, _x) in KEYWORDS_L.iter().enumerate() {
        if index == KEYWORDS_SIZE_L[i] {
            if KEYWORDS_L[i].eq(&data[..index]) {
                found = true;
                tok = (TOKEN::Keywords(KEYWORDS::from_usize(i)), KEYWORDS_SIZE_L[i]);
            }
        }
    }
    if !found {
        return (TOKEN::Words(String::from(&data[..index])), index);
    }
    return tok;
}

pub fn lex_topen(data: &str) -> (TOKEN, usize) {
    let mut index = 0;
    let mut found = false;
    for c in data.chars() {
        match c {
            c if c.is_alphabetic() => {
                index += 1;
            }
            c if c.is_digit(10) => {
                index += 1;
            }
            '-' | '_' => {
                index += 1;
            }
            _ => {
                break;
            }
        }
    }
    return (TOKEN::TOpen(String::from(&data[..index])), index);
}

pub fn lex_tclose(data: &str) -> (TOKEN, usize) {
    let mut index = 0;
    let mut found = false;
    for c in data.chars() {
        match c {
            c if c.is_alphabetic() => {
                index += 1;
            }
            c if c.is_digit(10) => {
                index += 1;
            }
            '-' | '_' => {
                index += 1;
            }
            _ => {
                break;
            }
        }
    }
    return (TOKEN::TClose(String::from(&data[..index])), index);
}

pub fn lex_quoted(data: &str) -> (TOKEN, usize) {
    let mut escape = false;
    let mut closed = false;
    let mut new_data: String = "".to_string();
    let mut index = 0;
    let tok = (TOKEN::EOF, 0);
    match data.chars().next() {
        Some('"') => { index += 1 }
        Some(_) => { return (TOKEN::Error("expected opening \"".to_string()), 1); }
        None => { return tok }
    }
    for c in (&data[1..]).chars() {
        match c {
            '\\' => {
                escape = true;
                index += 1
            }
            '"' => {
                if !escape {
                        closed = true;
                        index += 1;
                        break;
                }
                new_data.push('\"');
                index += 1;
            }
            _ => {
                if escape {
                    match c {
                        'n' => {
                            new_data.push('\n');
                        }
                        't' => {
                            new_data.push('\t');
                        }
                        '\\' => {
                            new_data.push('\\');
                        }
                        'r' => {
                            new_data.push('\r');
                        }
                        '0' => {
                            new_data.push('\0');
                        }
                        'x' => {
                            new_data.push('\x10');
                        }
                        'u' => {
                            new_data.push('\u{0010}');
                        }
                        _ => {
                            break;
                        }
                    }
                } else {
                    new_data.push(c);
                }
                index += 1;
            }
        }
    }
    if !closed {
        return (TOKEN::Error("expected closing \"".to_string()), index);
    }
    return (TOKEN::Words(new_data), index);
}

pub fn lex_op(data: &str, cmp: char, default: OPS, success: OPS) -> (TOKEN, usize) {
    let c = data.chars().next();
    match c {
        Some(c) => {
            if c == cmp {
                return (TOKEN::Operator(success), 2);
            } else {
                return (TOKEN::Operator(default), 1);
            }
        }
        _ => return (TOKEN::Operator(default), 1),
    }
}
pub fn lex_op3(
    data: &str,
    cmp: char,
    default: OPS,
    success: OPS,
    next: char,
    second_success: OPS,
) -> (TOKEN, usize) {
    let c = data.chars().next();
    match c {
        Some(c) => {
            if c == cmp {
                return (TOKEN::Operator(success), 2);
            } else if c == next {
                return (TOKEN::Operator(second_success), 2);
            } else {
                return (TOKEN::Operator(default), 1);
            }
        }
        _ => return (TOKEN::Operator(default), 1),
    }
}

pub fn tokenize(data: &str) -> Vec<(TOKEN, usize)> {
    let mut vec = Vec::new();
    let mut iter = data.chars();
    let mut skip = 0;
    while let Some(c) = iter.next() {
        match c {
            ';' => vec.push((TOKEN::SColon, 1)),
            ':' => vec.push((TOKEN::Colon, 1)),
            ',' => vec.push((TOKEN::Comma, 1)),
            ')' => vec.push((TOKEN::CParen, 1)),
            '(' => vec.push((TOKEN::OParen, 1)),
            '$' => vec.push((TOKEN::Dollar, 1)),
            '{' => vec.push((TOKEN::OBrace, 1)),
            '}' => vec.push((TOKEN::CBrace, 1)),
            '"' => {
                vec.push(lex_quoted(&data[..]));
            }
            '@' => vec.push((TOKEN::At, 1)),
            '#' => vec.push((TOKEN::Pound, 1)),
            '[' => vec.push((TOKEN::OArray, 1)),
            ']' => vec.push((TOKEN::CArray, 1)),
            '\'' => {
                vec.push(lex_char(&data[skip + 1..]));
            }
            '/' => {
                vec.push(lex_op(&data[skip + 1..], '=', OPS::Div, OPS::DivAs));
            }
            '+' => {
                vec.push(lex_op3(
                    &data[skip + 1..],
                    '=',
                    OPS::Add,
                    OPS::AddAs,
                    '+',
                    OPS::Inc,
                ));
            }
            '>' => {
                vec.push(lex_op3(
                    &data[skip + 1..],
                    '=',
                    OPS::Gt,
                    OPS::GtEq,
                    '>',
                    OPS::RShift,
                ));
            }
            '<' => {
                vec.push(lex_op3(
                    &data[skip + 1..],
                    '=',
                    OPS::Lt,
                    OPS::LtEq,
                    '<',
                    OPS::LShift,
                ));
            }
            '-' => {
                vec.push(lex_op3(
                    &data[skip + 1..],
                    '=',
                    OPS::Sub,
                    OPS::SubAs,
                    '-',
                    OPS::Dec,
                ));
            }
            '&' => {
                vec.push(lex_op3(
                    &data[skip + 1..],
                    '=',
                    OPS::And,
                    OPS::AndAs,
                    '&',
                    OPS::AndLog,
                ));
            }
            '|' => {
                vec.push(lex_op3(
                    &data[skip + 1..],
                    '=',
                    OPS::Or,
                    OPS::OrAs,
                    '|',
                    OPS::OrLog,
                ));
            }
            '^' => {
                vec.push(lex_op(&data[skip + 1..], '=', OPS::Xor, OPS::XorAs));
            }
            '%' => {
                vec.push(lex_op(&data[skip + 1..], '=', OPS::Mod, OPS::ModAs));
            }
            '*' => {
                vec.push(lex_op(&data[skip + 1..], '=', OPS::Mul, OPS::MulAs));
            }
            '!' => {
                vec.push(lex_op(
                    &data[skip + 1..],
                    '=',
                    OPS::NotLog,
                    OPS::NotEquality,
                ));
            }
            '~' => {
                vec.push(lex_op(&data[skip + 1..], '=', OPS::Not, OPS::NotAs));
            }
            '=' => {
                vec.push(lex_op(&data[skip + 1..], '=', OPS::As, OPS::Equality));
            }
            '\t' => vec.push((TOKEN::Empty, seek_past_whitespace(&data[skip + 1..]) + 1)),
            '\n' => vec.push((TOKEN::Empty, seek_past_whitespace(&data[skip + 1..]) + 1)),
            ' ' => vec.push((TOKEN::Empty, seek_past_whitespace(&data[skip + 1..]) + 1)),
            c if c.is_alphabetic() => vec.push(lex_word(&data[skip..])),
            c if c.is_digit(10) => vec.push(lex_number(&data[skip..])),
            _ => vec.push((TOKEN::Error("Invalid token found".to_string()), 1)),
        }
        skip += vec.last().unwrap().1;
        iter.advance_by(vec.last().unwrap().1 - 1).unwrap();
    }
    vec.push((TOKEN::EOF, 0));
    return vec;
}

#[cfg(test)]
mod tests {
    use crate::*;
use std::io::{self, Write};
    #[test]
    fn test_correct_keywords_lengths() {
        assert_eq!(KEYWORDS_SIZE_L.len(), KEYWORDS_L.len());
        for (i, x) in KEYWORDS_L.iter().enumerate() {
            assert_eq!(x.len(), KEYWORDS_SIZE_L[i]);
        }
    }
    #[test]
    fn test_correct_preproc_lengths() {
        assert_eq!(PREPROC_SIZE_L.len(), PREPROC_L.len());
        for (i, x) in PREPROC_L.iter().enumerate() {
            assert_eq!(x.len(), PREPROC_SIZE_L[i]);
        }
    }
    #[test]
    fn test_seek_past_whitespace_end() {
        let len = seek_past_whitespace("  \t\n");
        assert_eq!(len, 4);
    }
    #[test]
    fn test_seek_past_whitespace_in() {
        let len = seek_past_whitespace(" \t\n\ty");
        assert_eq!(len, 4);
    }
    #[test]
    fn test_seek_past_whitespace_early() {
        let len = seek_past_whitespace(" ");
        assert_eq!(len, 1);
    }
    #[test]
    fn test_empty() {
        let vec = tokenize("");
        vec.get(0).unwrap();
        assert_eq!(vec.get(0).unwrap().0, TOKEN::EOF);
    }
    #[test]
    fn test_tokenize_simple() {
        let vec = tokenize(";:,)($@#[]{}");
        let semi = vec.get(0).unwrap();
        let colon = vec.get(1).unwrap();
        let comma = vec.get(2).unwrap();
        let cparen = vec.get(3).unwrap();
        let oparen = vec.get(4).unwrap();
        let dollar = vec.get(5).unwrap();
        let at = vec.get(6).unwrap();
        let pre = vec.get(7).unwrap();
        let oarr = vec.get(8).unwrap();
        let carr = vec.get(9).unwrap();
        let obrac = vec.get(10).unwrap();
        let cbrac = vec.get(11).unwrap();

        assert_eq!(semi.0, TOKEN::SColon);
        assert_eq!(colon.0, TOKEN::Colon);
        assert_eq!(comma.0, TOKEN::Comma);
        assert_eq!(cparen.0, TOKEN::CParen);
        assert_eq!(oparen.0, TOKEN::OParen);
        assert_eq!(dollar.0, TOKEN::Dollar);
        assert_eq!(at.0, TOKEN::At);
        assert_eq!(pre.0, TOKEN::Pound);
        assert_eq!(oarr.0, TOKEN::OArray);
        assert_eq!(carr.0, TOKEN::CArray);
        assert_eq!(obrac.0, TOKEN::OBrace);
        assert_eq!(cbrac.0, TOKEN::CBrace);
    }

    #[test]
    fn test_tokenize_keywords() {
        for (i, x) in KEYWORDS_L.iter().enumerate() {
            let vec = tokenize(&x);
            assert_eq!(
                vec.get(0).unwrap().0,
                TOKEN::Keywords(KEYWORDS::from_usize(i))
            );
        }
    }

    #[test]
    fn test_tokenize_multi_keywords() {
        let vec = tokenize("const mut");
        let one = vec.get(0).unwrap();
        let two = vec.get(2).unwrap();

        assert_eq!(one.0, TOKEN::Keywords(KEYWORDS::CONST));
        assert_eq!(two.0, TOKEN::Keywords(KEYWORDS::MUT));
    }

    #[test]
    fn test_tokenize_words() {
        let vec = tokenize("hello worlds");
        let one = vec.get(0).unwrap();
        let two = vec.get(2).unwrap();
        assert_eq!(one.0, TOKEN::Words("hello".to_string()));
        assert_eq!(one.1, 5);

        assert_eq!(two.0, TOKEN::Words("worlds".to_string()));
        assert_eq!(two.1, 6);
    }
    
    #[test]
    fn test_tokenize_quotes() {
        let vec = tokenize("\"hello worlds\"");
        let one = vec.get(0).unwrap();
        assert_eq!(one.0, TOKEN::Words("hello worlds".to_string()));
        assert_eq!(one.1, 14);
    }

    #[test]
    fn test_tokenize_chars() {
        let vec = tokenize("'\\0' 'c' 'a' '\\r' '\\n' '\\t' 'h' '\\z'");
        let one = vec.get(0).unwrap();
        let two = vec.get(2).unwrap();
        let three = vec.get(4).unwrap();
        let four = vec.get(6).unwrap();
        let five = vec.get(8).unwrap();
        let six = vec.get(10).unwrap();
        let seven = vec.get(12).unwrap();
        let eight = vec.get(14).unwrap();
        assert_eq!(one.0, TOKEN::Char('\0'));
        assert_eq!(two.0, TOKEN::Char('c'));
        assert_eq!(three.0, TOKEN::Char('a'));
        assert_eq!(four.0, TOKEN::Char('\r'));
        assert_eq!(five.0, TOKEN::Char('\n'));
        assert_eq!(six.0, TOKEN::Char('\t'));
        assert_eq!(seven.0, TOKEN::Char('h'));
        assert_eq!(
            eight.0,
            TOKEN::Error("invalid escaped character".to_string())
        );
    }

    #[test]
    fn test_tokenize_ops() {
        let vec0 = tokenize("+ += ++");
        let vec1 = tokenize("- -= --");
        let vec2 = tokenize("* *=");
        let vec3 = tokenize("% %=");
        let vec4 = tokenize("/ /=");
        let vec5 = tokenize("< <= << <<=");
        let vec6 = tokenize("> >= >> >>=");
        let vec7 = tokenize("& &= &&");
        let vec8 = tokenize("| |= ||");
        let vec9 = tokenize("! !=");
        let vec10 = tokenize("~ ~=");
        let vec11 = tokenize("= ==");

        assert_eq!(vec0.get(0).unwrap().0, TOKEN::Operator(OPS::Add));
        assert_eq!(vec0.get(2).unwrap().0, TOKEN::Operator(OPS::AddAs));
        assert_eq!(vec0.get(4).unwrap().0, TOKEN::Operator(OPS::Inc));

        assert_eq!(vec1.get(0).unwrap().0, TOKEN::Operator(OPS::Sub));
        assert_eq!(vec1.get(2).unwrap().0, TOKEN::Operator(OPS::SubAs));
        assert_eq!(vec1.get(4).unwrap().0, TOKEN::Operator(OPS::Dec));

        assert_eq!(vec2.get(0).unwrap().0, TOKEN::Operator(OPS::Mul));
        assert_eq!(vec2.get(2).unwrap().0, TOKEN::Operator(OPS::MulAs));

        assert_eq!(vec3.get(0).unwrap().0, TOKEN::Operator(OPS::Mod));
        assert_eq!(vec3.get(2).unwrap().0, TOKEN::Operator(OPS::ModAs));

        assert_eq!(vec4.get(0).unwrap().0, TOKEN::Operator(OPS::Div));
        assert_eq!(vec4.get(2).unwrap().0, TOKEN::Operator(OPS::DivAs));

        assert_eq!(vec5.get(0).unwrap().0, TOKEN::Operator(OPS::Lt));
        assert_eq!(vec5.get(2).unwrap().0, TOKEN::Operator(OPS::LtEq));
        assert_eq!(vec5.get(4).unwrap().0, TOKEN::Operator(OPS::LShift));
        // assert_eq!(vec5.get(6).unwrap().0, TOKEN::Operator(OPS::LShiftAs));

        assert_eq!(vec6.get(0).unwrap().0, TOKEN::Operator(OPS::Gt));
        assert_eq!(vec6.get(2).unwrap().0, TOKEN::Operator(OPS::GtEq));
        assert_eq!(vec6.get(4).unwrap().0, TOKEN::Operator(OPS::RShift));
        // assert_eq!(vec6.get(6).unwrap().0, TOKEN::Operator(OPS::RShiftAs));

        assert_eq!(vec7.get(0).unwrap().0, TOKEN::Operator(OPS::And));
        assert_eq!(vec7.get(2).unwrap().0, TOKEN::Operator(OPS::AndAs));
        assert_eq!(vec7.get(4).unwrap().0, TOKEN::Operator(OPS::AndLog));

        assert_eq!(vec8.get(0).unwrap().0, TOKEN::Operator(OPS::Or));
        assert_eq!(vec8.get(2).unwrap().0, TOKEN::Operator(OPS::OrAs));
        assert_eq!(vec8.get(4).unwrap().0, TOKEN::Operator(OPS::OrLog));

        assert_eq!(vec9.get(0).unwrap().0, TOKEN::Operator(OPS::NotLog));
        assert_eq!(vec9.get(2).unwrap().0, TOKEN::Operator(OPS::NotEquality));

        assert_eq!(vec10.get(0).unwrap().0, TOKEN::Operator(OPS::Not));
        assert_eq!(vec10.get(2).unwrap().0, TOKEN::Operator(OPS::NotAs));

        assert_eq!(vec11.get(0).unwrap().0, TOKEN::Operator(OPS::As));
        assert_eq!(vec11.get(2).unwrap().0, TOKEN::Operator(OPS::Equality));
    }
    #[test]
    fn test_parse_preprocessor_commands() {
        let result = parse_start("#import \"math\"");
        let import = Node::new(TOKEN::Pre(PREPROC::IMPORT), Span::new(1,7));
        let words = Node::new(Expr::Literal(Node::new(TOKEN::Words("math".to_string()), Span::new(8, 14))), Span::new(8,14));
        let expected = Expr::PreExpr(import, words);
        assert_eq!(*result.get(0).unwrap(), expected);
    }
}
