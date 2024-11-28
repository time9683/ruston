use crate::{lexer::{Number, Token}, sintax::{Expresion, Literal, Statement}, table::SymbolTable};
use crate::DataType;



#[derive(Debug,PartialEq,Clone)]
enum Err{
  TypeMismatch,  
}



struct Gramatic{
  table:SymbolTable,
  program:Vec<Statement>
}

impl Gramatic{
  pub fn new(table:SymbolTable, program:Vec<Statement>)->Self{
    Gramatic{
      table,
      program
    }
  }




  pub fn infer_type(&mut self,exp:&Expresion) -> Result<DataType,Err>{

    match exp {
      Expresion::Binary(left, op,right) => {
        let left_type = self.infer_type(left)?;
        let right_type = self.infer_type(right)?;
        

        if let Token::Operator(_) = op {
          match (left_type, right_type){
            (DataType::Float,_) | (_,DataType::Float) => Ok(DataType::Float),
             (DataType::Integer,DataType::Integer) => Ok(DataType::Integer),
             _ => Err(Err::TypeMismatch)
             }
        }else if let Token::LogicalOperator(op) = op {
          
          match op.as_str() {
            "&&" | "||" => {
              match (&left_type,&right_type){
                (DataType::Boolean,DataType::Boolean) => Ok(DataType::Boolean),
                 _ => Err(Err::TypeMismatch)
                 }
            }
            _ => {

               if  left_type == right_type {
                  return Ok(DataType::Boolean)
                }else{
                  panic!("Type mismatch the left and right side must have the same type")
                }
            }
          }



        }else{

          panic!("Invalid operator")
        }        
      },
      Expresion::Literal(value) => {
          match value {
            Literal::Number(n) =>{
               match n  {
                Number::Float(_) =>  Ok(DataType::Float),
                Number::Integer(_) => Ok(DataType::Integer),
 
               }
              }
            Literal::Boolean(_) => Ok(DataType::Boolean),
            Literal::String(_) => Ok(DataType::String),
              
          }
      },
      Expresion::Array(vec_exp ) => {
        let mut data_type: Option<DataType> = None;
        for exp in vec_exp {
          let exp_type = self.infer_type(exp)?;

          if  data_type.is_none(){
            data_type = Some(exp_type.clone());
          }

          if  data_type.is_some() && data_type.unwrap() != exp_type {
               return Err(Err::TypeMismatch);
          }
          data_type = Some(exp_type);
        }
        Ok(DataType::Array(Box::new(data_type.unwrap()),vec_exp.len().try_into().unwrap()))
      },

      Expresion::Tuple(vec_exp ) => {
        let mut data_types : Vec<DataType> = vec![];
        for exp in vec_exp {
          let exp_type = self.infer_type(exp)?;
          data_types.push(exp_type);
        }
        Ok(DataType::Tuple(data_types))
      },
      // otras expresiones
      _ => todo!()
    }
    
  }

}



#[cfg(test)]
mod tests{
use crate::{lexer::{Lexer, Token}, sintax::Sintax, table::SymbolKind};

use super::*;


  #[test]
  fn infer_float_types(){
    // simple expresion 1 + 1.5
    let exp   = Expresion::Binary(
      Box::new(Expresion::Literal(Literal::Number(Number::Float(1.2)))),
      Token::Operator("+".to_string()),
      Box::new(Expresion::Literal(Literal::Number(Number::Float(1.5))))
    );
    let  table = SymbolTable::new();
    let mut gramatic = Gramatic::new(table, vec![]);

    let result = gramatic.infer_type(&exp).expect("Type mismatch");
    println!("{:?}", result);

    assert_eq!(result, DataType::Float);
  }

  #[test]
  fn infer_boolean_types(){
    // simple expresion 1 + 1.5
    let exp   = Expresion::Binary(
      Box::new(Expresion::Literal(Literal::Number(Number::Float(1.2)))),
      Token::LogicalOperator("==".to_string()),
      Box::new(Expresion::Literal(Literal::Number(Number::Float(1.4))))
    );
    let  table = SymbolTable::new();
    let mut gramatic = Gramatic::new(table, vec![]);

    let result = gramatic.infer_type(&exp).expect("Type mismatch");
    println!("{:?}", result);

    assert_eq!(result, DataType::Boolean);
  }

  #[test]
  fn infer_boolean_types_2(){
    // simple expresion  ((1 + 2) < 5) && (1 == 2) -> boolean
    let lexer = Lexer::new("((1 + 2) < 5) && (1 == 2);".to_string());
    let mut parser = Sintax::new(lexer);

    parser.parse();

    let exp = &parser.program[0];

    let exp   =  match exp {
      Statement::ExpressionStatement(exp) => exp,
      _ => panic!("Invalid expresion")
    };


    let  table = SymbolTable::new();
    let mut gramatic = Gramatic::new(table, vec![]);

    let result = gramatic.infer_type(&exp).expect("Type mismatch");
    println!("{:?}", result);

    assert_eq!(result, DataType::Boolean);
  }

