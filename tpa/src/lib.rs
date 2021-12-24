#![feature(iter_advance_by)]

mod lexer;
mod parser;
mod token;
mod tracker;

use crate::lexer::*;
use crate::parser::*;
use crate::token::*;
use crate::tracker::*;

//pub fn parse_start(data: &str) -> Vec<Expr> {
//    let mut vec: Vec<Expr> = Vec::new();
//    let mut tracker = Tracker::new(data);
//    loop {
//        let token = tracker.get_next();
//        match token {
//            TOKEN::Pound => {
//                let result = parse_preproc(&mut tracker);
//                match result {
//                    Ok(r) => vec.push(r),
//                    _ => {}
//                }
//            }
//            TOKEN::EOF => {
//                break;
//            }
//            _ => vec.push(Expr::Error(Node::new(
//                TOKEN::Error("error".to_string()),
//                Span::new(1, 1),
//            ))),
//        }
//    }
//    if tracker.current() == 0 {
//        vec.push(Expr::Error(Node::new(TOKEN::EOF, Span::new(0, 0))));
//        return vec;
//    }
//    return vec;
//}

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
            assert_eq!(format!("{}", KEYWORDS_L[i]), format!("{}", KEYWORDS::from_usize(i))); 
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
    fn test_parse_import() {
        let mut tracker = Tracker::new("import \"math\"");

        let result = parse_preproc(&mut tracker);
        let import = (TOKEN::Pre(PREPROC::IMPORT), Span::new(0, 6));
        let words = Box::new(Expr::StringLiteral((
            TOKEN::Words("math".to_string()),
            Span::new(7, 13),
        )));
        let expected = Expr::PreExpr(import, words);
        assert_eq!(result.unwrap(), expected);
    }
}
