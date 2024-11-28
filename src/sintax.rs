use core::panic;
use std::any::Any;
use std::process::exit;
use crate::visitor::{Visitable, Visitor};
use crate::{Symbol, SymbolTable, UseType};
use crate::table::SymbolKind;
use crate::lexer::{Lexer, Number, Token};

#[derive(Debug, PartialEq, Clone)]
pub enum DataType {
    Integer,
    Float,
    String,
    Boolean,
    Void,
    Array(Box<DataType>),
    Tuple(Vec<DataType>),
    Identifier(String),
}


#[derive(Debug)]
pub enum Literal {
    Number(Number),
    String(String),
    Boolean(bool),
}


impl  Visitable for Literal{
    fn  accept(&self, visitor: &mut dyn Visitor) -> String {
        match  self {
            Literal::Number(number) => visitor.visit_number(number),
            Literal::String(string) => visitor.visit_string(string),
            Literal::Boolean(boolean) => visitor.visit_boolean(boolean),
        }
    }
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

impl Visitable for Expresion{
    
        fn  accept(&self, visitor: &mut dyn Visitor) -> String {
            match self {
                Expresion::Literal(literal) => visitor.visit_literal(literal),
                Expresion::Identifier(identifier) => visitor.visit_identifier(identifier),
                Expresion::Binary(left, operator, right) => visitor.visit_binary(left, operator, right),
                Expresion::FnCall(name, args) => visitor.visit_fn_call(name, args),
                Expresion::Array(elements) => visitor.visit_array(elements),
                Expresion::Unary(operator, operand) => visitor.visit_unary(operator, operand),
                Expresion::Range(start, end, inclusive) => visitor.visit_range(start, end, *inclusive),
                _ => {panic!("no implementado");}
            }
        }
}





#[derive(Debug)]
pub enum Statement {
    ExpressionStatement(Expresion),
    Declaration(String, Option<Expresion>),
    Assignment(Expresion, Expresion),
    If(Expresion, Vec<Statement>, Option< Box<Statement>>, u32),
    Loop(Vec<Statement>, u32),
    For(String,Expresion,Vec<Statement>, u32),
    FnDeclaration(String, Vec<String>, Vec<Statement>, u32),
    Return(Option<Expresion>),
}


impl Visitable for  Statement{

    fn  accept(&self, visitor: &mut dyn Visitor) -> String {
        match self {
            Statement::ExpressionStatement(expression) => visitor.visit_expression_statement(expression),
            Statement::Declaration(id, expression) => visitor.visit_declaration(id, expression),
            Statement::Assignment(left, right) => visitor.visit_assignment(left, right),
            Statement::If(condition, block, else_block, scope_id) => visitor.visit_if(condition, block, else_block, *scope_id),
            Statement::Loop(block, scope_id) => visitor.visit_loop(block, *scope_id),
            Statement::For(id, exp, block, scope_id) => visitor.visit_for(id, exp, block, *scope_id),
            Statement::FnDeclaration(id, params, block, scope_id) => visitor.visit_fn_declaration(id, params, block, *scope_id),
            Statement::Return(exp) => visitor.visit_return(exp),
        }
    }


}




pub struct Sintax {
    lexer: Lexer,
    pub program: Vec<Statement>,
    pub table: SymbolTable,
    current_scope_id: u32,
}

impl Sintax {
    pub fn new(lexer: Lexer) -> Sintax {
        Sintax {
            lexer,
            program: Vec::new(),
            table: SymbolTable::new(),
            current_scope_id: 0,
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
                self.lexer.get_next_token(); // consume identifier
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
            let mut param_types = Vec::new();
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
                            param_types.push(data_type.unwrap());
                            params.push(id);
                        }
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
            if return_type.is_none() {
                let (line,col) =  self.lexer.get_current_position();
                eprintln!("Expected '-> type' at line {} col {}", line, col);
                exit(1);
            }

            let (line,_) = self.lexer.get_current_position();
            self.table.insert(Symbol::function(
                id.clone(),
                line,
                0,
                UseType::Declaration,
                return_type,
                param_types,
            ));

            let scope_id = self.generate_scope_id();
            self.table.create_scope(scope_id);
            self.table.enter_scope(scope_id);
            let block = self.parse_block();
            self.table.exit_scope();
            return Statement::FnDeclaration(id, params, block, scope_id);
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

                let scope_id = self.generate_scope_id();
                self.table.create_scope(scope_id);
                self.table.enter_scope(scope_id);
                let block = self.parse_block();
                self.table.exit_scope();
                return Statement::For(id, exp, block, scope_id);
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
        let scope_id = self.generate_scope_id();
        self.table.create_scope(scope_id);
        self.table.enter_scope(scope_id);
        let block =  self.parse_block();
        self.table.exit_scope();
        Statement::Loop(block, scope_id)
    }

