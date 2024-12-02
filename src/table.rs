use std::collections::HashMap;

use crate::sintax::DataType;
use std::fmt;

#[derive(Debug, Clone)]
pub enum UseType {
    Declaration,
    Reference,
}

#[derive(Debug, Clone)]
pub enum SymbolKind {
    Variable {
        data_type: Option<DataType>,
        assigned: bool,
    },
    Function {
        data_type: Option<DataType>,
        parameters: Vec<String>,
        param_types: Vec<DataType>,
    },
}

impl fmt::Display for SymbolKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SymbolKind::Variable { data_type, assigned } => {
                match data_type {
                    Some(data) =>{
                        write!(f, "{:?} - Assigned: {}", data, assigned)
                    },
                    None =>{
                        write!(f,"no defined")
                    }
            

                }
            }
            SymbolKind::Function { data_type, parameters, param_types} => {
                write!(f, "Function: {:?} with parameters {:?}", data_type, parameters.iter().zip(param_types.iter()).collect::<Vec<_>>())
            }
        }
    }
}



#[derive(Debug, Clone)]
pub struct Symbol {
    pub value: String, // Lexeme
    pub  occurrence: usize, // Line of first occurrence
    pub scope: u32,  // Scope level
    pub use_type: UseType, // Type of identifier, either declaration or reference
    pub   kind: SymbolKind, // Kind of symbol, either variable or function
}

impl Symbol {
    pub fn variable(value: String, occurrence: usize, scope: u32, use_type: UseType, data_type: Option<DataType>) -> Self {
        Symbol {
            value,
            occurrence,
            scope,
            use_type,
            kind: SymbolKind::Variable { data_type, assigned: false },
        }
    }

    pub fn function(value: String, occurrence: usize, scope: u32, use_type: UseType, data_type: Option<DataType>, parameters: Vec<String>, param_types: Vec<DataType>) -> Self {
        Symbol {
            value,
            occurrence,
            scope,
            use_type,
            kind: SymbolKind::Function { data_type, parameters, param_types },
        }
    }
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    pub all_scopes: HashMap<u32, HashMap<String, Symbol>>, // Cambiar a HashMap con ID de scope
    active_scopes: Vec<u32>, // Pila de IDs de scopes activos
}

impl SymbolTable {
    pub fn new() -> Self {

        let mut table = SymbolTable {
            all_scopes: HashMap::new(),
            active_scopes: vec![0],
        };

        table.create_scope(0); // Crear scope global
        table
    }

    pub fn create_scope(&mut self, scope_id: u32) {
        self.all_scopes.insert(scope_id, HashMap::new()); // Crear nuevo scope con ID
    }

    pub fn enter_scope(&mut self, scope_id: u32) {
        if self.all_scopes.contains_key(&scope_id) {
            self.active_scopes.push(scope_id);
        }
    }

    pub fn exit_scope(&mut self) {
        self.active_scopes.pop();
    }

    pub fn insert(&mut self, symbol: Symbol) {
        if let Some(&current_scope) = self.active_scopes.last() {
            if let Some(scope) = self.all_scopes.get_mut(&current_scope) {
                scope.insert(symbol.value.clone(), symbol);
            }
        }
    }

    pub fn lookup(&self, value: &str) -> Option<&Symbol> {
        for &scope_id in self.active_scopes.iter().rev() {
            if let Some(scope) = self.all_scopes.get(&scope_id) {
                if let Some(symbol) = scope.get(value) {
                    return Some(symbol);
                }
            }
        }
        None
    }

    pub fn get_all_symbols(&self) -> Vec<&Symbol> {
        self.all_scopes.values().flat_map(|scope| scope.values()).collect()
    }

    pub fn get_symbol(&mut self, value: &str) -> Option<&mut Symbol> {
        for scope in self.all_scopes.values_mut() {
            if let Some(symbol) = scope.get_mut(value) {
                return Some(symbol);
            }
        }
        None
    }

    pub fn read_symbol(&self, value: &str) -> Option<&Symbol> {
        for scope in self.all_scopes.values() {
            if let Some(symbol) = scope.get(value) {
                return Some(symbol);
            }
        }
        None
    }

    pub fn update_var_type(&mut self, value: &str, value_type: DataType) {
        if let Some(symbol) = self.get_symbol(value) {
            symbol.kind = SymbolKind::Variable { data_type: Some(value_type), assigned: false };
        }
    }

    pub fn update_var_assigned(&mut self, value: &str) {
        if let Some(symbol) = self.get_symbol(value) {
            if let SymbolKind::Variable { data_type, .. } = &symbol.kind {
                symbol.kind = SymbolKind::Variable { data_type: data_type.clone(), assigned: true };
            }
        }
    }

    pub fn get_params(&self, name: &str) -> Option<&Vec<DataType>> {
        // This functions returns a vector with the parameters of a function
        // in a tuple with the name of the parameter and its data type
        if let Some(symbol) = self.read_symbol(name) {
            if let SymbolKind::Function { param_types, .. } = &symbol.kind {
                return Some(param_types);
            }
        }
        None
    } 
}
