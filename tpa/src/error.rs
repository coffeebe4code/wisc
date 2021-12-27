use std::fmt;

pub struct Error {
    ekind: ErrorKind,
    base: String,
    start: usize,
    end: usize,
    line: usize,
    file: String,
    expected: String,
    found: String
}

pub enum ErrorKind {
    Token,
    Parser,
    Compiler,
    Type
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", format!("{} {}", self.ekind, self.base));
        }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", match self {
            ErrorKind::Token => "Token Error: ",
            ErrorKind::Parser => "Parser Error: ",
            ErrorKind::Compiler => "Compilation Error: ",
            ErrorKind::Type => "Type Error: "});
        }
    }
