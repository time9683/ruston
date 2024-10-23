mod lexer;
use lexer::{Lexer,Token};


fn main() {
    // get the path to the file
    let args  =  std::env::args().collect::<Vec<String>>();
    let path = &args[1];
    // check if the file extension is .rstn
    if  !path.ends_with(".rstn")
    {
        panic!("Invalid file extension");
    }
    // read the file
    let source = std::fs::read_to_string(path).expect("Could not read file");
    // create a lexer
    let mut lexer = Lexer::new(source);

    // loop through the tokens
    loop {
        let token = lexer.get_next_token();
        if token == Token::EOF {
            break;
        }
        println!("{:?}", token);
    }

    

}
