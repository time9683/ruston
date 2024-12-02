// let me get the output without warnings geez

mod lexer;
mod sintax;
mod table;
mod tree_display;
mod visitor;
mod generator;
pub mod semantic;

use lexer::{Lexer,Token};
use dialoguer::{theme::ColorfulTheme,Select};
use sintax::Sintax;
use tree_display::display_tree;
use table::{Symbol,SymbolTable, UseType};
use semantic::Semantic;

use generator:: PythonGenerator;


fn main() {
    // get the path to the file
    let args  =  std::env::args().collect::<Vec<String>>();
    if  args.len() < 2
    {
        // print the usage
        println!("Usage: cargo run <path_to_file>");
        return;
    }
    let path = &args[1];
    // check if the file extension is .rstn
    if  !path.ends_with(".rstn")
    {
        panic!("Invalid file extension");
    }
    // read the file
    let source = std::fs::read_to_string(path).expect("Could not read file");
    // create a lexer
    let mut lexer = Lexer::new(&source);

    if args.len() == 2{
        let mut parser = Sintax::new(lexer.clone());
        parser.parse();
        // display_tree(&parser.program);
        // println!("{:#?}", parser.table);
        let mut generator = PythonGenerator::new(parser.program, parser.table);
        let code = generator.generate();
        println!("{}", code);
        return
    }













    let options = &["Lexical Analysis", "Syntax Analysis", "Semantic Analysis"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select the analysis you want to perform")
        .items(&options[..])
        .default(0)
        .interact()
        .unwrap();
    match selection {
        0 => {
            analysis_lexical(&mut lexer);
        }
        1 => {
            let mut parser = Sintax::new(lexer);
            parser.parse();
            display_tree(&parser.program);
            let table = parser.table;

            println!("{:<20} | {:<20} | {:<20}", "Name", "Type", "Data Type");
            println!("{:-<62}", "");
            for symbol in table.all_scopes {
                for symbol in symbol.1 {
                    println!("{:<20} | {:<20} | {:<20}", symbol.0, format!("{:?}", symbol.1.use_type), symbol.1.kind);
                }
            }
            


        }
        2 => {
            let mut parser = Sintax::new(lexer);
            parser.parse();
            let mut semantic = Semantic::new(parser.program, parser.table);
            semantic.semantic_check();

            let table = semantic.table;

            println!("{:<20} | {:<20} | {:<20}", "Name", "Type", "Data Type");
            println!("{:-<62}", "");
            for symbol in table.all_scopes {
                for symbol in symbol.1 {
                    println!("{:<20} | {:<20} | {:<20}", symbol.0, format!("{:?}", symbol.1.use_type), symbol.1.kind);
                }
            }
            
        }
        _ => {
            println!("Invalid selection");
        }

    }

}


fn get_type_and_value(token: &Token) -> (&str, String) {
    match token {
        Token::ArrowType => ("ArrowType", "->".to_string()),
        Token::In => ("In", "in".to_string()),
        Token::True => ("True", "true".to_string()),
        Token::False => ("False", "false".to_string()),
        Token::Else => ("Else", "else".to_string()),
        Token::LeftParen => ("LeftParen", "(".to_string()),
        Token::RightParen => ("RightParen", ")".to_string()),
        Token::LeftBrace => ("LeftBrace", "{".to_string()),
        Token::RightBrace => ("RightBrace", "}".to_string()),
        Token::LeftBracket => ("LeftBracket", "[".to_string()),
        Token::RightBracket => ("RightBracket", "]".to_string()),
        Token::Comma => ("Comma", ",".to_string()),
        Token::Semicolon => ("Semicolon", ";".to_string()),
        Token::Dot => ("Dot", ".".to_string()),
        Token::Equal => ("Equal", "=".to_string()),
        Token::Identifier(value) => ("Identifier", value.clone()),
        Token::Number(value) => {
            match value {
                lexer::Number::Integer(value) => ("Number:Integer", value.to_string()),
                lexer::Number::Float(value) => ("Number:Float", value.to_string()),
            }
        }
        Token::Operator(value) => ("Operator", value.clone()),
        Token::LogicalOperator(value) => ("LogicalOperator", value.clone()),
        Token::Function => ("Function", "fn".to_string()),
        Token::If => ("If", "if".to_string()),
        Token::For => ("For", "for".to_string()),
        Token::Let => ("Let", "let".to_string()),
        Token::Loop => ("Loop", "loop".to_string()),
        Token::Return => ("Return", "return".to_string()),
        Token::String(value) => ("String", value.clone()),
        Token::EOF => ("EOF", "".to_string()),
        Token::Colon => ("Colon", ":".to_string()),
        Token::TypeInt => ("TypeInt", "int".to_string()),
        Token::TypeFloat => ("TypeFloat", "float".to_string()),
        Token::TypeString => ("TypeString", "string".to_string()),
        Token::TypeBool => ("TypeBool", "bool".to_string()),
        Token::Const => ("Const", "const".to_string()),
        Token::Range => ("Range", "..".to_string()),
        Token::RangeInclusive => ("RangeInclusive", "..=".to_string()),

    }
}

fn analysis_lexical(lexer: &mut Lexer) -> Vec<Token> {
    // loop through the tokens
    let mut tokens = Vec::new();

    loop {
        let token = lexer.get_next_token();
        if token == Token::EOF {
            break;
        }
        tokens.push(token);
    }
    // sort tokens by their type
    tokens.sort_by(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b)));

    // print tokens in a table format
    println!("{:<20} | {:<20}", "Token Type", "Token Value");
    println!("{:-<41}", "");
    for token in &tokens {
        let (token_type, token_value) = get_type_and_value(token);
        println!("{:<20} | {:<20}", token_type, token_value);
    }

    tokens
}