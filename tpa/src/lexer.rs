use crate::token::*;

pub fn lex_body(data: &str) -> (TOKEN, usize) {
    let next = data.chars().next();
    match next {
        Some('{') => {
            
        }
        Some(_) => { return (TOKEN::Error("Expected {, found {}"), 
        None(_) => {
            return (TOKEN::Error("Expected {, found EOF".to_string()), 0);
        }
    }
}

pub fn lex_number(data: &str) -> (TOKEN, usize) {
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

pub fn seek_past_whitespace(data: &str) -> usize {
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
        Some('"') => index += 1,
        Some(_) => {
            return (TOKEN::Error("expected opening \"".to_string()), 1);
        }
        None => return tok,
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
