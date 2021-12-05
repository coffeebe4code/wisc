#![feature(iter_advance_by)]
#[derive(Debug, PartialEq)]
pub struct TPAError(String);

const TYPES_L: &'static [&'static str] = &[
    "i32", "u32", "i64", "i16", "u16", "u8", "i8", "bit", "f64", "f32", "fn", "null", "char",
    "string", "box", "ptr", "addr", "list",
];

const KEYWORDS_L: &'static [&'static str] = &[
    "mut", "const", "i32", "u32", "i64", "i16", "u16", "u8", "i8", "bit", "f64", "f32", "fn", "if",
    "else", "type", "this", "null", "undef", "char", "string", "inline", "static", "switch", "for",
    "in", "of", "break", "enum", "pub", "return", "async", "await", "box", "trait", "ptr", "match",
    "addr", "list", "vol", "true", "false",
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
    4, 4, 3, 4, 5,
];

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
            _ => {
                panic!("no enum for u32");
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
    Operator(OPS),
    Number([u8; 8]),
    Words(String),
    Keywords(KEYWORDS),
    Template(String),
    Char(char),
    Error(String),
    Percent,
    OArray,
    CArray,
    FSlash,
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
}

fn parse_number(data: &str, vec: &mut Vec<(TOKEN, usize)>) -> () {
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
    let val = &data[..index - 1];
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

pub fn parse_op_fslash(data: &str, vec: &mut Vec<(TOKEN, usize)>) -> () {
    let c = data.chars().next();
    match c {
        Some(c) => {
            if c == '=' {
                vec.push((TOKEN::Operator(OPS::DivAs), 2));
            } else {
                vec.push((TOKEN::Operator(OPS::Div), 1));
            }
        }
        _ => {
            vec.push((TOKEN::Operator(OPS::Div), 1));
        }
    }
}
pub fn parse_char(data: &str, vec: &mut Vec<(TOKEN, usize)>, prev: &TOKEN) -> () {
    let mut escape = false;
    let mut closed = false;
    let mut new_data: char = '\0';
    let mut index = 0;
    for c in data.chars() {
        match c {
            '\\' => {
                escape = true;
                index += 1
            }
            '\'' => {
                if !escape {
                    closed = true;
                    index += 1;
                    break;
                }
                new_data = '\'';
                index += 1;
            }
            _ => {
                if escape {
                    match c {
                        'n' => {
                            new_data = '\n';
                        }
                        't' => {
                            new_data = '\t';
                        }
                        '\\' => {
                            new_data = '\\';
                        }
                        'r' => {
                            new_data = '\r';
                        }
                        '0' => {
                            new_data = '\0';
                        }
                        'x' => {
                            new_data = '\x10';
                        }
                        'u' => {
                            new_data = '\u{0010}';
                        }
                        _ => {
                            break;
                        }
                    }
                } else {
                    new_data = c;
                }
                index += 1;
            }
        }
    }
    if !closed {
        return vec.push((TOKEN::Error("expected closing \'".to_string()), index));
    }
    return vec.push((TOKEN::Char(new_data), index - 1));
}

pub fn parse_word(data: &str, vec: &mut Vec<(TOKEN, usize)>) -> () {
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
    for (i, _x) in KEYWORDS_L.iter().enumerate() {
        if index == KEYWORDS_SIZE_L[i] {
            if KEYWORDS_L[i].eq(&data[..index]) {
                found = true;
                vec.push((TOKEN::Keywords(KEYWORDS::from_usize(i)), KEYWORDS_SIZE_L[i]));
            }
        }
    }
    if !found {
        vec.push((TOKEN::Words(String::from(&data[..index])), index));
    }
}

pub fn parse_template(data: &str, vec: &mut Vec<(TOKEN, usize)>) -> () {
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
}

pub fn parse_quoted(data: &str, vec: &mut Vec<(TOKEN, usize)>, prev: &TOKEN) -> () {
    let mut escape = false;
    let mut closed = false;
    let mut new_data: String = "".to_string();
    let mut index = 0;
    for c in data.chars() {
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
        return vec.push((TOKEN::Error("expected closing \"".to_string()), index));
    }
}

pub fn tokenize(data: &str) -> Vec<(TOKEN, usize)> {
    let mut vec = Vec::new();
    let prev: TOKEN = TOKEN::Empty;
    for curr in data.chars() {
        match curr {
            ';' => vec.push((TOKEN::SColon, 1)),
            ':' => vec.push((TOKEN::Colon, 1)),
            ',' => vec.push((TOKEN::Comma, 1)),
            ')' => vec.push((TOKEN::CParen, 1)),
            '(' => vec.push((TOKEN::OParen, 1)),
            '$' => vec.push((TOKEN::Dollar, 1)),
            '"' => {
                vec.push((TOKEN::DQuote, 1));
                parse_quoted(&data[1..], &mut vec, &prev);
            }
            '@' => vec.push((TOKEN::At, 1)),
            '%' => vec.push((TOKEN::Percent, 1)),
            '#' => vec.push((TOKEN::Pound, 1)),
            '/' => vec.push((TOKEN::FSlash, 1)),
            '[' => vec.push((TOKEN::OArray, 1)),
            ']' => vec.push((TOKEN::CArray, 1)),
            '\'' => {
                vec.push((TOKEN::SQuote, 1));
                parse_char(&data[1..], &mut vec, &prev);
            }
            '\t' => vec.push((TOKEN::Empty, seek_past_whitespace(&data[1..]) + 1)),
            '\n' => vec.push((TOKEN::Empty, seek_past_whitespace(&data[1..]) + 1)),
            ' ' => vec.push((TOKEN::Empty, seek_past_whitespace(&data[1..]) + 1)),
            c if c.is_alphabetic() => parse_word(&data[..], &mut vec),
            c if c.is_digit(10) => parse_number(&data[..], &mut vec),
            _ => vec.push((TOKEN::Error("Invalid token found".to_string()), 1)),
        }
    }
    return vec;
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_correct_keywords_lengths() {
        assert_eq!(KEYWORDS_SIZE_L.len(), KEYWORDS_L.len());
        for (i, x) in KEYWORDS_L.iter().enumerate() {
            assert_eq!(x.len(), KEYWORDS_SIZE_L[i]);
        }
    }
    #[test]
    fn test_seek_past_whitespace_end() {
        println!("thing");
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
    fn test_tokenize_simple() {
        let vec = tokenize(";:,)($@%#[]");
        let semi = vec.get(0).unwrap();
        let colon = vec.get(1).unwrap();
        let comma = vec.get(2).unwrap();
        let cparen = vec.get(3).unwrap();
        let oparen = vec.get(4).unwrap();
        let dollar = vec.get(5).unwrap();
        let at = vec.get(6).unwrap();
        let percent = vec.get(7).unwrap();
        let pre = vec.get(8).unwrap();
        let oarr = vec.get(9).unwrap();
        let carr = vec.get(10).unwrap();

        assert_eq!(semi.0, TOKEN::SColon);
        assert_eq!(colon.0, TOKEN::Colon);
        assert_eq!(comma.0, TOKEN::Comma);
        assert_eq!(cparen.0, TOKEN::CParen);
        assert_eq!(oparen.0, TOKEN::OParen);
        assert_eq!(dollar.0, TOKEN::Dollar);
        assert_eq!(at.0, TOKEN::At);
        assert_eq!(percent.0, TOKEN::Percent);
        assert_eq!(pre.0, TOKEN::Pound);
        assert_eq!(oarr.0, TOKEN::OArray);
        assert_eq!(carr.0, TOKEN::CArray);
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
        let two = vec.get(1).unwrap();

        assert_eq!(one.0, TOKEN::Keywords(KEYWORDS::CONST));

        assert_eq!(two.0, TOKEN::Keywords(KEYWORDS::MUT));
    }

    #[test]
    fn test_tokenize_words() {
        let vec = tokenize("hello worlds");
        let one = vec.get(0).unwrap();
        let two = vec.get(1).unwrap();

        assert_eq!(one.0, TOKEN::Words("hello".to_string()));
        assert_eq!(one.1, 5);

        assert_eq!(two.0, TOKEN::Words("worlds".to_string()));
        assert_eq!(two.1, 6);
    }
}
