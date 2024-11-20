use core::panic;
use std::process::exit;

use crate::lexer::{Lexer, Number, Token};

#[derive(Debug)]
pub enum DataType {
    Integer,
    Float,
    String,
    Boolean,
    Void,
    Array(Box<DataType>),
    Tuple(Vec<DataType>),
    identifier(String),
}


#[derive(Debug)]
pub enum Literal {
    Number(Number),
    String(String),
    Boolean(bool),
}



#[derive(Debug)]
pub enum Expresion {
    Literal(Literal),
    Identifier(String),
    Binary(Box<Expresion>, Token, Box<Expresion>),
    FnCall(String, Vec<Expresion>),
    Tuple(Vec<Expresion>),
    Array(Vec<Expresion>),
    Index(Box<Expresion>, Box<Expresion>),
    Member(Box<Expresion>, String),
    TupleIndex(Box<Expresion>, usize),
    Unary(Token, Box<Expresion>),
    Range(Box<Expresion>, Box<Expresion>, bool), // bool indica si es inclusivo
}

impl Expresion {
    // a function take a token and return the literal value of the token, if the token is not a literal return None
    fn get_literal(token: Token) -> Option<Literal> {
        match token {
            Token::Number(value) => match value {
                Number::Integer(value) => Some(Literal::Number(Number::Integer(value))),
                Number::Float(value) => Some(Literal::Number(Number::Float(value))),
            },
            Token::String(value) => Some(Literal::String(value)),
            Token::True => Some(Literal::Boolean(true)),
            Token::False => Some(Literal::Boolean(false)),
            _ => None,
        }
    }
    
}





#[derive(Debug)]
pub enum Statement {
    ExpressionStatement(Expresion),
    Declaration(String, Option<Expresion>),
    Assignment(Expresion, Expresion),
    If(Expresion, Vec<Statement>, Option< Box<Statement>>),
    Loop(Vec<Statement>),
    For(String,Expresion,Vec<Statement>),
    FnDeclaration(String, Vec<String>, Vec<Statement>),
    Return(Option<Expresion>),
}

pub struct Sintax {
    lexer: Lexer,
    pub program: Vec<Statement>,
}

