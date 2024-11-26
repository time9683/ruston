use std::fmt::{write, Display};
#[derive(Debug, PartialEq, PartialOrd,Clone)]
pub enum Number {
    Float(f32),
    Integer(i32),
}


impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Float(value) => write!(f, "{}", value),
            Number::Integer(value) => write!(f, "{}", value),
        }
    }
}

impl Eq for Number {}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Number::Float(a), Number::Float(b)) => a.partial_cmp(b).unwrap(),
            (Number::Integer(a), Number::Integer(b)) => a.cmp(b),
            (Number::Float(_), Number::Integer(_)) => std::cmp::Ordering::Greater,
            (Number::Integer(_), Number::Float(_)) => std::cmp::Ordering::Less,
        }
    }
}

// tokens
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord,Clone)]
pub enum Token {
    Function,
    If,
    Else,
    Identifier(String),
    Number(Number),
    String(String),
    For,
    In,
    Loop,
    Let,
    Const,
    Return,
    Operator(String),          // +, -, *, /, %, **
    LogicalOperator(String), // &&, ||, !, ==, !=, <, >, <=, >=
    Equal,
    // CompoundEqual(String), // +=, -=, *=, /=, %=
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Comma,        // ,
    Semicolon,    // ;
    Colon,        // :
    Dot,          // .
    EOF,
    TypeInt, // datatype
    TypeString, // datatype
    TypeFloat, // datatype
    TypeBool, // datatype
    True ,
    False,
    ArrowType, // ->
    Range,
    RangeInclusive,
}


impl  Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Range => write!(f, ".."),
            Token::RangeInclusive => write!(f, "..="),
            Token::ArrowType => write!(f, "->"),
            Token::In => write!(f, "in"),
            Token::False => write!(f, "false"),
            Token::True => write!(f, "true"),
            Token::Else => write!(f, "else"),
            Token::Function => write!(f, "fn"),
            Token::If => write!(f, "if"),
            Token::Identifier(value) => write!(f, "{}", value),
            Token::Number(value) => write!(f, "{}", value),
            Token::String(value) => write!(f, "{}", value),
            Token::For => write!(f, "for"),
            Token::Loop => write!(f, "loop"),
            Token::Let => write!(f, "let"),
            Token::Const => write!(f, "const"),
            Token::Return => write!(f, "return"),
            Token::Operator(value) => write!(f, "{}", value),
            Token::LogicalOperator(value) => write!(f, "{}", value),
            Token::Equal => write!(f, "="),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::LeftBrace => write!(f, "{{"),
            Token::RightBrace => write!(f, "}}"),
            Token::LeftBracket => write!(f, "["),
            Token::RightBracket => write!(f, "]"),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::Colon => write!(f, ":"),
            Token::Dot => write!(f, "."),
            Token::EOF => write!(f, "EOF"),
            Token::TypeInt => write!(f, "int"),
            Token::TypeString => write!(f, "string"),
            Token::TypeFloat => write!(f, "float"),
            Token::TypeBool => write!(f, "bool"),
        }
    }
}











