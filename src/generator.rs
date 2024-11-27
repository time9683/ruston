use crate::sintax::{Statement, Expresion, Literal};
use crate::table::SymbolTable;
use crate::visitor::{Visitable, Visitor};
use crate::lexer::{Token, Number};

#[derive(Debug)]
pub struct PythonGenerator {
  programng: Vec<Statement>,
  indent: usize,
  simbol_table: SymbolTable,
  generated_code: String,
}

impl PythonGenerator {
  pub fn new(program: Vec<Statement>, simbol_table: SymbolTable) -> Self {
    Self {
      programng: program,
      indent: 0,
      simbol_table,
      generated_code: String::new(),
    }
  }

  pub fn generate(&mut self) -> String {
    let mut code = String::new();
    let mut visitor = PythonVisitor::new(self.indent);
    for statement in &mut self.programng {
      let generate = statement.accept(&mut visitor);
      code.push_str(&generate);
      code.push('\n');
    }
    code
  }
}

#[derive(Debug, Clone)]
struct PythonVisitor {
  indent: usize,
}

impl PythonVisitor {
  pub fn new(indent: usize) -> Self {
    Self { indent }
  }

  fn increment_indent(&mut self) {
    self.indent += 4;
  }

  fn decrement_indent(&mut self) {
    self.indent -= 4;
  }


}

impl Visitor for PythonVisitor {
  fn visit_expression_statement(&mut self, expression: &Expresion) -> String {
    let exp = expression.accept(self);
    format!("{}{}", " ".repeat(self.indent), exp)
  }

  fn visit_declaration(&mut self, name: &String, value: &Option<Expresion>) -> String {
    match value {
      Some(expr) => format!("{}{} = {}", " ".repeat(self.indent), name, expr.accept(self)),
      None => format!("{}{} = None", " ".repeat(self.indent), name),
    }
  }

  fn visit_assignment(&mut self, left: &Expresion, right: &Expresion) -> String {
    format!("{}{} = {}", " ".repeat(self.indent), left.accept(self), right.accept(self))
  }

  fn visit_if(&mut self, condition: &Expresion, then_branch: &Vec<Statement>, else_branch: &Option<Box<Statement>>, _: u32) -> String {
    let mut code = format!("{}if {}:\n", " ".repeat(self.indent), condition.accept(self));
    self.increment_indent();
    for statement in then_branch {
      code.push_str(&statement.accept(self));
      code.push('\n');
    }
    self.decrement_indent();
    // else bramch can be else or else if
    if let Some(else_branch) = else_branch {
      code.push_str(&format!("{}else:\n", " ".repeat(self.indent)));
      self.increment_indent();
      code.push_str(&else_branch.accept(self));
      self.decrement_indent();
    }
    code
   
  }

  fn visit_loop(&mut self, body: &Vec<Statement>, scope_id: u32) -> String {
    let mut code = format!("{}while True:\n", " ".repeat(self.indent));
    self.increment_indent();
    for statement in body {
      code.push_str(&statement.accept(self));
      code.push('\n');
    }
    self.decrement_indent();
    code
  }

  fn visit_for(&mut self, variable: &String, iterable: &Expresion, body: &Vec<Statement>, scope_id: u32) -> String {
    let mut code = format!("{}for {} in {}:\n", " ".repeat(self.indent), variable, iterable.accept(self));
    self.increment_indent();
    for statement in body {
      code.push_str(&statement.accept(self));
      code.push('\n');
    }
    self.decrement_indent();
    code
   
  }

  fn visit_fn_declaration(&mut self, name: &String, params: &Vec<String>, body: &Vec<Statement>, _: u32) -> String {
    let mut code = format!("{}def {}({}):\n", " ".repeat(self.indent), name, params.join(", "));
    self.increment_indent();
    for statement in body {
      code.push_str(&statement.accept(self));
      code.push('\n');
    }
    self.decrement_indent();
    code

  }

  fn visit_return(&mut self, value: &Option<Expresion>) -> String {
    match value {
      Some(expr) => format!("{}return {}", " ".repeat(self.indent), expr.accept(self)),
      None => format!("{}return", " ".repeat(self.indent)),
    }
  
  }

  fn visit_literal(&mut self, literal: &Literal) -> String {
    match literal {
      Literal::Number(number) => self.visit_number(number),
      Literal::String(string) => self.visit_string(string),
      Literal::Boolean(boolean) => self.visit_boolean(boolean),
    }
    
  }

  fn visit_identifier(&mut self, identifier: &String) -> String {
    identifier.clone()
  }

  fn visit_binary(&mut self, left: &Expresion, operator: &Token, right: &Expresion) -> String {
    let left = left.accept(self);
    let right = right.accept(self);
    format!("({} {} {})", left,to_python_operator(operator), right)
  
  }
  fn visit_fn_call(&mut self, name: &String, args: &Vec<Expresion>) -> String {
    let args = args.iter().map(|arg| arg.accept(self)).collect::<Vec<String>>().join(", ");
    format!("{}({})", name, args)
   
  }

  fn visit_array(&mut self, elements: &Vec<Expresion>) -> String {
    let elements = elements.iter().map(|element| element.accept(self)).collect::<Vec<String>>().join(", ");
    format!("[{}]", elements)
  
  }

  fn visit_unary(&mut self, operator: &Token, operand: &Expresion) -> String {
    let operand = operand.accept(self);
    format!("{}{}",to_python_operator(operator), operand)

  }

  fn visit_range(&mut self, start: &Expresion, end: &Expresion, inclusive: bool) -> String {
    let start = start.accept(self);
    let end = end.accept(self);
    if inclusive {
      // range python inclusive
      format!("range({}, {} + 1)", start, end)
    } else {
      format!("range({}, {})", start, end)
    }
  
  }

  fn visit_number(&mut self, number: &Number) -> String {
    match number {
      Number::Integer(value) => value.to_string(),
      Number::Float(value) => value.to_string(),
    }
  
  }

  fn visit_string(&mut self, string: &String) -> String {
    format!("\"{}\"", string)
  }

  fn visit_boolean(&mut self, boolean: &bool) -> String {
    boolean.to_string()
  }
  
  fn visit_tuple(&mut self, elements: &Vec<Expresion>) -> String {
    let elements = elements.iter().map(|element| element.accept(self)).collect::<Vec<String>>().join(", ");
    format!("({})", elements)
    }
  
  fn visit_index(&mut self, array: &Expresion, index: &Expresion) -> String {
    format!("{}[{}]", array.accept(self), index.accept(self))
    }
  
  fn visit_member(&mut self, object: &Expresion, member: &String) -> String {
    format!("{}.{}", object.accept(self), member)
    }
  
  fn visit_tuple_index(&mut self, tuple: &Expresion, index: usize) -> String {
      format!("{}[{}]", tuple.accept(self), index)
    }
}



fn to_python_operator(operator: &Token) -> String {
  match operator.to_string().as_str() {
    "&&" => "and".to_string(),
    "||" => "or".to_string(),
    "!" => "not ".to_string(),
    _ => operator.to_string(),
  }
}