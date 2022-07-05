use crate::Error;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

type SymbolMap = HashMap<String, Symbol>;

/// A struct that represents a symbol in a symbol table.
/// Each symbol has a name, an identifier, a type, and a class
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    id: usize,
    pub symbol_type: SymbolType,
    pub symbol_class: SymbolClass,
}

impl PartialEq<Self> for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.symbol_type == other.symbol_type
            && self.symbol_class == other.symbol_class
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Symbol(name: {}, id: {}, type: {}, class: {})",
            self.name, self.id, self.symbol_type, self.symbol_class
        )
    }
}

/// The type of a symbol, e.g., return type of a function or type of a variable
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SymbolType {
    Void,
    Boolean,
    Integer,
    Float,
    String,
}

impl Display for SymbolType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// The class of a symbol, we track functions, functions' parameters, and variables
#[derive(Debug, Clone, PartialEq)]
pub enum SymbolClass {
    Function { parameters: Vec<Symbol> },
    Parameter,
    Variable,
}

impl Symbol {
    /// Returns true if the symbol refers to a function, otherwise false
    pub fn is_function(&self) -> bool {
        matches!(self.symbol_class, SymbolClass::Function { .. })
    }
}

impl Display for SymbolClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// A SymbolTable tracks the declared symbols and the scopes in which they were declared.
#[derive(Debug)]
pub struct SymbolTable {
    // Track all symbols associated with each scope. The first element represents the global scope,
    // the last element represents the current scope
    scopes: Vec<SymbolMap>,
    // The number of symbols in the table
    num_symbols: usize,
    // The ids of inserted functions, from oldest to newest
    function_ids: Vec<usize>,
    // The return type of the function in which the current scope is defined
    function_type: Option<SymbolType>,
}

impl SymbolTable {
    /// Create an empty SymTab instance. The SymTab initially starts in the global scope without any
    /// defined symbols.
    pub fn new() -> Self {
        SymbolTable::default()
    }

    /// Create and return a new symbol for a function
    pub fn function_symbol(&self, name: String, symbol_type: SymbolType) -> Symbol {
        Symbol {
            name,
            id: self.num_symbols,
            symbol_type,
            symbol_class: SymbolClass::Function { parameters: vec![] },
        }
    }

    /// Create and return a new symbol for a variable
    pub fn variable_symbol(&self, name: String, symbol_type: SymbolType) -> Symbol {
        Symbol {
            name,
            id: self.num_symbols,
            symbol_type,
            symbol_class: SymbolClass::Variable,
        }
    }

    /// Create and return a new symbol for a parameter
    pub fn parameter_symbol(&self, name: String, symbol_type: SymbolType) -> Symbol {
        Symbol {
            name,
            id: self.num_symbols,
            symbol_type,
            symbol_class: SymbolClass::Parameter,
        }
    }

    /// Return the type of the last declared function, aka. its return type
    /// This method is meant to be used for comparing the return type of a function with 
    /// the value returned by a _return_ statement in the function's body.
    pub fn function_type(&self) -> Option<SymbolType> {
        self.function_type
    }

    /// Enter a new scope. All symbols that are added, after a new scope has been entered, will be
    /// associated with the new scope. A SymTab initially starts in the global scope.
    pub fn enter_scope(&mut self) {
        // Create a new empty map for the new scope
        self.scopes.push(HashMap::new());
    }

    /// Leave the current scope and remove all symbols associated with the scope from the symbol table.
    ///
    /// # Panics
    /// This method panics if *leave_scope* is used on the global scope.
    pub fn leave_scope(&mut self) {
        if self.scopes.len() == 1 {
            panic!("Invalid state! Called *leave_scope* on the global scope.");
        }
        // Remove all variables defined in the current scope
        let symbols = self.scopes.pop().unwrap();
        self.num_symbols -= symbols.len();

        // Remove the ids of functions from the id tracker
        symbols.iter().map(|(_, s)| &s.symbol_class).for_each(|s| {
            if let SymbolClass::Function { .. } = s {
                self.function_ids.pop();
                self.function_type = None;
            }
        });
    }

