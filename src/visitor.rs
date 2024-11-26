use crate::sintax::*;
use crate::lexer::{Number, Token};

// Literal {
//   Number(Number),
//   String(String),
//   Boolean(bool),
// }

// pub enum Expresion {
//   Literal(Literal),
//   Identifier(String),
//   Binary(Box<Expresion>, Token, Box<Expresion>),
//   FnCall(String, Vec<Expresion>),
//   Tuple(Vec<Expresion>),
//   Array(Vec<Expresion>),
//   Index(Box<Expresion>, Box<Expresion>),
//   Member(Box<Expresion>, String),
//   TupleIndex(Box<Expresion>, usize),
//   Unary(Token, Box<Expresion>),
//   Range(Box<Expresion>, Box<Expresion>, bool), // bool indica si es inclusivo
// }

// pub enum Statement {
// ExpressionStatement(Expresion),
// Declaration(String, Option<Expresion>),
// Assignment(Expresion, Expresion),
// If(Expresion, Vec<Statement>, Option< Box<Statement>>, u32),
// Loop(Vec<Statement>, u32),
// For(String,Expresion,Vec<Statement>, u32),
// FnDeclaration(String, Vec<String>, Vec<Statement>, u32),
// Return(Option<Expresion>),
// }

pub trait Visitor {
  fn visit_expression_statement(&mut self, expression: &Expresion) -> String;
  fn visit_declaration(&mut self, name: &String, value: &Option<Expresion>) -> String;
  fn visit_assignment(&mut self, left: &Expresion, right: &Expresion) -> String;
  fn visit_if(&mut self, condition: &Expresion, then_branch: &Vec<Statement>, else_branch: &Option<Box<Statement>>, scope_id: u32) -> String;
  fn visit_loop(&mut self, body: &Vec<Statement>, scope_id: u32) -> String;
  fn visit_for(&mut self, variable: &String, iterable: &Expresion, body: &Vec<Statement>, scope_id: u32) -> String;
  fn visit_fn_declaration(&mut self, name: &String, params: &Vec<String>, body: &Vec<Statement>, scope_id: u32) -> String;
  fn visit_return(&mut self, value: &Option<Expresion>) -> String;

  fn visit_literal(&mut self, literal: &Literal) -> String;
  fn visit_identifier(&mut self, identifier: &String) -> String;
  fn visit_binary(&mut self, left: &Expresion, operator: &Token, right: &Expresion) -> String;
  fn visit_fn_call(&mut self, name: &String, args: &Vec<Expresion>) -> String;
  fn visit_array(&mut self, elements: &Vec<Expresion>) -> String;
  fn visit_unary(&mut self, operator: &Token, operand: &Expresion) -> String;
  fn visit_range(&mut self, start: &Expresion, end: &Expresion, inclusive: bool) -> String;

  fn visit_number(&mut self, number: &Number) -> String;
  fn visit_string(&mut self, string: &String) -> String;
  fn visit_boolean(&mut self, boolean: &bool) -> String;
}

pub trait Visitable {
  fn accept(&self, visitor: &mut dyn Visitor) -> String;
}
