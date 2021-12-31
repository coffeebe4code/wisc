use logos::{Lexer, Logos};
use std::mem::discriminant;

pub fn variant_comp<T>(a: &T, b: &T) -> bool {
    discriminant(a) == discriminant(b)
}

fn hex_bounds<'a>(lexer: &mut Lexer<'a, Token>) -> Result<usize, String> {
    let trimmed = lexer.slice().trim_start_matches("0x");
    let num = usize::from_str_radix(trimmed, 16);
    match num {
        Err(_) => Err("invalid hex value".to_string()),
        Ok(val) => Ok(val),
    }
}

fn bin_bounds<'a>(lexer: &mut Lexer<'a, Token>) -> Result<usize, String> {
    let trimmed = lexer.slice().trim_start_matches("0b");
    let num = usize::from_str_radix(trimmed, 2);
    match num {
        Err(_) => Err("invalid binary value".to_string()),
        Ok(val) => Ok(val),
    }
}
fn slice_begin_end<'a>(trim: &'a str) -> &'a str {
    &trim[1..trim.len() - 1]
}

fn string_bounds<'a>(lexer: &mut Lexer<'a, Token>) -> Result<String, String> {
    let mut escape = false;
    let mut new_data: String = "".to_string();
    for c in slice_begin_end(lexer.slice()).chars() {
        match c {
            '\\' => {
                escape = true;
            }
            '"' => {
                new_data.push('\"');
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
                        _ => {
                            return Err("invalid escape character".to_string());
                        }
                    }
                    escape = false;
                
                } else {
                    new_data.push(c);
                }
            }
        }
    }
    return Ok(new_data);
}