    /// Declare a new symbol in the current scope and add it to the symbol table. If the symbol is a
    /// parameter, it is also added to the parameter list of the last function symbol
    ///
    /// # Error
    /// This method returns an Error result if the given symbol has already been declared in the
    /// current scope. Otherwise, it returns Ok.
    pub fn insert(&mut self, symbol: Symbol) -> Result<(), Error> {
        // Make sure that the symbol has not been declared in the current scope
        let current_scope = self.scopes.last_mut().unwrap();

        let result = if current_scope.contains_key(&symbol.name) {
            Err(Error::Semantic(format!(
                "{} has been defined twice in the current scope ({})",
                symbol,
                self.scopes.len()
            )))
        } else {
            // Add the symbol to the current scope
            current_scope.insert(symbol.name.clone(), symbol.clone());
            self.num_symbols += 1;
            Ok(())
        };

        if result.is_ok() {
            match symbol.symbol_class {
                SymbolClass::Function { .. } => {
                    // If it is a function, we track its id, so that we can later add parameters to it
                    self.function_ids.push(symbol.id);
                    // We also track its return type, so that it can be considered by the parser
                    self.function_type = Some(symbol.symbol_type);
                }
                SymbolClass::Parameter => {
                    // If it is a parameter, we also add it to the last function's parameters
                    let id = *self.function_ids.last().unwrap();
                    let last_function = self
                        .get_by_id_mut(id)
                        .expect("Was not able to find function");

                    if let SymbolClass::Function { parameters } = &mut last_function.symbol_class {
                        parameters.push(symbol);
                    }
                }
                SymbolClass::Variable => {
                    // Nothing to do
                }
            }
        }
        result
    }

    /// Return a reference to the first symbol with the given name.
    ///
    /// This method searches for the first occurrence of a symbol with the given name, starting from
    /// the innermost scope (aka. current scope) and traversing outwards till the global scope.
    /// The method returns _None_ if no symbol with the name is found.
    pub fn get(&self, name: &str) -> Option<&Symbol> {
        for map in self.scopes.iter().rev() {
            if let Some(symbol) = map.get(name) {
                return Some(symbol);
            }
        }
        None
    }

    /// Get a mutable reference to a symbol by its id
    fn get_by_id_mut(&mut self, id: usize) -> Option<&mut Symbol> {
        for map in self.scopes.iter_mut().rev() {
            for symbol in map.values_mut() {
                if symbol.id == id {
                    return Some(symbol);
                }
            }
        }
        None
    }
}

