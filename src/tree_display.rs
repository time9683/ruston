use crate::sintax::{Statement, Expresion, Literal};
use crate::lexer::Number;

pub fn display_tree(program: &Vec<Statement>) {
  println!("Program:");
  for statement in program {
    display_statement(statement, 0, true);
  }
}

fn display_statement(statement: &Statement, indent: usize, is_last: bool) {
  let indent_str = " ".repeat(indent);
  let branch = if is_last { "└── " } else { "├── " };
  match statement {
    Statement::ExpressionStatement(expr) => {
      println!("{}{}ExpressionStatement:", indent_str, branch);
      display_expression(expr, indent + 4, true);
    }
    Statement::Declaration(name, expr) => {
      println!("{}{}Declaration: {}", indent_str, branch, name);
      if let Some(expr) = expr {
        display_expression(expr, indent + 4, true);
      }
    }
    Statement::Assignment(lhs, rhs) => {
      println!("{}{}Assignment:", indent_str, branch);
      display_expression(lhs, indent + 4, false);
      display_expression(rhs, indent + 4, true);
    }
    Statement::If(cond, body, else_stmt,_) => {
      println!("{}{}If:", indent_str, branch);
      display_expression(cond, indent + 4, false);
      println!("{}    Body:", indent_str);
      for (i, stmt) in body.iter().enumerate() {
        display_statement(stmt, indent + 8, i == body.len() - 1);
      }
      if let Some(else_stmt) = else_stmt {
        println!("{}    Else:", indent_str);
        display_statement(else_stmt, indent + 4, true);
      }
    }
    Statement::Loop(body,_) => {
      println!("{}{}Loop:", indent_str, branch);
      for (i, stmt) in body.iter().enumerate() {
        display_statement(stmt, indent + 4, i == body.len() - 1);
      }
    }
    Statement::For(var, range, body, _) => {
      println!("{}{}For: {}", indent_str, branch, var);
      display_expression(range, indent + 4, false);
      for (i, stmt) in body.iter().enumerate() {
        display_statement(stmt, indent + 8, i == body.len() - 1);
      }
    }
    Statement::FnDeclaration(name, params, body, _) => {
      println!("{}{}Function Declaration: {}", indent_str, branch, name);
      println!("{}    Parameters: {:?}", indent_str, params);
      for (i, stmt) in body.iter().enumerate() {
        display_statement(stmt, indent + 8, i == body.len() - 1);
      }
    }
    Statement::Return(expr) => {
      println!("{}{}Return:", indent_str, branch);
      if let Some(expr) = expr {
        display_expression(expr, indent + 4, true);
      }
    }
  }
}

fn display_expression(expr: &Expresion, indent: usize, is_last: bool) {
  let indent_str = " ".repeat(indent);
  let branch = if is_last { "└── " } else { "├── " };
  match expr {
    Expresion::Literal(lit) => {
      println!("{}{}Literal: {:?}", indent_str, branch, lit);
    }
    Expresion::Identifier(name) => {
      println!("{}{}Identifier: {}", indent_str, branch, name);
    }
    Expresion::Binary(lhs, op, rhs) => {
      println!("{}{}Binary Expression:", indent_str, branch);
      display_expression(lhs, indent + 4, false);
      println!("{}    Operator: {:?}", indent_str, op);
      display_expression(rhs, indent + 4, true);
    }
    Expresion::FnCall(name, args) => {
      println!("{}{}Function Call: {}", indent_str, branch, name);
      for (i, arg) in args.iter().enumerate() {
        display_expression(arg, indent + 4, i == args.len() - 1);
      }
    }
    Expresion::Tuple(elements) => {
      println!("{}{}Tuple:", indent_str, branch);
      for (i, element) in elements.iter().enumerate() {
        display_expression(element, indent + 4, i == elements.len() - 1);
      }
    }
    Expresion::Array(elements) => {
      println!("{}{}Array:", indent_str, branch);
      for (i, element) in elements.iter().enumerate() {
        display_expression(element, indent + 4, i == elements.len() - 1);
      }
    }
    Expresion::Index(array, index) => {
      println!("{}{}Index:", indent_str, branch);
      display_expression(array, indent + 4, false);
      display_expression(index, indent + 4, true);
    }
    Expresion::Member(expr, member) => {
      println!("{}{}Member Access: {}", indent_str, branch, member);
      display_expression(expr, indent + 4, true);
    }
    Expresion::TupleIndex(expr, index) => {
      println!("{}{}Tuple Index: {}", indent_str, branch, index);
      display_expression(expr, indent + 4, true);
    }
    Expresion::Unary(op, expr) => {
      println!("{}{}Unary Expression:", indent_str, branch);
      println!("{}    Operator: {:?}", indent_str, op);
      display_expression(expr, indent + 4, true);
    }
    Expresion::Range(start, end, inclusive) => {
      println!("{}{}Range:", indent_str, branch);
      display_expression(start, indent + 4, false);
      display_expression(end, indent + 4, false);
      println!("{}    Inclusive: {}", indent_str, inclusive);
    }
  }
}

pub fn print_expression(expr: &Expresion) {
  match expr {
    Expresion::Literal(lit) => {
      match lit {
        // TODO: The display of number is buggy, it prints all as integers
        Literal::Number(number) => {
          print!("{} ", number);
        }
        Literal::Boolean(value) => {
          print!("{} ", value);
        }
        Literal::String(value) => {
          print!("{} ", value);
        }
      }
    }
    Expresion::Identifier(name) => {
      print!("{} ", name);
    }
    Expresion::Binary(lhs, op, rhs) => {
      print_expression(lhs);
      print!("{} ", op);
      print_expression(rhs);
    }
    Expresion::FnCall(name, args) => {
      print!("{} ", name);
      for arg in args {
        print_expression(arg);
      }
    }
    Expresion::Tuple(elements) => {
      print!("(");
      for element in elements {
        print_expression(element);
        print!(", ");
      }
      print!(") ");
    }
    Expresion::Array(elements) => {
      print!("[");
      for element in elements {
        print_expression(element);
        print!(", ");
      }
      print!("] ");
    }
    Expresion::Index(array, index) => {
      print_expression(array);
      print_expression(index);
    }
    Expresion::Member(expr, member) => {
      print_expression(expr);
      print!(".{} ", member);
    }
    Expresion::TupleIndex(expr, index) => {
      print_expression(expr);
      print!("[{}] ", index);
    }
    Expresion::Unary(op, expr) => {
      print!("{} ", op);
      print_expression(expr);
    }
    Expresion::Range(start, end, inclusive) => {
      print_expression(start);
      if *inclusive {
        print!("..=");
      } else {
        print!("..");
      }
      print_expression(end);
    }
  }
}

