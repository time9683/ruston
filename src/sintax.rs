use crate::lexer::{Lexer, Token};

enum  Expresion{
    Literal(i32),
    Binary(Box<Expresion>, char, Box<Expresion>),
}



enum Statement {
    ExpressionStatement(Expresion),
}





pub struct Sintax {
    lexer: Lexer,
    program: Vec<Statement>
}


impl Sintax{

    pub fn new(lexer: Lexer) -> Sintax{
        Sintax{
            lexer,
            program: Vec::new()
        }
    }

    pub fn parse(&mut self){
        while self.lexer.peek_token() != Token::EOF{
            let token = self.lexer.get_next_token();
            println!("{:?}", token);
        }
    }


}