#[derive(Debug, Clone)]
pub struct Lexer {
    source: String,
    current: usize,
    line: usize,
    col: usize,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            source,
            current: 0,
            col: 1,  // column
            line: 1, // row
        }
    }

    pub fn get_current_position(&self) -> (usize, usize) {
        (self.line, self.col)
    }

    pub fn save_position(&self) -> usize {
        self.current
    }

    pub fn restore_position(&mut self, position: usize) {
        self.current = position;
    }


    fn current_char(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.col += 1;

        if let Some('\n') = self.current_char() {
            self.line += 1;
            self.col = 1;
        }

        // return the next character
        self.current_char()
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current + 1)
    }

    fn scan_number(&mut self) -> Number {
        let mut number = String::new();
        let mut is_float = false;

        if let Some('-') = self.current_char() {
            number.push('-');
            self.advance();
        }

        while let Some(c) = self.current_char() {
            if c.is_digit(10) {
            number.push(c);
            self.advance();
            } else if c == '.' {
            if is_float {
                break; // second dot found, stop parsing
            }
            if self.peek() == Some('.') {
                break; // it's a range, not a float
            }
            is_float = true;
            number.push(c);
            self.advance();
            } else {
            break;
            }
        }

        if let Some('e') | Some('E') = self.current_char() {
            is_float = true;
            number.push('e');
            self.advance();

            if let Some('-') | Some('+') = self.current_char() {
                number.push(self.current_char().unwrap());
                self.advance();
            }

            while let Some(c) = self.current_char() {
                if c.is_digit(10) {
                    number.push(c);
                    self.advance();
                } else {
                    break;
                }
            }
        }

        // Check if the next character is alphanumeric, which would make the number invalid
        if let Some(c) = self.current_char() {
            if c.is_alphanumeric() {
                eprintln!(
                    "Error: Invalid number format at line: {}, col: {}",
                    self.line, self.col
                );
                // skip the invalid number
                while let Some(c) = self.current_char() {
                    if !c.is_alphanumeric() {
                        break;
                    }
                    self.advance();
                }
                return Number::Integer(0);
            }
        }

        if is_float {
            Number::Float(number.parse().unwrap())
        } else {
            Number::Integer(number.parse().unwrap())
        }
    }

    fn scan_string(&mut self) -> String {
        let mut string = String::new();
        let mut is_closed = false;
        self.advance(); // consume the opening quote
        while let Some(c) = self.current_char() {
            if c == '"' {
                self.advance(); // consume the closing quote
                is_closed = true;
                break;
            } else {
                string.push(c);
                self.advance();
            }
        }
        if !is_closed {
            // unclosed string print col and line
            eprintln!(
                "Error: Unclosed string at line: {}, col: {}",
                self.line, self.col
            );
            std::process::exit(1);
        }
        string
    }

    fn scan_identifier_keyword(&mut self) -> String {
        let mut identifier = String::new();
        while let Some(c) = self.current_char() {
            if c.is_alphanumeric() || c == '_' {
                identifier.push(c);
                self.advance();
            } else {
                break;
            }
        }
        identifier
    }

    pub fn get_next_token(&mut self) -> Token {
        loop {
            if let Some(c) = self.current_char() {
                match c {
                    ' ' | '\n' | '\r' | '\t' => {
                        self.advance();
                    }
                    '(' => {
                        self.advance();
                        return Token::LeftParen;
                    }
                    ')' => {
                        self.advance();
                        return Token::RightParen;
                    }
                    '{' => {
                        self.advance();
                        return Token::LeftBrace;
                    }
                    '}' => {
                        self.advance();
                        return Token::RightBrace;
                    }
                    '[' => {
                        self.advance();
                        return Token::LeftBracket;
                    }
                    ']' => {
                        self.advance();
                        return Token::RightBracket;
                    }
                    ',' => {
                        self.advance();
                        return Token::Comma;
                    }

                    ':' => {
                        self.advance();
                        return Token::Colon;
                    }

                    ';' => {
                        self.advance();
                        return Token::Semicolon;
                    }
                    '.' => {
                        // print current
                        if self.peek() == Some('.') {
                            self.advance();
                            if self.peek() == Some('=') {
                                self.advance(); // consume .
                                self.advance(); // consume =
                                return Token::RangeInclusive;
                            }
                            self.advance();
                            return Token::Range;
                        }
                        self.advance();
                        return Token::Dot;
                    }
                    '+' | '-' | '*' | '/' | '%' => {
                        // if operator is "/" evaluate if it is a comment  in line
                        if c == '/' && self.peek() == Some('/') {
                            // skip comment line
                            while let Some(c) = self.current_char() {
                                if c == '\n' {
                                    break;
                                }
                                self.advance();
                            }
                            continue;
                        }

                        // if operator is "/" evaluate if it is a comment block
                        if c == '/' && self.peek() == Some('*') {
                            // skip comment block
                            self.advance();
                            self.advance();
                            let mut is_closed = false;
                            while let Some(c) = self.current_char() {
                                if c == '*' && self.peek() == Some('/') {
                                    self.advance();
                                    self.advance();
                                    is_closed = true;
                                    break;
                                }
                                self.advance();
                            }
                            if !is_closed {
                                eprintln!(
                                    "Error: Unclosed comment block at line: {}, col: {}",
                                    self.line, self.col
                                );
                                std::process::exit(1);
                            }
                            continue;
                        }

                        // if operator is "-" evaluate if it is a negative number
                        if c == '-' && self.peek().unwrap().is_digit(10) {
                            return Token::Number(self.scan_number());
                        }

                        if c == '*' && self.peek() == Some('*') {
                            self.advance();
                            self.advance();
                            return Token::Operator("**".to_string());
                        }

                        if c == '-' && self.peek().unwrap() == '>' {
                            self.advance();
                            self.advance();
                            return Token::ArrowType;
                        }

                        self.advance();
                        return Token::Operator(c.to_string());
                    }
                    '&' | '|' | '!' | '=' => {
                        self.advance();
                        if let Some(c2) = self.current_char() {
                            if (c == '&' && c2 == '&')
                                || (c == '|' && c2 == '|')
                                || (c == '=' && c2 == '=')
                                || (c == '!' && c2 == '=')
                            {
                                self.advance();
                                return Token::LogicalOperator(format!("{}{}", c, c2));
                            } else if c == '=' {
                                return Token::Equal;
                            } else {
                                return Token::Operator(c.to_string());
                            }
                        } else if c == '=' {
                            return Token::Equal;
                        } else {
                            return Token::Operator(c.to_string());
                        }
                    }
                    '<' | '>' => {
                        self.advance();
                        if let Some(c2) = self.current_char() {
                            if c2 == '=' {
                                self.advance();
                                return Token::LogicalOperator(format!("{}=", c));
                            } else {
                                return Token::LogicalOperator(c.to_string());
                            }
                        } else {
                            return Token::LogicalOperator(c.to_string());
                        }
                    }
                    '"' => return Token::String(self.scan_string()),
                    _ => {
                        if c.is_digit(10) {
                            return Token::Number(self.scan_number());
                        } else if c.is_alphabetic() || c == '_' {
                            let identifier = self.scan_identifier_keyword();
                            return match identifier.as_str() {
                                "fn" => Token::Function,
                                "if" => Token::If,
                                "for" => Token::For,
                                "let" => Token::Let,
                                "return" => Token::Return,
                                "loop" => Token::Loop,
                                "in" => Token::In,
                                "int" => Token::TypeInt,
                                "string" => Token::TypeString,
                                "float" => Token::TypeFloat,
                                "bool" => Token::TypeBool,
                                "const" => Token::Const,
                                "else" => Token::Else,
                                "true" => Token::True,
                                "false" => Token::False,
                                _ => Token::Identifier(identifier),
                            };
                        } else {
                            panic!("Unexpected character: {}", c);
                        }
                    }
                }
            } else {
                return Token::EOF;
            }
        }
    }

    pub fn peek_token(&mut self) -> Token {
        let mut lexer = self.clone();
        lexer.get_next_token()
    }
}