fn char_bounds<'a>(lexer: &mut Lexer<'a, Token>) -> Result<char, String> {
    let mut escape = false;
    let mut data = '\0';
    for c in slice_begin_end(lexer.slice()).chars() {
        match c {
            '\\' => {
                escape = true;
            }
            '\'' => {
                if escape {
                    data = '\''
                }
            }
            _ => {
                if escape {
                    match c {
                        'n' => {
                    data = '\n'
                        }
                        't' => {
                    data = '\t'
                        }
                        '\\' => {
                    data = '\\'
                        }
                        'r' => {
                    data = '\r'
                        }
                        '0' => {
                        }
                        _ => {
                            return Err("invalid escape character".to_string());
                        }
                    }
                } else {
                    data = c;
                }
            }
        }
    }
    return Ok(data);
}

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[token("import")]
    Import,
    #[token("define")]
    Define,
    #[token("macro")]
    Macro,
    #[token("test")]
    Test,
    #[token("bench")]
    Bench,
    #[token("release")]
    Release,
    #[token("debug")]
    Debug,
    #[token("mut")]
    Mut,
    #[token("const")]
    Const,
    #[token("i32")]
    I32,
    #[token("u32")]
    U32,
    #[token("i64")]
    I64,
    #[token("i16")]
    I16,
    #[token("u16")]
    U16,
    #[token("u8")]
    U8,
    #[token("i8")]
    I8,
    #[token("bit")]
    Bit,
    #[token("f64")]
    F64,
    #[token("f32")]
    F32,
    #[token("d32")]
    D32,
    #[token("d64")]
    D64,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("type")]
    Type,
    #[token("this")]
    This,
    #[token("self")]
    WSelf,
    #[token("null")]
    Null,
    #[token("undef")]
    Undef,
    #[token("char")]
    Char,
    #[token("uni")]
    Uni,
    #[token("string")]
    WString,
    #[token("inline")]
    Inline,
    #[token("static")]
    Static,
    #[token("switch")]
    Switch,
    #[token("for")]
    For,
    #[token("in")]
    In,
    #[token("of")]
    Of,
    #[token("break")]
    Break,
    #[token("enum")]
    Enum,
    #[token("pub")]
    Pub,
    #[token("return")]
    Return,
    #[token("async")]
    Async,
    #[token("await")]
    Await,
    #[token("box")]
    WBox,
    #[token("trait")]
    Trait,
    #[token("ptr")]
    Ptr,
    #[token("match")]
    Match,
    #[token("addr")]
    Addr,
    #[token("vol")]
    Vol,
    #[token("list")]
    List,
    #[token("arr")]
    Arr,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("void")]
    Void,

    #[token("->")]
    Yield,
    #[token("(")]
    OParen,
    #[token(")")]
    CParen,
    #[token("{")]
    OBrace,
    #[token("}")]
    CBrace,
    #[token("[")]
    OArray,
    #[token("]")]
    CArray,

    #[token(".")]
    Dot,
    #[token(",")]
    Comma,
    #[token("$")]
    Dollar,
    #[token("?")]
    Question,
    #[token("#")]
    Pound,
    #[token(":")]
    Colon,
    #[token(";")]
    SColon,
    #[token("`")]
    Backtick,
    #[token("@")]
    At,
    #[token("<")]
    Lt,
    #[token(">")]
    Gt,
    #[token("/")]
    Div,
    #[token("\\")]
    BSlash,
    #[token("+")]
    Plus,
    #[token("_")]
    Rest,
    #[token("-")]
    Sub,
    #[token("*")]
    Mul,
    #[token("|")]
    Or,
    #[token("&")]
    And,
    #[token("^")]
    Xor,
    #[token("<<")]
    LShift,
    #[token(">>")]
    RShift,
    #[token("~")]
    Not,
    #[token("=")]
    As,
    #[token("~=")]
    NotAs,
    #[token("|=")]
    OrAs,
    #[token("^=")]
    XorAs,
    #[token("<<=")]
    LShiftAs,
    #[token(">>=")]
    RShiftAs,
    #[token("&&")]
    AndLog,
    #[token("||")]
    OrgLog,
    #[token("!=")]
    NotEquality,
    #[token("==")]
    Equality,
    #[token("!")]
    NotLog,
    #[token("%")]
    Mod,
    #[token("++")]
    Inc,
    #[token("--")]
    Dec,
    #[token("+=")]
    AddAs,
    #[token("-=")]
    SubAs,
    #[token("/=")]
    DivAs,
    #[token("%=")]
    ModAs,

    #[regex(r#""([^"\\]|\\t|\\u|\\n|\\")*""#, string_bounds)]
    DQuote(String),
    #[regex(r#"'(\\')'|'(.|\\t|\\u|\\n|\\\\|\\0|\\r||\\)'"#, char_bounds)]
    SQuote(char),

    #[regex("[a-zA-Z]+")]
    Symbol,
    #[regex("0x[0-9a-fA-F]+", hex_bounds)]
    Hex(usize),
    #[regex("0b[0-1]+", bin_bounds)]
    Bin(usize),
    #[regex("[1-9][0-9]+|0")]
    Num,

    #[token("\n")]
    NewLine,
    #[error]
    #[regex(r"[ \t\r\f]+", logos::skip)]
    Error,
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn lex_keywords() {
        let mut lexer = Token::lexer("static inline macro staticy Static");
        assert_eq!(lexer.next(), Some(Token::Static));
        assert_eq!(lexer.span(), 0..6);
        assert_eq!(lexer.next(), Some(Token::Inline));
        assert_eq!(lexer.span(), 7..13);
        assert_eq!(lexer.next(), Some(Token::Macro));
        assert_eq!(lexer.span(), 14..19);
        assert_eq!(lexer.next(), Some(Token::Symbol));
        assert_eq!(lexer.span(), 20..27);
        assert_eq!(lexer.next(), Some(Token::Symbol));
        assert_eq!(lexer.span(), 28..34);
    }
    #[test]
    fn lex_numbers() {
        let mut lexer = Token::lexer("222 0x22FF 0b01011");
        assert_eq!(lexer.next(), Some(Token::Num));
        assert_eq!(lexer.span(), 0..3);
        assert_eq!(lexer.next(), Some(Token::Hex(8959)));
        assert_eq!(lexer.span(), 4..10);
        assert_eq!(lexer.next(), Some(Token::Bin(11)));
        assert_eq!(lexer.span(), 11..18);
    }
    #[test]
    fn lex_string() {
        let mut lexer = Token::lexer("good \"hello\" \"\" \"\\t\" \"\\\"\"");
        assert_eq!(lexer.next(), Some(Token::Symbol));
        assert_eq!(lexer.span(), 0..4);
        assert_eq!(lexer.next(), Some(Token::DQuote("hello".to_string())));
        assert_eq!(lexer.span(), 5..12);
        assert_eq!(lexer.slice(), "\"hello\"");
        assert_eq!(lexer.next(), Some(Token::DQuote("".to_string())));
        assert_eq!(lexer.span(), 13..15);
        assert_eq!(lexer.next(), Some(Token::DQuote("\t".to_string())));
        assert_eq!(lexer.span(), 16..20);
        assert_eq!(lexer.next(), Some(Token::DQuote("\"".to_string())));
        assert_eq!(lexer.span(), 21..25);
    }
    #[test]
    fn lex_char() {
        let mut lexer = Token::lexer("'c' '\\t' '\\r' '\\0' '' '\\''");
        assert_eq!(lexer.next(), Some(Token::SQuote('c')));
        assert_eq!(lexer.span(), 0..3);
        assert_eq!(lexer.next(), Some(Token::SQuote('\t')));
        assert_eq!(lexer.span(), 4..8);
        assert_eq!(lexer.next(), Some(Token::SQuote('\r')));
        assert_eq!(lexer.span(), 9..13);
        assert_eq!(lexer.next(), Some(Token::SQuote('\0')));
        assert_eq!(lexer.span(), 14..18);
        assert_eq!(lexer.next(), Some(Token::SQuote('\0')));
        assert_eq!(lexer.span(), 19..21);
        assert_eq!(lexer.next(), Some(Token::SQuote('\'')));
        assert_eq!(lexer.span(), 22..26);
    }
    #[test]
    fn lex_single() {
        let mut lexer = Token::lexer(";:,)($@#[]{}?`");
        assert_eq!(lexer.next(), Some(Token::SColon));
        assert_eq!(lexer.next(), Some(Token::Colon));
        assert_eq!(lexer.next(), Some(Token::Comma));
        assert_eq!(lexer.next(), Some(Token::CParen));
        assert_eq!(lexer.next(), Some(Token::OParen));
        assert_eq!(lexer.next(), Some(Token::Dollar));
        assert_eq!(lexer.next(), Some(Token::At));
        assert_eq!(lexer.next(), Some(Token::Pound));
        assert_eq!(lexer.next(), Some(Token::OArray));
        assert_eq!(lexer.next(), Some(Token::CArray));
        assert_eq!(lexer.next(), Some(Token::OBrace));
        assert_eq!(lexer.next(), Some(Token::CBrace));
        assert_eq!(lexer.next(), Some(Token::Question));
        assert_eq!(lexer.next(), Some(Token::Backtick));
    }
}
