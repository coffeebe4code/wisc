#[derive(Debug, PartialEq)]
pub enum TOKEN {
    Question,
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
    UnderScore,
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

pub const PREPROC_L: &'static [&'static str] =
    &["import", "define", "macro", "test", "release", "debug"];
pub const PREPROC_SIZE_L: &'static [usize] = &[6, 6, 5, 4, 7, 5];
pub const KEYWORDS_L: &'static [&'static str] = &[
    "mut", "const", "i32", "u32", "i64", "i16", "u16", "u8", "i8", "bit", "f64", "f32", "fn", "if",
    "else", "type", "this", "null", "undef", "char", "string", "inline", "static", "switch", "for",
    "in", "of", "break", "enum", "pub", "return", "async", "await", "box", "trait", "ptr", "match",
    "addr", "list", "vol", "true", "false", "func", "function", "void",
];

pub const KEYWORDS_SIZE_L: &'static [usize] = &[
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
    REL,
    DEBUG,
}

impl PREPROC {
    pub fn from_usize(value: usize) -> PREPROC {
        match value {
            0 => PREPROC::IMPORT,
            1 => PREPROC::DEFINE,
            2 => PREPROC::MACRO,
            3 => PREPROC::TEST,
            4 => PREPROC::REL,
            5 => PREPROC::DEBUG,
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
    pub fn from_usize(value: usize) -> KEYWORDS {
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
        '_' => TOKEN::UnderScore,
        '?' => TOKEN::Question,
        c if c.is_alphabetic() => TOKEN::Alpha,
        c if c.is_digit(10) => TOKEN::Digit,
        _ => TOKEN::Error("Invalid token found".to_string()),
    }
}