    fn parse_if(&mut self) -> Statement {
        self.lexer.get_next_token(); // consume if
        if self.lexer.get_next_token() == Token::LeftParen { // consume (
            let condition = self.parse_expresion();  // consume inner expresion
            if self.lexer.get_next_token() == Token::RightParen { // consume )  
                let scope_id = self.generate_scope_id();
                self.table.create_scope(scope_id);
                self.table.enter_scope(scope_id);
                let block = self.parse_block();
                self.table.exit_scope();
                let else_block = if self.lexer.peek_token() == Token::Else {
                    self.lexer.get_next_token();
                    if self.lexer.peek_token() == Token::If {
                        Some(Box::new(self.parse_if()))
                    } else {
                        self.table.enter_scope(scope_id);
                        let else_block = Some(Box::new(Statement::If(
                            Expresion::Literal(Literal::Boolean(true)),
                            self.parse_block(),
                            None,
                            scope_id,
                        )));
                        self.table.exit_scope();
                        else_block
                    }
                } else {
                    None
                };
                return Statement::If(condition, block, else_block, scope_id);
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
        let token = self.lexer.get_next_token();
        let id = match token{
            Token::Identifier(id) => id,
            _ => {
                println!("wtf bro {}",token);
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
        let is_const: bool =  if  self.lexer.get_next_token() == Token::Const  { true }  else  {false};
        let id = self.lexer.get_next_token();

        match id {
            Token::Identifier(id) => {
                let data_type = self.parse_type();
                let expresion =  if self.lexer.peek_token() == Token::Equal {
                    self.lexer.get_next_token(); // consume '='
                     Some(self.parse_expresion())
                } else {
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
                
                let (line,col) =  self.lexer.get_current_position();
                if self.lexer.get_next_token() == Token::Semicolon {
                    self.table.insert(Symbol::variable(id.clone(), line, 0, UseType::Declaration, data_type));
                    return Statement::Declaration(id, expresion);
                } else {
                    eprintln!("Expected semicolon at line {} col {}", line, col);
                    exit(1);
                }
            }
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
            let right = self.parse_expoperator();
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
                if self.lexer.peek_token() == Token::RightParen {
                    // esto faltaria por arreglar 
                    return self.parse_tuple();
                } else {
                    let exp =  self.parse_expresion();
                    if self.lexer.get_next_token() == Token::RightParen {
                        return exp;
                    } else {
                        let (line,col) =  self.lexer.get_current_position();
                        eprintln!("Expected ')' at line {} col {}", line, col);
                        exit(1);
                    }
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


    // TODO! add type for array and tuple
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

    fn generate_scope_id(&mut self) -> u32 {
        self.current_scope_id += 1;
        self.current_scope_id
    }
    

    pub fn semantic_check(&mut self) {
        // Iterate over all statements in the program
        for statement in &self.program {
            // Check until a type error is found
            if !self.check_type(statement) {
                return;
            }
        }
        
        println!("Success: Type checking passed");
    }

    fn check_type(&self, statement: &Statement) -> bool {
        // Checking the types involves either checking innermost statements
        // or collecting the types of the contained expressions, to then 
        // check wether they match or not
        let mut type_collection: Vec<DataType> = Vec::new();
        
        match statement {
            // Check the type of the innermost statements
            Statement::FnDeclaration(name, params, body, _) => {
                for statement in body {
                    self.check_type(statement);
                }
            }
            Statement::If(cond, body, else_stmt,_) => {
                for statement in body {
                    self.check_type(statement);
                }
            }
            Statement::Loop(body,_) => {
                for statement in body {
                    self.check_type(statement);
                }
            }
            Statement::For(var, range, body, _) => {
                for statement in body {
                    self.check_type(statement);
                }
            }
            

            // Collect the types of the contained expressions
            Statement::Return(expr) | Statement::Declaration(_,  expr) => {
                if let Some(expr) = expr {
                    type_collection = self.collect_types(expr, type_collection);
                    if !self.check_collection(type_collection) {
                        println!("Type Error: Mismatching types involved in expression {:?}", expr);
                        return false;
                    }
                    return true;
                } else {
                    return true;
                }
            }
            Statement::Assignment(expr1, expr2) => {
                type_collection = self.collect_types(expr1, type_collection);
                type_collection = self.collect_types(expr2, type_collection);
                if !self.check_collection(type_collection) {
                    println!("Type Error: Mismatching types involved in expression {:?}", expr1);
                    return false;
                }
                return true;
            }
            Statement::ExpressionStatement(expr) => {
                type_collection = self.collect_types(expr, type_collection);
                if !self.check_collection(type_collection) {
                    println!("Type Error: Mismatching types involved in expression {:?}", expr);
                    return false;
                }
                return true;
            }
        }

        return true;
    }

    fn collect_types(&self, expr: &Expresion, type_collection: Vec<DataType>) -> Vec<DataType> {
        let mut type_collection = type_collection;
        
        match expr {
            Expresion::Literal(literal) => {
                match literal {
                    Literal::Number(_) => {
                        type_collection.push(DataType::Integer);
                    }
                    Literal::String(_) => {
                        type_collection.push(DataType::String);
                    }
                    Literal::Boolean(_) => {
                        type_collection.push(DataType::Boolean);
                    }
                }
            }
            Expresion::Identifier(id) => {
                let symbol = self.table.lookup(id);
                if let Some(symbol) = symbol {
                    // Get the type if it's a variable, the rest'll get ignored
                    match &symbol.kind {
                        SymbolKind::Variable { data_type } => {
                            if let Some(data_type) = data_type {
                                type_collection.push(data_type.clone());
                            }
                            else {
                                type_collection.push(DataType::Void);
                            }
                        }
                        _ => {}
                    }
                }
            }
            Expresion::Binary(left, _, right) => {
                type_collection = self.collect_types(left, type_collection);
                type_collection = self.collect_types(right, type_collection);
            }
            Expresion::FnCall(name, args) => {
                // Validate if the function exists in the symbol table
                let symbol = self.table.lookup(name);

                // Get the type if it's a function, ignore the rest
                if let Some(symbol) = symbol {
                    match &symbol.kind {
                        SymbolKind::Function { data_type, parameters } => {
                            // Validate the function value
                            if let Some(data_type) = data_type {
                                // Validate the args before pushing the type

                                type_collection.push(data_type.clone());
                            }
                            else {
                                type_collection.push(DataType::Void);
                            }
                        }
                        _ => {}
                    }
                } else {
                    println!("Type Error: Function '{}' not found in symbol table", name);
                    type_collection.push(DataType::Void);
                }
            }
            
        }
        return type_collection;
    }

    fn check_collection(&self, type_collection: Vec<DataType>) -> bool {
        let mut data_type= &DataType::Void;

        for i in 0..type_collection.len() {
            if i == 0 {
                data_type = &type_collection[i];
            } else {
                if data_type != &type_collection[i]  {
                    return false;
                }
            }
        }
        return true;
    }
}



