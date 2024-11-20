use crate::sintax::{Statement, Expresion};

pub fn display_tree(program: &Vec<Statement>) {
  for statement in program {
    display_statement(statement, 0);
  }
}

fn display_statement(statement: &Statement, indent: usize) {
  let indent_str = " ".repeat(indent);
  match statement {
    Statement::ExpressionStatement(expr) => {
      println!("{}ExpressionStatement:", indent_str);
      display_expression(expr, indent + 2);
    }
    Statement::Declaration(name, expr) => {
      println!("{}Declaration: {}", indent_str, name);
      if let Some(expr) = expr {
        display_expression(expr, indent + 2);
      }
    }
    Statement::Assignment(lhs, rhs) => {
      println!("{}Assignment:", indent_str);
      display_expression(lhs, indent + 2);
      display_expression(rhs, indent + 2);
    }
    Statement::If(cond, body, else_stmt) => {
      println!("{}If:", indent_str);
      display_expression(cond, indent + 2);
      println!("{}Body:", indent_str);
      for stmt in body {
        display_statement(stmt, indent + 2);
      }
      if let Some(else_stmt) = else_stmt {
        println!("{}Else:", indent_str);
        display_statement(else_stmt, indent + 2);
      }
    }
    Statement::Loop(body) => {
      println!("{}Loop:", indent_str);
      for stmt in body {
        display_statement(stmt, indent + 2);
      }
    }
    Statement::For(var, range, body) => {
      println!("{}For: {}", indent_str, var);
      display_expression(range, indent + 2);
      for stmt in body {
        display_statement(stmt, indent + 2);
      }
    }
    Statement::FnDeclaration(name, params, body) => {
      println!("{}Function Declaration: {}", indent_str, name);
      println!("{}Parameters: {:?}", indent_str, params);
      for stmt in body {
        display_statement(stmt, indent + 2);
      }
    }
    Statement::Return(expr) => {
      println!("{}Return:", indent_str);
      if let Some(expr) = expr {
        display_expression(expr, indent + 2);
      }
    }
  }
}

fn display_expression(expr: &Expresion, indent: usize) {
  let indent_str = " ".repeat(indent);
  match expr {
    Expresion::Literal(lit) => {
      println!("{}Literal: {:?}", indent_str, lit);
    }
    Expresion::Identifier(name) => {
      println!("{}Identifier: {}", indent_str, name);
    }
    Expresion::Binary(lhs, op, rhs) => {
      println!("{}Binary Expression:", indent_str);
      display_expression(lhs, indent + 2);
      println!("{}Operator: {:?}", indent_str, op);
      display_expression(rhs, indent + 2);
    }
    Expresion::FnCall(name, args) => {
      println!("{}Function Call: {}", indent_str, name);
      for arg in args {
        display_expression(arg, indent + 2);
      }
    }
    Expresion::Tuple(elements) => {
      println!("{}Tuple:", indent_str);
      for element in elements {
        display_expression(element, indent + 2);
      }
    }
    Expresion::Array(elements) => {
      println!("{}Array:", indent_str);
      for element in elements {
        display_expression(element, indent + 2);
      }
    }
    Expresion::Index(array, index) => {
      println!("{}Index:", indent_str);
      display_expression(array, indent + 2);
      display_expression(index, indent + 2);
    }
    Expresion::Member(expr, member) => {
      println!("{}Member Access: {}", indent_str, member);
      display_expression(expr, indent + 2);
    }
    Expresion::TupleIndex(expr, index) => {
      println!("{}Tuple Index: {}", indent_str, index);
      display_expression(expr, indent + 2);
    }
    Expresion::Unary(op, expr) => {
      println!("{}Unary Expression:", indent_str);
      println!("{}Operator: {:?}", indent_str, op);
      display_expression(expr, indent + 2);
    }
    Expresion::Range(start, end, inclusive) => {
      println!("{}Range:", indent_str);
      display_expression(start, indent + 2);
      display_expression(end, indent + 2);
      println!("{}Inclusive: {}", indent_str, inclusive);
    }
  }
}



