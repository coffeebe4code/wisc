pub struct TPAError(String);
pub struct AST {}
pub enum TOKEN {
    Type(String),
    Declaration(String),
    Property(String),
    Builtin(String),
    Pre(String),
    Interface(String),
    CParen,
    OParen,
    Comment,
    Quoted(String),
    Float(f64),
    Array(Vec<TOKEN>),
    Number(u64),
    Words(String),
    Char(char),
    Empty,
    End,
}

//pub fn tokenize(data: &str) -> Result<(TOKEN, usize), (TPAError, usize)> {
pub fn tokenize() -> Result<(TOKEN, usize), (TPAError, usize)> {
    //    let curr = data.chars().next().unwrap();
    return Ok((TOKEN::End, 1));
    //    match curr {
    //      ';' => Ok((TOKEN::End, seek_past_newline(data))),
    //        ':' => make_ident_or_error(&data[1..], 2),
    //        ',' => make_ident_or_error(&data[1..], 5),
    //        ')' => Ok((TOKEN::CParen, 1)),
    //        '(' => Ok((TOKEN::OParen, 1)),
    //        '$' => make_ident_or_error(&data[1..], 0),
    //        '"' => parse_quoted(&data[1..]),
    //        '@' => make_ident_or_error(&data[1..], 1),
    //        '%' => make_ident_or_error(&data[1..], 3),
    //        '#' => make_ident_or_error(&data[1..], 4),
    //        '[' => parse_array(&data[1..]),
    //        '\'' => parse_char(&data[1..]),
    //        '\t' => Ok((TOKEN::Empty, seek_past_whitespace(&data[1..]))),
    //        '\n' => Ok((TOKEN::Empty, seek_past_whitespace(&data[1..]))),
    //        ' ' => Ok((TOKEN::Empty, seek_past_whitespace(&data[1..]))),
    //        c if c.is_alphabetic() => make_ident_or_error(&data[..], 6),
    //        c if c.is_digit(10) => parse_number(&data[..]),
    //        _ => Err((TPAError(format!("invalid token: {}", curr)), 0)),
    //    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
