#![feature(iter_advance_by)]
#[derive(Debug, PartialEq)]
pub struct TPAError(String);

const TYPES_L: &'static [&'static str] = &[
    "i32",
    "u32",
    "i64",
    "i16",
    "u16",
    "u8",
    "i8",
    "bit",
    "f64",
    "f32",
    "fn",
    "null",
    "char",
    "string",
    "box",
    "ptr",
    "addr",
    "list",
];

const KEYWORDS_L: &'static [&'static str] = &[
    "mut",
    "const",
    "i32",
    "u32",
    "i64",
    "i16",
    "u16",
    "u8",
    "i8",
    "bit",
    "f64",
    "f32",
    "fn",
    "if",
    "else",
    "type",
    "this",
    "null",
    "undef",
    "char",
    "string",
    "inline",
    "static",
    "switch",
    "for",
    "in",
    "of",
    "break",
    "enum",
    "pub",
    "return",
    "async",
    "await",
    "box",
    "trait",
    "ptr",
    "match",
    "addr",
    "list",
    "vol",
    "true",
    "false",
];

const KEYWORDS_SIZE_L: &'static [u8] = &[
    3,
    5,
    3,
    3, // u32
    3,
    3,
    3,
    2,
    2,
    3, // bit
    3,
    3,
    2,
    2, // if
    4,
    4,
    4,
    4,
    5, // undef
    4,
    6, // string
    6,
    6,
    6, // switch
    3,
    2,
    2,
    5, // break
    4,
    3,
    6,
    5,
    5,
    3,
    5,
    3,
    5, // match
    4,
    4,
    3,
    4,
    5
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
}

#[derive(Debug, PartialEq)]
pub enum TOKEN {
    Dot,
    Dollar,
    Pound,
    Words(String),
    Keywords(KEYWORDS),
    Number(String),
    Operator(String),
    Char(char),
    OArray,
    CArray,
    SQuote,
    DQuote,
    FSlash,
    BSlash,
    Comma,
    Colon,
    SColon,
    Backtick,
    At,
    OCaret,
    CCaret,
    UCaret,
    CParen,
    OParen,
    CBrace,
    OBrace,
    Empty,
    End,
}

fn seek_past_newline_and_empty(data: &str) -> usize {
    let mut index = 0;
    let mut found = false;
    for c in data.chars() {
        match c {
            ' ' => {}
            '\n' => found = true,
            '\t' => {}
            _ => {
                if found == true {
                    break;
                }
            }
        }
        index += 1;
    }
    return index;
}

fn safe_ender(c: char) -> bool {
    match c {
            ' ' => true,
            '\n' => true,
            '\t' => true,
            ')' => true,
            '}' => true,
            ';' => true,
            ',' => true,
            ']' => true,
            _ => false,
    }
}

fn parse_number(data: &str) -> Result<(TOKEN, usize), (TPAError, usize)> {
    let mut index = 0;
    let mut found = false;
    for c in data.chars() {
        match c {
            c if safe_ender(c) => { found = true; }
            'x' => {}
            'b' => {}
            'f' => {}
            'l' => {}
            c if c.is_digit(10) => {}
            _ => {
                return Err((
                    TPAError(format!("invalid character found in number {}", c)),
                    index,
                ));
            }
        }
        index += 1;
        if found == true {
            break;
        }
    }
    let val = &data[..index - 1];
    return Ok((TOKEN::Number(val.parse().unwrap()), index - 1));
}

fn ensure_word_to_end(data: &str) -> Result<usize, (TPAError, usize)> {
    let mut index = 0;
    let mut found = false;
    for c in data.chars() {
        match c {
            c if safe_ender(c) => { found = true; }
            c if c.is_alphabetic() => {
                index += 1;
            }
            c if c.is_digit(10) => {
                index += 1;
            }
            '-' => {
                index += 1;
            }
            '_' => index += 1,
            _ => {
                return Err((TPAError(format!("invalid character in word {}", c)), index));
            }
        }
        if found {
            break;
        }
    }
    return Ok(index);
}

fn seek_past_whitespace(data: &str) -> usize {
    let mut index = 0;
    let mut found = false;
    for c in data.chars() {
        match c {
            ' ' => {
                index -= 1;
                found = true
            }
            '\n' => {
                index -= 1;
                found = true
            }
            '\t' => {
                index -= 1;
                found = true
            }
            _ => {
                if found == true {
                    break;
                }
                index += 1;
            }
        }
    }
    return index;
}

