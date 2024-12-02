use crate::sintax::{Statement, DataType, Expresion, Literal};
use crate::lexer::{Token, Number};
use crate::table::{SymbolTable, SymbolKind};
use crate::tree_display::print_expression;

pub struct Semantic {
    program: Vec<Statement>,
    pub table: SymbolTable,
}

impl Semantic {
    pub fn new(program: Vec<Statement>, table: SymbolTable) -> Self {
        Semantic {
            program,
            table
        }
    }

    pub fn semantic_check(&mut self) {
        let mut error = false;
        // Iterate over all statements in the program
        for statement in &self.program.clone() {
            // Check until a type error is found
            if !self.check_type(statement) {
                error = true;
                break
            }
        }
        // If no type errors were found, print success message
        // TODO: This prints success even if there's a failure in the type checking, probably a recursion thing
        if !error {
            println!("Success: Type checking passed");
        }
    }

    fn check_type(&mut self, statement: &Statement) -> bool {
        // Checking the types involves either checking innermost statements
        // or collecting the types of the contained expressions, to then 
        // check wether they match or not
        let mut type_collection: Vec<DataType> = Vec::new();
        let mut valid = false;
        
        match statement {
            // Check the type of the innermost statements
            Statement::FnDeclaration(_, _, body, _) => {
                for statement in body {
                    // Check until a type error is found
                    valid = self.check_type(statement);
                    if !valid {
                        break;
                    }
                }
                return valid;
            }
            Statement::If(cond, body, else_stmt,_) => {
                // Collect the types of the condition
                type_collection = self.collect_types(cond, type_collection);
                
                if let DataType::Boolean = type_collection[0] {
                    // Validate statements if the condition results in a boolean
                    for statement in body {
                        // Check until a type error is found
                        valid = self.check_type(statement);
                    }
                    if let Some(else_stmt) = else_stmt {
                        valid =  self.check_type(else_stmt);
                    }
                } else {
                    println!("Type Error: Condition must result in a boolean");
                    print_expression(cond);
                    println!();
                }
                return valid;
            }
            Statement::Loop(body,_) => {
                for statement in body {
                    // Check until a type error is found
                    valid = self.check_type(statement);
                    if !valid {
                        break;
                    }
                }
                return valid;
            }
            Statement::For(_, range, body, _) => {
                // The range is given by either a range expression or an array, both of which
                // can be validated by just collecting their types, and checking if they only include
                // integers.
                let mut valid_range = false;
                let types: Vec<DataType> = self.collect_types(range, type_collection);

                match range {
                    Expresion::Range(_, _, _) => {
                        if self.check_collection(types) {
                            valid_range = true;
                        }
                    }
                    Expresion::Array(_) => {
                        if types[0] == DataType::Integer {
                            valid_range = true;
                        }
                    }
                    _ => {}
                }
                
                // Evaluate statements only if range was valid
                if valid_range {
                    for statement in body {
                        // Check until a type error is found
                        valid = self.check_type(statement);
                        if !valid {
                            break;
                        }
                    }
                } else {
                    println!("Type Error: Invalid range, use only integers");
                    println!("{:?}", print_expression(range));
                }

                return valid;
            }
            

            // Collect the types of the contained expressions
            Statement::Return(expr) => {
                if let Some(expr) = expr {
                    type_collection = self.collect_types(expr, type_collection);
                    if !self.check_collection(type_collection.clone()) {
                        println!("Type Error: Mismatching types in statement");
                        print_expression(expr);
                        println!();
                        return false;
                    }
                    return true;
                } else {
                    return true;
                }
            }
            Statement::Declaration(id,  expr) => {
                // Check type for inference
                let id_type = self.collect_id_type(id);

                if id_type != DataType::Void {
                    type_collection.push(id_type.clone());
                }
                
                if let Some(expr) = expr {
                    type_collection = self.collect_types(expr, type_collection);

                    // Infer type if it's not defined
                    if id_type == DataType::Void && type_collection.len() == 1{
                        self.table.update_var_type(id, type_collection[0].clone());
                    }

                    if !self.check_collection(type_collection.clone()) {
                        println!("Type Error: Mismatching types in declaration");
                        print!("let {} = ", id);
                        print_expression(expr);
                        println!();
                        println!("{:?}", type_collection);
                        return false;
                    }
                    // Set assigned to true
                    self.table.update_var_assigned(id);
                    return true;
                } else {
                    return true;
                }
            }
            Statement::Assignment(expr1, expr2) => {
                let mut identifier: &str = "";
                // Validate left expression is an identifier and push its type
                if let Expresion::Identifier(id) = expr1 {
                    identifier = id;
                    let symbol = self.table.read_symbol(id);

                    if let Some(symbol) = symbol {
                        if let SymbolKind::Variable { data_type, .. } = &symbol.kind {
                            if let Some(data_type) = data_type {
                                type_collection.push(data_type.clone());
                            } else {
                                type_collection.push(DataType::Void);
                            }
                        }
                        else {
                            println!("Type Error: Can't assign value to function {}", id);
                            return false;
                        }
                    } else {
                        println!("Type Error: Identifier '{}' not found in symbol table", id);
                        return false;
                    }
                }

                type_collection = self.collect_types(expr2, type_collection);
                if !self.check_collection(type_collection.clone()) {
                    println!("Type Error: Mismatching types in assignment");
                    print_expression(expr1);
                    print!("= ");
                    print_expression(expr2);
                    println!();
                    println!("{:?}", type_collection);
                    return false;
                }
                // Set assigned to true
                self.table.update_var_assigned(identifier);
                return true;
            }
            Statement::ExpressionStatement(expr) => {
                type_collection = self.collect_types(expr, type_collection);
                if !self.check_collection(type_collection.clone()) {
                    println!("Type Error: Mismatching types in expression statement");
                    print_expression(expr);
                    println!();
                    println!("{:?}", type_collection);
                    return false;
                }
                return true;
            }
        }
    }