impl Display for SymbolTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut rows = String::new();
        for (id, scope) in self.scopes.iter().enumerate() {
            let mut sorted_values: Vec<&Symbol> = scope.values().collect();
            sorted_values.sort_by(|s1, s2| s1.id.cmp(&s2.id));
            let scope_rows: String = sorted_values
                .iter()
                .map(|v| format!("{}", v))
                .fold(String::new(), |acc, v| format!("{}{}\n", acc, v));

            rows = format!("{}\n+++ Scope: {} +++\n{}", rows, id, scope_rows)
        }
        write!(f, "")
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        SymbolTable {
            scopes: vec![SymbolMap::new()],
            num_symbols: 0,
            function_ids: vec![],
            function_type: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Some utility functions
    fn first_symbol() -> Symbol {
        Symbol {
            name: "First".to_string(),
            id: 0,
            symbol_type: SymbolType::Void,
            symbol_class: SymbolClass::Variable,
        }
    }

    fn second_symbol() -> Symbol {
        Symbol {
            name: "Second".to_string(),
            id: 1,
            symbol_type: SymbolType::Boolean,
            symbol_class: SymbolClass::Function { parameters: vec![] },
        }
    }

    fn third_symbol() -> Symbol {
        Symbol {
            name: "Third".to_string(),
            id: 2,
            symbol_type: SymbolType::Integer,
            symbol_class: SymbolClass::Function { parameters: vec![] },
        }
    }

    #[test]
    fn empty_table() {
        let sym_tab = SymbolTable::default();
        assert_eq!(sym_tab.scopes.len(), 1);
        assert_eq!(sym_tab.scopes.get(0).unwrap().len(), 0)
    }

    #[test]
    fn insert_symbol() {
        let mut sym_tab = SymbolTable::new();
        let symbol = first_symbol();
        sym_tab.insert(symbol.clone()).unwrap();
        assert_eq!(sym_tab.scopes.get(0).unwrap().len(), 1);
        assert_eq!(sym_tab.scopes.get(0).unwrap().get("First"), Some(&symbol));
    }

    #[test]
    fn insert_symbols_same_scope() {
        let mut sym_tab = SymbolTable::new();
        let symbol_a = first_symbol();
        let symbol_b = second_symbol();
        sym_tab.insert(symbol_a.clone()).unwrap();
        sym_tab.insert(symbol_b.clone()).unwrap();
        assert_eq!(sym_tab.scopes.get(0).unwrap().len(), 2);
        assert_eq!(sym_tab.scopes.get(0).unwrap().get("First"), Some(&symbol_a));
        assert_eq!(
            sym_tab.scopes.get(0).unwrap().get("Second"),
            Some(&symbol_b)
        );
    }

    #[test]
    fn insert_symbols_multiple_scopes() {
        let mut sym_tab = SymbolTable::new();
        let symbol_a = first_symbol();
        let symbol_b = second_symbol();
        let symbol_c = third_symbol();

        sym_tab.insert(symbol_a.clone()).unwrap();
        sym_tab.enter_scope();

        sym_tab.insert(symbol_b.clone()).unwrap();
        sym_tab.enter_scope();

        sym_tab.insert(symbol_c.clone()).unwrap();

        assert_eq!(sym_tab.scopes.len(), 3);
        assert_eq!(sym_tab.scopes.get(0).unwrap().len(), 1);
        assert_eq!(sym_tab.scopes.get(1).unwrap().len(), 1);
        assert_eq!(sym_tab.scopes.get(2).unwrap().len(), 1);

        assert_eq!(sym_tab.scopes.get(0).unwrap().get("First"), Some(&symbol_a));
        assert_eq!(
            sym_tab.scopes.get(1).unwrap().get("Second"),
            Some(&symbol_b)
        );
        assert_eq!(sym_tab.scopes.get(2).unwrap().get("Third"), Some(&symbol_c));
    }

    #[test]
    fn leave_scope() {
        let mut sym_tab = SymbolTable::new();

        sym_tab.insert(first_symbol()).unwrap();
        sym_tab.enter_scope();

        sym_tab.insert(second_symbol()).unwrap();
        sym_tab.enter_scope();

        sym_tab.insert(third_symbol()).unwrap();

        assert_eq!(sym_tab.scopes.len(), 3);
        assert_eq!(sym_tab.scopes.get(0).unwrap().len(), 1);
        assert_eq!(sym_tab.scopes.get(1).unwrap().len(), 1);
        assert_eq!(sym_tab.scopes.get(2).unwrap().len(), 1);

        sym_tab.leave_scope();
        assert_eq!(sym_tab.scopes.len(), 2);
        sym_tab.leave_scope();
        assert_eq!(sym_tab.scopes.len(), 1);
    }

    #[test]
    fn lookup() {
        let mut sym_tab = SymbolTable::new();
        let symbol_a = first_symbol();
        let symbol_b = second_symbol();
        let symbol_c = third_symbol();

        sym_tab.insert(symbol_a.clone()).unwrap();
        sym_tab.enter_scope();

        sym_tab.insert(symbol_b.clone()).unwrap();
        sym_tab.enter_scope();

        sym_tab.insert(symbol_c.clone()).unwrap();

        assert_eq!(sym_tab.get("First"), Some(&symbol_a));
        assert_eq!(sym_tab.get("Second"), Some(&symbol_b));
        assert_eq!(sym_tab.get("Third"), Some(&symbol_c));
    }

    #[test]
    fn lookup_closest() {
        let mut sym_tab = SymbolTable::new();
        let symbol_a = first_symbol();
        let symbol_b = second_symbol();
        let symbol_c = third_symbol();

        sym_tab.insert(symbol_a.clone()).unwrap();
        sym_tab.enter_scope();

        let closest = Symbol {
            name: "First".to_string(),
            id: 1,
            symbol_type: SymbolType::Float,
            symbol_class: SymbolClass::Function { parameters: vec![] },
        };

        sym_tab.insert(closest.clone()).unwrap();
        sym_tab.insert(symbol_b).unwrap();
        sym_tab.enter_scope();

        sym_tab.insert(symbol_c).unwrap();

        assert_eq!(sym_tab.get("First"), Some(&closest));

        sym_tab.leave_scope();
        sym_tab.leave_scope();
        assert_eq!(sym_tab.get("First"), Some(&symbol_a));
    }
}