pub fn parse_char(data: &str) -> Result<(TOKEN, usize), (TPAError, usize)> {
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
                            return Err((
                                TPAError(format!("invalid escaped character {}", c)),
                                index,
                            ));
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
        return Err((TPAError("expected closing \'".to_string()), index));
    }
    return Ok((TOKEN::Char(new_data), index - 1));
}

pub fn parse_quoted(data: &str) -> Result<(TOKEN, usize), (TPAError, usize)> {
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
                            return Err((
                                TPAError(format!("invalid escape character {}", c)),
                                index,
                            ));
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
        return Err((TPAError("expected closing \"".to_string()), index));
    }
    return Ok((TOKEN::Words(new_data), index - 1));
}

pub fn parse_array(data: &str) -> Result<(TOKEN, usize), (TPAError, usize)> {
    let mut vec: Vec<TOKEN> = vec![];
    let mut closed = false;
    let mut index = 0;
    let mut iter = data.chars();
    loop {
        match iter.next() {
            None => {
                return Err((TPAError(format!("expected closing ]").to_string()), index));
            }
            Some(' ') => {
                index += 1;
            }
            Some('\'') => {
                let char_or = parse_char(&data[index..]);
            }
            Some(']') => {
                closed = true;
                index += 1;
                break;
            }
            Some('"') => {
                let quote_or = parse_quoted(&data[index..]);
                match quote_or {
                    Err(val) => return Err(val),
                    Ok(val) => {
                        vec.push(val.0);
                        iter.advance_by(val.1 - 1);
                        index += val.1;
                    }
                }
            }
            Some(val) => {
                if val.is_alphabetic() {
                    let word_or = make_ident_or_error(&data[index..], 6);
                    match word_or {
                        Err(v) => return Err(v),
                        Ok(v) => {
                            vec.push(v.0);
                            iter.advance_by(v.1 - 1);
                            index += v.1;
                        }
                    }
                } else if val.is_digit(10) {
                    let num_or = parse_number(&data[index..]);
                    match num_or {
                        Err(v) => return Err(v),
                        Ok(v) => {
                            vec.push(v.0);
                            iter.advance_by(v.1 - 1);
                            index += v.1;
                        }
                    }
                } else {
                    return Err((TPAError(format!("invalid character {:?}", iter)), index));
                }
            }
        }
    }
    if !closed {
        return Err((TPAError(format!("expected closing ]").to_string()), index));
    }
    return Ok((TOKEN::Array(vec), index - 1));
}

pub fn tokenize(data: &str) -> Result<(TOKEN, usize), (TPAError, usize)> {
    let curr = data.chars().next().unwrap();
    match curr {
        ';' => Ok((TOKEN::End, seek_past_newline(data))),
        ':' => make_ident_or_error(&data[1..], 2),
        ',' => make_ident_or_error(&data[1..], 5),
        ')' => Ok((TOKEN::CParen, 1)),
        '(' => Ok((TOKEN::OParen, 1)),
        '$' => make_ident_or_error(&data[1..], 0),
        '"' => parse_quoted(&data[1..]),
        '@' => make_ident_or_error(&data[1..], 1),
        '%' => make_ident_or_error(&data[1..], 3),
        '#' => make_ident_or_error(&data[1..], 4),
        '/' => Ok((TOKEN::Comment, seek_past_newline(data))),
        '[' => parse_array(&data[1..]),
        '\'' => parse_char(&data[1..]),
        '\t' => Ok((TOKEN::Empty, seek_past_whitespace(&data[1..]))),
        '\n' => Ok((TOKEN::Empty, seek_past_whitespace(&data[1..]))),
        ' ' => Ok((TOKEN::Empty, seek_past_whitespace(&data[1..]))),
        c if c.is_alphabetic() => make_ident_or_error(&data[..], 6),
        c if c.is_digit(10) => parse_number(&data[..]),
        _ => Err((TPAError(format!("invalid token: {}", curr)), 0)),
    }
}
#[cfg(test)]
mod tests {
    use crate::{KEYWORDS_L, KEYWORDS_SIZE_L};
    #[test]
    fn correct_keywords_lengths() {
        assert_eq!(KEYWORDS_SIZE_L.len(), KEYWORDS_L.len());
        for (i,x) in KEYWORDS_L.iter().enumerate() {
            assert_eq!(x.len(), usize::from(KEYWORDS_SIZE_L[i]));
        }
    }
}