    fn collect_types(&self, expr: &Expresion, type_collection: Vec<DataType>) -> Vec<DataType> {
        let mut type_collection = type_collection;
        
        match expr {
            // Collect the type of the innermost expressions
            Expresion::Binary(left, token, right) => {
                let mut left_collection: Vec<DataType> = Vec::new();
                let left_type: DataType;
                let mut right_collection: Vec<DataType> = Vec::new();
                let right_type: DataType;
                let mut bin_type: Vec<DataType> = Vec::new();

                // The idea is to recursively collect the types of the expressions, so that each pair 
                // of expressions are added to the collection as a single type if they're valid.

                // WARNING: This recycles a lot of code, but it's necessary as the logic for each type of
                // binary expression is different. This could be refactored into a separate function, but
                // it would be harder to read and maintain.
                match token {
                    Token::Operator(_) => {
                        left_collection = self.collect_types(left, left_collection);
                        // Continue only if the left expression checks out and only has numbers
                        if (&left_collection[0] == &DataType::Integer || &left_collection[0] == &DataType::Float) &&
                            self.check_collection(left_collection.clone()) {
                                left_type = left_collection[0].clone();
                        }
                        else {
                            println!("Type Error: Non-numeric types in arithmetic operation");
                            bin_type.push(DataType::Void);
                            return bin_type;
                        }

                        right_collection = self.collect_types(right, right_collection);
                        // Continue only if the right expression checks out and only has numbers
                        if (&right_collection[0] == &DataType::Integer || &right_collection[0] == &DataType::Float) &&
                            self.check_collection(right_collection.clone()) {
                                right_type = right_collection[0].clone();
                        } else {
                            println!("Type Error: Non-numeric types in arithmetic operation");
                            bin_type.push(DataType::Void);
                            return bin_type;
                        }

                        if left_type == right_type {
                            bin_type.push(left_type);
                        } else {
                            bin_type.push(DataType::Void);
                        }
                    }
                    Token::LogicalOperator(op) => {
                        // Collection for ops like && is different to ==, !=, etc.
                        match op.as_str() {
                            "||" | "&&" => {
                                // Same logic as arithmetic operations, but allows and returns boolean expressions exclusively
                                left_collection = self.collect_types(left, left_collection);
                                // Continue only if the left expression checks out and only has numbers
                                if &left_collection[0] == &DataType::Boolean && 
                                    self.check_collection(left_collection.clone()) {
                                        left_type = left_collection[0].clone();
                                }
                                else {
                                    println!("Type Error: Non-boolean types in boolean operation");
                                    bin_type.push(DataType::Void);
                                    return bin_type;
                                }

                                right_collection = self.collect_types(right, right_collection);
                                // Continue only if the right expression checks out and only has numbers
                                if &right_collection[0] == &DataType::Boolean && 
                                    self.check_collection(right_collection.clone()) {
                                        right_type = right_collection[0].clone();
                                } else {
                                    println!("Type Error: Non-boolean types in boolean operation");
                                    bin_type.push(DataType::Void);
                                    return bin_type;
                                }

                                if left_type == right_type {
                                    bin_type.push(DataType::Boolean);
                                } else {
                                    bin_type.push(DataType::Void);
                                }
                            }
                            _ => {
                                // Same logic as arithmetic operations, but returning bool
                                // Allow only boolean expressions with numeric expressions
                                left_collection = self.collect_types(left, left_collection);
                                // Continue only if the left expression checks out and only has numbers
                                if (&left_collection[0] == &DataType::Integer || &left_collection[0] == &DataType::Float) &&
                                    self.check_collection(left_collection.clone()) {
                                        left_type = left_collection[0].clone();
                                }
                                else {
                                    println!("Type Error: Non-numeric types in boolean comparison");
                                    bin_type.push(DataType::Void);
                                    return bin_type;
                                }

                                right_collection = self.collect_types(right, right_collection);
                                // Continue only if the right expression checks out and only has numbers
                                if (&right_collection[0] == &DataType::Integer || &right_collection[0] == &DataType::Float) &&
                                    self.check_collection(right_collection.clone()) {
                                        right_type = right_collection[0].clone();
                                } else {
                                    println!("Type Error: Non-numeric types in boolean comparison");
                                    bin_type.push(DataType::Void);
                                    return bin_type;
                                }

                                if left_type == right_type {
                                    bin_type.push(DataType::Boolean);
                                } else {
                                    bin_type.push(DataType::Void);
                                }
                            }
                        }
                    }
                    _ => {}
                }
                return bin_type;
            }
            // WARNING: Array and Tuples share the collection of each expression,
            // which could be refactored if it weren't necessary to shortcircuit
            // the complete evaluation of this function if a type check fails
            Expresion::Array(elements) => {
                let mut arr_collection: Vec<DataType> = Vec::new();
                // Collect and validate the type of each expression in the array
                for element in elements {
                    let mut expr_coll: Vec<DataType> = Vec::new();
                    expr_coll = self.collect_types(element, expr_coll);
                    if self.check_collection(expr_coll.clone()) {
                        arr_collection.push(expr_coll[0].clone());
                    }
                    else {
                        // Shortcircuit the evaluation if a type error is found
                        arr_collection.push(DataType::Void);
                        return arr_collection;
                    }
                }

                // Validate the type of the array contents
                if !self.check_collection(arr_collection.clone()) {
                    println!("Type Error: Mismatching types in array");
                    println!("{:?}", arr_collection);
                    return arr_collection;
                }

                // Push the array type with its contained type
                type_collection.push(DataType::Array(Box::new(arr_collection[0].clone()), elements.len() as i32));
            }
            // TODO: Tuples' types need to be validated as well
            Expresion::Tuple(elements) => {
                let mut tup_collection: Vec<DataType> = Vec::new();
                for element in elements {
                    // Validate each element in the tuple
                    let mut expr_coll: Vec<DataType> = Vec::new();
                    expr_coll = self.collect_types(element, expr_coll);
                    if self.check_collection(expr_coll.clone()) {
                        tup_collection.push(expr_coll[0].clone());
                    }
                    else {
                        // Shortcircuit the evaluation if a type error is found
                        tup_collection.push(DataType::Void);
                        return tup_collection;
                    }
                }

                // Push the tuple type with its contained types
                type_collection.push(DataType::Tuple(tup_collection));
            }
            Expresion::Unary(op, expr) => {
                let mut expr_collection: Vec<DataType> = Vec::new();
                // Validate token type
                if &Token::Operator("-".to_string()) == op {
                    expr_collection = self.collect_types(expr, expr_collection);
                    // Return void if the type isn't a number
                    for data_type in expr_collection.clone() {
                        if data_type != DataType::Integer && data_type != DataType::Float {
                            println!("Type Error: Non-numeric type in unary operation");
                            expr_collection.push(DataType::Void);
                            return expr_collection;
                        }
                    }
                } else if &Token::Operator("!".to_string()) == op {
                    type_collection = self.collect_types(expr, type_collection);
                    // Return void if the type isn't a boolean
                    for data_type in type_collection.clone() {
                        if data_type != DataType::Boolean {
                            println!("Type Error: Non-boolean type in unary operation");
                            type_collection.push(DataType::Void);
                            return type_collection;
                        }
                    }
                } else {
                    type_collection.push(DataType::Void);
                    return type_collection;
                }
            }
            Expresion::Range(start, end, _) => {
                type_collection = self.collect_types(start, type_collection);
                type_collection = self.collect_types(end, type_collection);

                // Return void if the types aren't integers
                for data_type in type_collection.clone() {
                    if data_type != DataType::Integer {
                        println!("Type Error: Non-integer type in range");
                        type_collection.push(DataType::Void);
                        return type_collection;
                    }
                }
            }

            // Collect type of the actual terminal expression
            Expresion::Literal(literal) => {
                match literal {
                    Literal::Number(number) => {
                        match number {
                            Number::Integer(_) => {
                                type_collection.push(DataType::Integer);
                            }
                            Number::Float(_) => {
                                type_collection.push(DataType::Float);
                            }
                        }
                    }
                    Literal::String(_) => {
                        type_collection.push(DataType::String);
                    }
                    Literal::Boolean(_) => {
                        type_collection.push(DataType::Boolean);
                    }
                }
            }
            // TODO: Need to validate it's in the same scope as the expression
            // like var_declaration.scope_id <= var_use.scope_id -> True | This won't work, because that'd make it available for all further scopes
            Expresion::Identifier(id) => {
                // Collect the type of the identifier if it's a variable
                let var_type = self.collect_id_type(id);
                let param_type: DataType;
                let assigned: bool;

                // Validate the type of the identifier
                match var_type {
                    // If it's undefined, it's not a variable, so check if it's a param
                    DataType::Undefined => {
                        param_type = self.collect_param_type(id);
                        match param_type {
                            // If it's still undefined, it's not a variable or a param
                            DataType::Undefined => {
                                println!("Type Error: Identifier '{}' not found in symbol table", id);
                                type_collection.push(DataType::Void);
                            }
                            // If it's a param, push the param's type if it's been assigned
                            _ => {
                                type_collection.push(param_type)
                            }
                        }
                    }
                    _ => {
                        assigned = self.check_assignment(id);
                        if assigned {
                            type_collection.push(var_type);
                        } else {
                            println!("Type Error: Identifier '{}' has no value assigned", id);
                            type_collection.push(DataType::Void);
                        }
                    }
                }
            }

            // Collect the type of the actual terminal expression, but need to validate innermost expressions
            Expresion::FnCall(name, args) => {
                // Validate if the function exists in the symbol table
                let fn_type = self.collect_id_type(name);

                // Push the function's type
                match fn_type {
                    DataType::Undefined => {
                        println!("Type Error: Function '{}' not found in symbol table", name);
                        type_collection.push(DataType::Void);
                    }
                    _ => {
                        // Validate the types of the arguments
                        if let Some(params) = self.table.get_params(name) {
                            let mut arg_types: Vec<DataType> = Vec::new();

                            for arg in args {
                                let mut arg_collection: Vec<DataType> = Vec::new();
                                arg_collection = self.collect_types(arg, arg_collection);
                                if !self.check_collection(arg_collection.clone()) {
                                    return arg_collection;
                                }
                                arg_types.push(arg_collection[0].clone());
                            }
                            // Validate the arguments match the function's parameters
                            for i in 0..params.len() {
                                if params[i] != arg_types[i] {
                                    println!("Type Error: Mismatching arguments in function call");
                                    type_collection.push(DataType::Void);
                                    return type_collection;
                                }
                            }

                            type_collection.push(fn_type);
                        }
                    }
                }    
            }
            Expresion::Index(array, index) => {
                // Validate the index type
                let mut index_collection: Vec<DataType> = Vec::new();
                index_collection = self.collect_types(index, index_collection);

                if index_collection.len() == 1 && index_collection[0] == DataType::Integer {
                    // Get the type of the array, if it exists
                    if let Expresion::Identifier(id) = &**array {
                        let array_type = self.collect_id_type(id);
                        if let DataType::Array(data_type, _) = array_type {
                            type_collection.push(*data_type);
                        } else {
                            println!("Type Error: Identifier '{}' is not an array", id);
                            type_collection.push(DataType::Void);
                        }
                    }
                    
                } else {
                    println!("Type Error: Non-integer type in array index");
                    type_collection.push(DataType::Void);
                    return type_collection;
                }
            }
            Expresion::TupleIndex(expr, index) => {
                // Get the ith type of the tuple, if it exists
                if let Expresion::Identifier(id) = &**expr {
                    let tup_type = self.collect_id_type(id);
                    if let DataType::Tuple(data_type) = tup_type {
                        type_collection.push(data_type[*index].clone());
                    } else {
                        println!("Type Error: Identifier '{}' is not an array", id);
                        type_collection.push(DataType::Void);
                    }
                }

            }
            Expresion::Member(_, _) => {
                // If this is about tuples, then the previous match will handle it, right?
            }
        }
        return type_collection;
    }

