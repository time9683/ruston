#[derive(Debug, PartialEq)]
pub enum Number{
Float(f32),
Integer(i32)
}


// tokens
#[derive(Debug, PartialEq)]
pub enum Token{
  Function,
  If,
  Identifier(String),
  Number(Number),
  String(String),
  For,
  Let,
  While,
  Return,
  Operator(char), // +, -, *, /, % 
  LogicalOperator(String), // &&, ||, !, ==, !=, <, >, <=, >=
  Equal,
  // CompoundEqual(String), // +=, -=, *=, /=, %=
  LeftParen, // (
  RightParen, // )
  LeftBrace, // {
  RightBrace, // }
  LeftBracket, // [
  RightBracket, // ]
  Comma, // ,
  Semicolon, // ;
  Dot, // .
  EOF,
  }


pub struct Lexer{
  source: String,
  current: usize,
  line: usize,
  col: usize,
}


impl  Lexer{

  pub fn new (source: String) -> Lexer{
    Lexer{
      source,
      current: 0,
      col: 1, // column
      line: 1, // row
    }
  }

  fn current_char(&self) -> Option<char>{
    self.source.chars().nth(self.current)
  }

  fn advance(&mut self) -> Option<char>{
    self.current += 1;
    self.col += 1;

    if let Some('\n') = self.current_char(){
      self.line += 1;
      self.col = 1;
    }

    // return the next character
    self.current_char()
  }

  fn peek(&self) -> Option<char>{
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

    if is_float {
      Number::Float(number.parse().unwrap())
    } else {
      Number::Integer(number.parse().unwrap())
    }
  }



  fn scan_string(&mut self) -> String{
    let mut string = String::new();
    let mut  is_closed = false;
    self.advance(); // consume the opening quote
    while let Some(c) = self.current_char(){
      if c == '"'{
        self.advance(); // consume the closing quote
        is_closed = true;
        break;
      }else{
        string.push(c);
        self.advance();
      }
    }
    if !is_closed{
      // unclosed string print col and line
      panic!("Unclosed string at line: {}, col: {}", self.line, self.col);
    }
    string
  }

  fn scan_identifier_keyword(&mut self) -> String{
    let mut identifier = String::new();
    while let Some(c) = self.current_char(){
      if c.is_alphanumeric() || c == '_'{
        identifier.push(c);
        self.advance();
      }else{
        break;
      }
    }
    identifier
   
  }


  pub fn get_next_token(&mut self) -> Token{

    loop {
        
    if let Some(c) = self.current_char(){
      match c{

        ' ' | '\n' | '\r' | '\t' => {
           self.advance();
        },
        '(' => {
          self.advance();
          return Token::LeftParen
        },
        ')' => {
          self.advance();
          return Token::RightParen
        },
        '{' => {
          self.advance();
          return Token::LeftBrace
        },
        '}' => {
          self.advance();
          return Token::RightBrace
        },
        '[' => {
          self.advance();
          return Token::LeftBracket
        },
        ']' => {
          self.advance();
          return Token::RightBracket
        },
        ',' => {
          self.advance();
          return Token::Comma
        },
        ';' => {
          self.advance();
          return Token::Semicolon
        },
        '.' => {
          self.advance();
          return Token::Dot
        },
        '+' | '-' | '*' | '/' | '%' => {
          // if operator is "/" evaluate if it is a comment  in line
          if c == '/' && self.peek() == Some('/'){
            // skip comment line
            while let Some(c) = self.current_char(){
              if c == '\n'{
                break;
              }
              self.advance();
            }
            continue;
          } 

          // if operator is "/" evaluate if it is a comment block
          if c == '/' && self.peek() == Some('*'){
            // skip comment block
            self.advance();
            self.advance();
            let mut is_closed = false;
            while let Some(c) = self.current_char(){
              if c == '*' && self.peek() == Some('/'){
                self.advance();
                self.advance();
                is_closed = true;
                break;
              }
              self.advance();
            }
            if !is_closed{
              panic!("Unclosed comment block at line: {}, col: {}", self.line, self.col);
            }
            continue;
          }

          // if operator is "-" evaluate if it is a negative number
          if c == '-' && self.peek().unwrap().is_digit(10){
            return Token::Number(self.scan_number())
          }

          self.advance();
          return Token::Operator(c)
        },
        '&' | '|' | '!' | '=' => {
            self.advance();
            if let Some(c2) = self.current_char(){
            if (c == '&' && c2 == '&') || (c == '|' && c2 == '|') || (c == '=' && c2 == '=') || (c == '!' && c2 == '='){
              self.advance();
              return Token::LogicalOperator(format!("{}{}", c, c2))
            } else if c == '=' {
              return Token::Equal
            } else {
              return Token::Operator(c)
            }
            } else if c == '=' {
            return Token::Equal
            } else {
            return Token::Operator(c)
            }
        },
        '<' | '>' => {
          self.advance();
          if let Some(c2) = self.current_char(){
            if c2 == '='{
              self.advance();
              return Token::LogicalOperator(format!("{}=", c))
            } else {
              return Token::LogicalOperator(c.to_string())
            }
          } else {
            return Token::LogicalOperator(c.to_string())
          }
        },
        '"' => {
          return Token::String(self.scan_string())
        },
        _ => {
          if c.is_digit(10){
            return Token::Number(self.scan_number())
          }else if c.is_alphabetic() || c == '_'{
            let identifier = self.scan_identifier_keyword();
           return  match identifier.as_str(){
              "fn" => Token::Function,
              "if" => Token::If,
              "for" => Token::For,
              "let" => Token::Let,
              "while" => Token::While,
              "return" => Token::Return,
              _ => Token::Identifier(identifier)
            }
          }else{
            panic!("Unexpected character: {}", c);
          }
        }
      }
    }else{
      return Token::EOF
    }
  }
}
  



}