  #[test]
  fn infer_arr_types(){
    // simple expresion  [1,2,3,4]
    let exp   = Expresion::Array(vec![
      Expresion::Literal(Literal::Number(Number::Float(1.2))),
      Expresion::Literal(Literal::Number(Number::Float(1.5))),
      Expresion::Literal(Literal::Number(Number::Float(1.5))),
      Expresion::Literal(Literal::Number(Number::Float(1.5)))
    ]);
    let  table = SymbolTable::new();
    let mut gramatic = Gramatic::new(table, vec![]);

    let result = gramatic.infer_type(&exp).expect("Type mismatch");
    println!("{:?}", result);

    assert_eq!(result, DataType::Array(Box::new(DataType::Float),4));
  }

  #[test]
  fn infer_bad_arr_types(){
    // this test is to prove that the array must have the same type and the infer_type function must return a type mismatch
    // simple expresion that is a bad array [1,2,3,4, true]
    let exp   = Expresion::Array(vec![
      Expresion::Literal(Literal::Number(Number::Float(1.2))),
      Expresion::Literal(Literal::Number(Number::Float(1.5))),
      Expresion::Literal(Literal::Number(Number::Float(1.5))),
      Expresion::Literal(Literal::Number(Number::Float(1.5))),
      Expresion::Literal(Literal::Boolean(true))
    
    ]);
    let  table = SymbolTable::new();
    let mut gramatic = Gramatic::new(table, vec![]);

    let result = gramatic.infer_type(&exp).unwrap_err();
    println!("{:?}", result);

    assert_eq!(result,Err::TypeMismatch);
  }


  #[test]
  // este test sirve para probar,que la generacion de tipos del compilador sea correcta y que la inferencia de tipos sea correcta e iguales
  fn infer_and_comp_simbol_table(){

    let lexer = Lexer::new("let x : [(int,int),4];".to_string());
    let exp: Expresion = Expresion::Array(vec![
      Expresion::Tuple(vec![
        Expresion::Literal(Literal::Number(Number::Integer(1))),
        Expresion::Literal(Literal::Number(Number::Integer(2)))
      ]),
      Expresion::Tuple(vec![
        Expresion::Literal(Literal::Number(Number::Integer(1))),
        Expresion::Literal(Literal::Number(Number::Integer(2)))
      ]),
      Expresion::Tuple(vec![
        Expresion::Literal(Literal::Number(Number::Integer(1))),
        Expresion::Literal(Literal::Number(Number::Integer(2)))
      ]),
      Expresion::Tuple(vec![
        Expresion::Literal(Literal::Number(Number::Integer(1))),
        Expresion::Literal(Literal::Number(Number::Integer(2)))
      ])
    ]);


    let mut parser = Sintax::new(lexer);
    parser.parse();

    let  table = parser.table;
    let x_data = table.lookup("x").expect("Symbol not found");
    let kind = &x_data.kind;

    let mut gramatic = Gramatic::new(table.clone(), parser.program);
    
    match kind {
      SymbolKind::Variable { data_type } => {

        let final_data = data_type.as_ref().unwrap();

        let result = gramatic.infer_type(&exp).expect("Type mismatch");
        assert_eq!(result, *final_data);
        
      }
      _ => panic!("Invalid data type")
        
    }

  
  }



  #[test]
  fn infer_and_comp_fncall(){
    // esta prueba sirve para probar que los tipos de los argumentos de una funcion sean iguales a los tipos de los parametros de la funcion
    // simple expresion  foo(1,true) para una funcion foo(x:int,y:bool)
    let call = Expresion::FnCall("foo".to_string(),vec![
      Expresion::Literal(Literal::Number(Number::Integer(1))),
      Expresion::Literal(Literal::Boolean(true))
    ]);


    let lexer = Lexer::new("fn foo(x:int,y:bool) -> int { x + y;}".to_string());

    let mut parser = Sintax::new(lexer);

    parser.parse();

    let  table = parser.table;
    let funtion_definiton = table.lookup("foo").expect("Symbol not found");
    let kind = &funtion_definiton.kind;
  
    let mut gramatic = Gramatic::new(table.clone(), parser.program);

    match kind {
      SymbolKind::Function { data_type: _, parameters } => {
        
        if let Expresion::FnCall(_ ,args) = call {

          assert_eq!(args.len(), parameters.len());

          for (arg, param) in args.iter().zip(parameters.iter()) {
            let arg_type = gramatic.infer_type(arg).expect("Type mismatch");
            assert_eq!(arg_type, *param);
          }
            
        }
 
        
      }
      _ => panic!("Invalid data type")
        
    }
    



  

  }


}