    fn collect_id_type(&self, id: &String) -> DataType {
        // This function is used to get the type of a variable or function identifier,
        // which will return:
        // - The type of the variable or function
        // - Undefined if it isn't found

        // Get the symbol if its a variable
        let symbol = self.table.read_symbol(id);
        if let Some(symbol) = symbol {
            // Get the type if it's a variable, the rest'll get ignored
            match &symbol.kind {
                SymbolKind::Variable { data_type, .. } => {
                    if let Some(data_type) = data_type {
                        return data_type.clone();
                    }
                    else {
                        return DataType::Void;
                    }
                }
                SymbolKind::Function { data_type, .. } => {
                    if let Some(data_type) = data_type {
                        return data_type.clone();
                    }
                }
            }
        }
        return DataType::Undefined;
    }

    fn check_assignment(&self, id: &String) -> bool {
        // This function is used to check wether a variable has been assigned a value or not

        // Get the symbol if its a variable
        let symbol = self.table.read_symbol(id);
        if let Some(symbol) = symbol {
            // Get the assignment if it's a variable, the rest'll get ignored
            match &symbol.kind {
                SymbolKind::Variable { assigned, .. } => {
                    return assigned.clone();
                }
                _ => {
                    return false;
                }
            }
        }
        return false;
    }
    
    fn collect_param_type(&self, id: &String) -> DataType {
        // This function is used to get the type of a function parameter identifier,
        // which will return:
        // - The type of the parameter if it's a parameter
        // - Undefined if it isn't found

        // Get the symbol if its a function parameter
        let symbols = self.table.get_all_symbols();
        for symbol in symbols {
            // Check the parameters of the functions
            match &symbol.kind {
                SymbolKind::Function { parameters, param_types, .. } => {
                    for (i, param) in parameters.iter().enumerate() {
                        if param == id {
                            return param_types[i].clone();
                        }
                    }
                }
                _ => {}
            }
        }
        DataType::Undefined
    }

    fn check_collection(&self, type_collection: Vec<DataType>) -> bool {
        let mut data_type= &DataType::Void;

        // Check for different types in expression
        for i in 0..type_collection.len() {
            if i == 0 {
                data_type = &type_collection[i];
            } else {
                if data_type != &type_collection[i]  {
                    return false;
                }
            }
        }
        // TODO: This prints both errors, but it should only print the first one

        // Validate no Void type in expression
        if data_type == &DataType::Void {
            println!("Type Error: Void type found in expression. Declare the value or use a valid type");
            return false;
        }
        
        return true;
    }
}