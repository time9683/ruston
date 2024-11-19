struct Symbol {
    token: Token,
    value: &str, // Lexeme
    // Unique to identifiers
    occurence: u32, // Line of first occurence
    scope: u32,  // Type of identifier
    use_type: &str, // Type of identifier, either declaration or reference
}

struct SymbolTable {
    symbols: Vec<Symbol>,
}

fn main() {
    let mut table = SymbolTable::new();
    table.push(Symbol::new(Token::Identifier, "x", 1, 1, "declaration"));
    table.push(Symbol::new(Token::Identifier, "x", 2, 1, "reference"));
    db!(table);
}