impl Sintax {
    pub fn new(lexer: Lexer) -> Sintax {
        Sintax {
            lexer,
            program: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        let mut program = Vec::new();
        while self.lexer.peek_token() != Token::EOF {
            program.push(self.parse_statement());
        }
        self.program = program;
        // println!("{:?}", self.program);
    }




    fn parse_statement(&mut self) -> Statement {
        let token = self.lexer.peek_token();
        match token {
            Token::Let | Token::Const => self.parse_declaration(),
            Token::If => self.parse_if(),
            Token::Loop => self.parse_loop(),
            Token::For => self.parse_for_loop(),
            Token::Function => self.func_declaration(),
            Token::Return => self.parse_return(),



            // TODO! recivisar mas tarde,erga se me olvido que queria reviasar :(
            Token::Identifier(_) => {
                // this is a hack to check all the next tokens without consuming them
                let current = self.lexer.save_position();
                self.lexer.get_next_token();
                match self.lexer.peek_token() {
                    Token::Equal => {
                        self.lexer.restore_position(current);
                        self.parse_assignment()
                    }
                    Token::Dot | Token::LeftBracket => {
                        self.lexer.restore_position(current);
                        let expr = self.parse_expresion();
                        if self.lexer.peek_token() == Token::Equal {
                            self.lexer.get_next_token(); // consume '='
                            let value = self.parse_expresion();
                            if self.lexer.get_next_token() == Token::Semicolon {
                                return Statement::Assignment(expr, value);
                            } else {
                                let (line, col) = self.lexer.get_current_position();
                                eprintln!("Expected semicolon at line {} col {}", line, col);
                                exit(1);
                            }
                        } else {
                            let (line, col) = self.lexer.get_current_position();
                            eprintln!("Expected '=' at line {} col {}", line, col);
                            exit(1);
                        }
                    }
                    _ => {
                        self.lexer.restore_position(current);
                        self.parse_expresion_statement()
                    }
                }
            }
            _ => {
                let exp = self.parse_expresion();
                if self.lexer.get_next_token() == Token::Semicolon {
                    return Statement::ExpressionStatement(exp);
                } else {
                    eprintln!("Expected semicolon");
                    exit(1);
                }
            }
        }
    }



    fn parse_tuple(&mut self) -> Expresion {
        let mut elements = Vec::new();
        let x = self.lexer.peek_token();
        println!("{:?}", x);
        while self.lexer.peek_token() != Token::RightParen {
            elements.push(self.parse_expresion());
            match self.lexer.peek_token() {
                Token::Comma => {
                    self.lexer.get_next_token();

                }
                Token::RightParen => {
                    break;
                }
                _ => {
                    let (line,col) =  self.lexer.get_current_position();
                    eprintln!("Expected ',' or ')' at line {} col {}", line, col);
                    exit(1);
                }
            }
        }
        if self.lexer.get_next_token() == Token::RightParen {
            return Expresion::Tuple(elements);
        } else {
            let (line,col) =  self.lexer.get_current_position();
            eprintln!("Expected ')' at line {} col {}", line, col);
            exit(1);
        }
    }



    // TODO! agregar lo de [element; size]
    fn parse_array(&mut self) -> Expresion {
        let mut elements = Vec::new();
        let x = self.lexer.peek_token();
        println!("{:?}", x);
        while self.lexer.peek_token() != Token::RightBracket {
            elements.push(self.parse_expresion());
            match self.lexer.peek_token() {
                Token::Comma => {
                    self.lexer.get_next_token();

                }
                Token::RightBracket => {
                    break;
                }
                _ => {
                    let (line,col) =  self.lexer.get_current_position();
                    eprintln!("Expected ',' or ']' at line {} col {}", line, col);
                    exit(1);
                }
            }
        }
        if self.lexer.get_next_token() == Token::RightBracket {
            return Expresion::Array(elements);
        } else {
            let (line,col) =  self.lexer.get_current_position();
            eprintln!("Expected ']' at line {} col {}", line, col);
            exit(1);
        }
    }




    fn parse_return(&mut self) -> Statement {
        self.lexer.get_next_token(); // consume return
        let exp = if self.lexer.peek_token() == Token::Semicolon {
            None
        } else {
            Some(self.parse_expresion())
        };
        if self.lexer.get_next_token() == Token::Semicolon {
            return Statement::Return(exp);
        } else {
            let (line,col) =  self.lexer.get_current_position();
            eprintln!("Expected semicolon at line {} col {}", line, col);
            exit(1);
        }
    }


    fn func_declaration(&mut self) -> Statement {
        self.lexer.get_next_token(); // consume fn

        // check if the next token is an identifier
        let id = match self.lexer.get_next_token() {
            Token::Identifier(id) => id,
            _ => {
                let (line,col) =  self.lexer.get_current_position();
                eprintln!("Expected identifier at line {} col {}", line, col);
                exit(1);
            }
        };

        // check if the next token is a left paren
        if self.lexer.get_next_token() == Token::LeftParen {
            let mut params = Vec::new();
            if self.lexer.peek_token() != Token::RightParen {
                loop {
                    match self.lexer.get_next_token() {
                        Token::Identifier(id) => {
                            
                            let data_type = self.parse_type();    
                            if data_type.is_none(){
                                let (line,col) =  self.lexer.get_current_position();
                                eprintln!("Expected data type at line {} col {}", line, col);
                                exit(1);
                            }
                        
                        
                            params.push(id)
                        
                        }
                            ,
                        _ => {
                            let (line,col) =  self.lexer.get_current_position();
                            eprintln!("Expected identifier at line {} col {}", line, col);
                            exit(1);
                        }
                    }
                    match self.lexer.peek_token() {
                        Token::Comma => {
                            self.lexer.get_next_token();
                        }
                        Token::RightParen => {
                            self.lexer.get_next_token();
                            break;
                        }
                        _ => {
                            let (line,col) =  self.lexer.get_current_position();
                            eprintln!("Expected ',' or ')' at line {} col {}", line, col);
                            exit(1);
                        }
                    }
                }
            } else {
                self.lexer.get_next_token();
            }

            // check the return type of the function
            let return_type = self.parse_return_type();
            if  return_type.is_none(){
                let (line,col) =  self.lexer.get_current_position();
                eprintln!("Expected '-> type' at line {} col {}", line, col);
                exit(1);
            }

            let block = self.parse_block();
            return Statement::FnDeclaration(id, params, block);
        } else {
            let (line,col) =  self.lexer.get_current_position();
            eprintln!("Expected '(' at line {} col {}", line, col);
            exit(1);
        }





    }


    fn parse_for_loop(&mut self) -> Statement {
        self.lexer.get_next_token(); // for
        
        if  self.lexer.get_next_token() != Token::LeftParen {
            let (line,col) =  self.lexer.get_current_position();
            eprintln!("Expected '(' at line {} col {}", line, col);
            exit(1);
        }

        if let Token::Identifier(id) = self.lexer.get_next_token()  {
            if self.lexer.get_next_token() == Token::In {
                let exp = self.parse_expresion();

                if self.lexer.get_next_token() != Token::RightParen {
                    let (line,col) =  self.lexer.get_current_position();
                    eprintln!("Expected ')' at line {} col {}", line, col);
                    exit(1);
                }



                let block = self.parse_block();
                return Statement::For(id, exp, block);
            }else{
                let (line,col) =  self.lexer.get_current_position();
                eprintln!("Expected 'in' at line {} col {}", line, col);
                exit(1);
            }
            
        }else{
            let (line,col) =  self.lexer.get_current_position();
            eprintln!("Expected identifier at line {} col {}", line, col);
            exit(1);
        }


    }

    fn  parse_loop(&mut self) -> Statement{
        self.lexer.get_next_token(); // consume loop
        let block =  self.parse_block();

        Statement::Loop(block)
    }

    fn parse_if(&mut self) -> Statement {
        self.lexer.get_next_token(); // consume if
        if self.lexer.get_next_token() == Token::LeftParen { // consume (
            let condition = self.parse_expresion();  // consume inner expresion
            if self.lexer.get_next_token() == Token::RightParen { // consume )  
                let block = self.parse_block();
                let else_block = if self.lexer.peek_token() == Token::Else {
                    self.lexer.get_next_token();
                    if self.lexer.peek_token() == Token::If {
                        Some(Box::new(self.parse_if()))
                    } else {
                        Some(
                            Box::new(
                                
                            Statement::If(
                            Expresion::Literal(Literal::Boolean(true)),
                            self.parse_block(),
                            None,
                        )))
                    }
                } else {
                    None
                };
                return Statement::If(condition, block, else_block);
            } else {
                let (line, col) = self.lexer.get_current_position();
                eprintln!("Expected ')' at line {} col {}", line, col);
                exit(1);
            }
        } else {
            let (line, col) = self.lexer.get_current_position();
            eprintln!("Expected '(' at line {} col {}", line, col);
            exit(1);
        }
    }





    fn parse_block(&mut self) -> Vec<Statement> {
        let mut block = Vec::new();
         self.lexer.get_next_token(); // consume  {
        while self.lexer.peek_token() != Token::RightBrace && self.lexer.peek_token() != Token::EOF {
            block.push(self.parse_statement());
        }
        
        if self.lexer.get_next_token() != Token::RightBrace {
            let (line,col) =  self.lexer.get_current_position();
            eprintln!("Expected '}}' at line {} col {}", line, col);
            exit(1);
        }
        block
    }



    fn parse_assignment(&mut self) -> Statement {
        let id = match self.lexer.get_next_token() {
            Token::Identifier(id) => id,
            _ => {
                let (line,col) =  self.lexer.get_current_position();
                eprintln!("Expected identifier at line {} col {}", line, col);
                exit(1);
            }
        };
        if self.lexer.get_next_token() == Token::Equal {
            let exp = self.parse_expresion();
            if self.lexer.get_next_token() == Token::Semicolon {
                return Statement::Assignment(Expresion::Identifier(id), exp);
            } else {
                let (line,col) =  self.lexer.get_current_position();
                eprintln!("Expected semicolon at line {} col {}", line, col);
                exit(1);
            }
        } else {
            let (line,col) =  self.lexer.get_current_position();
            eprintln!("Expected '=' at line {} col {}", line, col);
            exit(1);
        }
    }




    fn parse_declaration(&mut self) -> Statement {
        // if is let
        let is_const: bool =  if  self.lexer.get_next_token() == Token::Const  { true }  else  {false};
        


        let id = self.lexer.get_next_token();


        match id {
            Token::Identifier(id) => {

                //  check if the variable has a data type 
                // 
                let data_type = self.parse_type();

                // check if the variable has an expresion
                let expresion =  if self.lexer.peek_token() == Token::Equal {
                    self.lexer.get_next_token(); // consume '='
                     Some(self.parse_expresion())
                }else{
                    None
                };

  

                    if (is_const && expresion.is_none()) || (is_const && data_type.is_none()) {
                        let (line, col) = self.lexer.get_current_position();
                        eprintln!(
                            "Syntax Error [Line {}, Column {}]: Const declarations must include both a type annotation and an initial value.",
                            line, col
                        );
                        exit(1);
                    }
                

                    if self.lexer.get_next_token() == Token::Semicolon {
                        println!("se ha declarado la variable {} de tipo {:?} y con el valor {:?}", id, data_type, expresion);
                        return Statement::Declaration(id, expresion);
                    } else {
                        let (line,col) =  self.lexer.get_current_position();
                        eprintln!("Expected semicolon at line {} col {}", line, col);
                        exit(1);
                    }
                
            }
            // if is not an identifier
            _ => {
                let (line,col) =  self.lexer.get_current_position();
                let const_str = if is_const { "const" } else { "let" };
                eprintln!(
                    "Syntax error at line {} column {}: expected an identifier after '{}' but found '{}'.",
                    line, col, const_str, id
                );
                exit(1);
            }
        }
    }

    // EBNF Grammar  using the extended Backus-Naur Form
    // the grammar have to define the operator precedence
    // <expresion> ::=  <literal> | <expresion> <operator> <expresion> | <id> {  <acess_or_call>  }
    // <acess_or_call> ::= "." <id> |  "(" <listarg> ")"
    // <literal> ::= <number> | <fncall>
    // <fncall> ::=   <identifier> "("  <arg>?  <listarg> ')'
    // <listarg>  ::=  [ <arg> { , <arg>} ]
    // <number> ::= <integer> | <float>
    // <integer> ::= <digit>+
    // <float> ::= <digit>+ "." <digit>+
    // <operator> ::= "+" | "-" | "*" | "/" | "%" | "**"


    fn parse_expresion_statement(&mut self) -> Statement {
        let exp = self.parse_expresion();
        if self.lexer.get_next_token() == Token::Semicolon {
            return Statement::ExpressionStatement(exp);
        } else {
            let (line,col) =  self.lexer.get_current_position();
            eprintln!("Expected semicolon at line {} col {}", line, col);
            exit(1);
        }
    }



    fn parse_expresion(&mut self) -> Expresion {
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Expresion {
        let mut left = self.parse_logical_and();
        while self.lexer.peek_token() == Token::LogicalOperator("||".to_string()) {
            let operator = self.lexer.get_next_token();
            let right = self.parse_logical_and();
            left = Expresion::Binary(Box::new(left), operator, Box::new(right));
        }
        left
    }

    fn parse_logical_and(&mut self) -> Expresion {
        let mut left = self.parse_comparison();
        while self.lexer.peek_token() == Token::LogicalOperator("&&".to_string()) {
            let operator = self.lexer.get_next_token();
            let right = self.parse_comparison();
            left = Expresion::Binary(Box::new(left), operator, Box::new(right));
        }
        left
    }

    fn parse_comparison(&mut self) -> Expresion {
        let mut left = self.parse_term();
        while self.lexer.peek_token() == Token::LogicalOperator("<".to_string())
            || self.lexer.peek_token() == Token::LogicalOperator(">".to_string())
            || self.lexer.peek_token() == Token::LogicalOperator("<=".to_string())
            || self.lexer.peek_token() == Token::LogicalOperator(">=".to_string())
            || self.lexer.peek_token() == Token::LogicalOperator("==".to_string())
            || self.lexer.peek_token() == Token::LogicalOperator("!=".to_string())
        {
            let operator = self.lexer.get_next_token();
            let right = self.parse_term();
            left = Expresion::Binary(Box::new(left), operator, Box::new(right));
        }
        left
    }




    fn parse_term(&mut self) -> Expresion {
        let mut left = self.parse_factor();
        while self.lexer.peek_token() == Token::Operator("+".to_string()) || self.lexer.peek_token() == Token::Operator("-".to_string()) {
            let operator = self.lexer.get_next_token();
            let right = self.parse_factor();
            left = Expresion::Binary(Box::new(left), operator, Box::new(right));
        }
        left
    }

    fn parse_factor(&mut self) -> Expresion {
        let mut left = self.parse_expoperator();
        while self.lexer.peek_token() == Token::Operator("*".to_string()) || self.lexer.peek_token() == Token::Operator("/".to_string()) || self.lexer.peek_token() == Token::Operator ("%".to_string()) {
            let operator = self.lexer.get_next_token();
            let right = self.parse_literal();
            left = Expresion::Binary(Box::new(left), operator, Box::new(right));
        }

        left
    }

    // this is for exponentiation and precedence
    fn parse_expoperator(&mut self) -> Expresion {
        let mut left = self.parse_unary();
        while self.lexer.peek_token() == Token::Operator("**".to_string()) {
            let operator = self.lexer.get_next_token();
            let right = self.parse_unary();
            left = Expresion::Binary(Box::new(left), operator, Box::new(right));
        }
        left
    }



    fn parse_unary(&mut self) -> Expresion {
        if self.lexer.peek_token() == Token::Operator("-".to_string()) || self.lexer.peek_token() == Token::Operator("!".to_string()) {
            let operator = self.lexer.get_next_token();
            let right = self.parse_unary(); // Recursively parse unary to handle multiple unary operators
            return Expresion::Unary(operator, Box::new(right));
        }
        self.parse_literal()
    }

    fn parse_literal(&mut self) -> Expresion {
        let token = self.lexer.get_next_token();
        // println!("debug {:?}", token);

        match token {
            Token::Number(start) => {
                if self.lexer.peek_token() == Token::Range || self.lexer.peek_token() == Token::RangeInclusive {
                    let inclusive = self.lexer.get_next_token() == Token::RangeInclusive;
                    let end = self.parse_expresion();
                    return Expresion::Range(Box::new(Expresion::Literal(Literal::Number(start))), Box::new(end), inclusive);
                }

                return Expresion::Literal(Literal::Number(start));
            }
            _ => {
                if let Some(literal) = Expresion::get_literal(token.clone()) {
                    return Expresion::Literal(literal);
                }
            }
        }

        println!("debug {:?}", self.lexer.peek_token());
        match token {
            Token::Identifier(id) => match self.lexer.peek_token() {
                Token::LeftParen => {
                    return self.parse_fncall(id);
                }
                Token::Dot => {
                    self.lexer.get_next_token();
                    let mut expr = Expresion::Identifier(id);
                    loop {
                        match self.lexer.get_next_token() {
                            Token::Identifier(member) => {
                                expr = Expresion::Member(Box::new(expr), member);
                            }
                            Token::Number(Number::Integer(index)) => {
                                expr = Expresion::TupleIndex(Box::new(expr), index as usize);
                            }
                            _ => {
                                let (line, col) = self.lexer.get_current_position();
                                eprintln!("Expected identifier or number at line {} col {}", line, col);
                                exit(1);
                            }
                        }
                        if self.lexer.peek_token() != Token::Dot {
                            break;
                        }
                        self.lexer.get_next_token(); // consume '.'
                    }
                    return expr;
                }

                Token::LeftBracket => {
                    self.lexer.get_next_token();
                    let mut expr = Expresion::Identifier(id);
                    loop {
                        let index = self.parse_expresion();
                        if self.lexer.get_next_token() == Token::RightBracket {
                            expr = Expresion::Index(Box::new(expr), Box::new(index));
                            if self.lexer.peek_token() != Token::LeftBracket {
                                break;
                            }
                            self.lexer.get_next_token(); // consume '[' for next index
                        } else {
                            let (line, col) = self.lexer.get_current_position();
                            eprintln!("Expected ']' at line {} col {}", line, col);
                            exit(1);
                        }
                    }
                    return expr;
                }
                _ => {
                    return Expresion::Identifier(id.to_owned());
                }
            },
            Token::LeftBracket => {
                return self.parse_array();
            }
            Token::LeftParen => {
                let current = self.lexer.save_position();
                self.lexer.get_next_token(); // consume '('
                if self.lexer.peek_token() == Token::RightParen {
                    self.lexer.restore_position(current);
                    return self.parse_tuple();
                } else {
                    self.lexer.restore_position(current);
                    self.lexer.get_next_token(); // consume '('
                    return self.parse_expresion();
                }
            }
            _ => {
                let (line, col) = self.lexer.get_current_position();
                eprintln!(
                    "Expected an expression but found '{}' at line {} column {}",
                    token, line, col
                );
                exit(1);
            }
        }
    }

    fn parse_fncall(&mut self, name: String) -> Expresion {
        self.lexer.get_next_token(); // consume (
        let mut args: Vec<Expresion> = Vec::new();

        if self.lexer.peek_token() == Token::RightParen {
            self.lexer.get_next_token(); // consume )
        } else {
            loop {
                args.push(self.parse_expresion());

                match self.lexer.peek_token() {
                    Token::Comma => {
                        self.lexer.get_next_token();
                    }
                    Token::RightParen => {
                        self.lexer.get_next_token();
                        break;
                    }
                    _ => {
                        eprintln!("erga la cagaste feo");
                        exit(1);
                    }
                }
            }
        }

        return Expresion::FnCall(name, args);
    }


    fn parse_type(&mut self) ->  Option<DataType> {
       
        if  self.lexer.peek_token() == Token::Colon {
            self.lexer.get_next_token(); // consume ':'
            match self.lexer.get_next_token() {
                Token::TypeInt => Some(DataType::Integer),
                Token::TypeFloat => Some(DataType::Float),
                Token::TypeString => Some(DataType::String),
                Token::TypeBool => Some(DataType::Boolean),
                _ => {
                    let (line,col) =  self.lexer.get_current_position();
                    eprintln!("Expected data type at line {} col {}", line, col);
                    exit(1);
                }
            }
        } else {
            None
        }
    }

    fn parse_return_type(&mut self) ->  Option<DataType> {
        if  self.lexer.peek_token() == Token::ArrowType {
            self.lexer.get_next_token(); // consume '->'
            match self.lexer.get_next_token() {
                Token::TypeInt => Some(DataType::Integer),
                Token::TypeFloat => Some(DataType::Float),
                Token::TypeString => Some(DataType::String),
                Token::TypeBool => Some(DataType::Boolean),
                _ => {
                    let (line,col) =  self.lexer.get_current_position();
                    eprintln!("Expected data type at line {} col {}", line, col);
                    exit(1);
                }
            }
        } else {
            None
        }
    }


}
