use crate::parser::structures::symbol_table::SymbolType;
use crate::SyntaxTree;
use std::fmt::{Display, Formatter};

/// Represents different nodes for a C1 syntax tree
#[derive(Clone, Debug)]
pub enum NodeValue {
    // Virtual root node for the fully-parsed AST
    Root,

    // Literals
    Integer(i32),
    Float(f64),
    Boolean(bool),
    String(String),

    // Other
    Program,
    // The String should hold the name of the variable
    VariableDeclaration(String),
    // The String should hold the name and the SymbolType the type of the variable
    VariableRef(String, SymbolType),
    // Cast into the carried type
    Cast(SymbolType),
    // Parameter with a name
    Parameter(String),

    // Statements
    // Function declaration with a name
    FunctionDeclaration(String),
    // Function call with a name and return type
    FunctionCall(String, SymbolType),
    // Node for representing any kind of sequence, e.g., multiple statements
    Sequence,
    If,
    For,
    DoWhile,
    While,
    Print,
    // Assignment with the type of the value that is assigned
    Assign(SymbolType),
    // Return statement with the type of the returned value
    Return(SymbolType),

    // Expressions
    // Arithmetic operators with the type of the calculated value
    Plus(SymbolType),
    Minus(SymbolType),
    Times(SymbolType),
    Divide(SymbolType),
    UMinus(SymbolType),
    LogOr,
    LogAnd,
    Eq,
    Neq,
    Leq,
    Geq,
    Lst,
    Grt,
}

impl NodeValue {
    /// Get the SymbolType of this NodeValue instance
    pub fn symbol_type(&self) -> SymbolType {
        match self {
            NodeValue::Integer(_) => SymbolType::Integer,
            NodeValue::Float(_) => SymbolType::Float,
            NodeValue::Boolean(_) => SymbolType::Boolean,
            NodeValue::String(_) => SymbolType::String,
            NodeValue::Return(s_type)
            | NodeValue::VariableRef(_, s_type)
            | NodeValue::FunctionCall(_, s_type)
            | NodeValue::Cast(s_type)
            | NodeValue::Assign(s_type)
            | NodeValue::Plus(s_type)
            | NodeValue::Minus(s_type)
            | NodeValue::Times(s_type)
            | NodeValue::Divide(s_type)
            | NodeValue::UMinus(s_type) => *s_type,
            NodeValue::LogOr
            | NodeValue::LogAnd
            | NodeValue::Eq
            | NodeValue::Neq
            | NodeValue::Leq
            | NodeValue::Geq
            | NodeValue::Lst
            | NodeValue::Grt => SymbolType::Boolean,
            _ => SymbolType::Void,
        }
    }

    /// Get the symbol name of the symbol carried by this node. Returns None if there is no symbol
    pub fn symbol_name(&self) -> Option<String> {
        match self {
            NodeValue::VariableDeclaration(name)
            | NodeValue::VariableRef(name, _)
            | NodeValue::Parameter(name)
            | NodeValue::FunctionDeclaration(name)
            | NodeValue::FunctionCall(name, _) => Some(name.clone()),
            _ => None,
        }
    }
}

// Literals
pub fn integer_node(value: i32) -> SyntaxTree<NodeValue> {
    SyntaxTree::new(NodeValue::Integer(value))
}

pub fn float_node(value: f64) -> SyntaxTree<NodeValue> {
    SyntaxTree::new(NodeValue::Float(value))
}

pub fn boolean_node(value: bool) -> SyntaxTree<NodeValue> {
    SyntaxTree::new(NodeValue::Boolean(value))
}

pub fn string_node(value: String) -> SyntaxTree<NodeValue> {
    SyntaxTree::new(NodeValue::String(value))
}

// Other
pub fn program_node() -> SyntaxTree<NodeValue> {
    SyntaxTree::new(NodeValue::Program)
}

pub fn parameter_node(name: String) -> SyntaxTree<NodeValue> {
    SyntaxTree::new(NodeValue::Parameter(name))
}

pub fn variable_node(name: String) -> SyntaxTree<NodeValue> {
    SyntaxTree::new(NodeValue::VariableDeclaration(name))
}

pub fn variable_ref_node(name: String, s_type: SymbolType) -> SyntaxTree<NodeValue> {
    SyntaxTree::new(NodeValue::VariableRef(name, s_type))
}

// Statements
pub fn function_node(name: String) -> SyntaxTree<NodeValue> {
    SyntaxTree::new(NodeValue::FunctionDeclaration(name))
}

pub fn function_call_node(name: String, s_type: SymbolType) -> SyntaxTree<NodeValue> {
    SyntaxTree::new(NodeValue::FunctionCall(name, s_type))
}

pub fn sequence_node() -> SyntaxTree<NodeValue> {
    SyntaxTree::new(NodeValue::Sequence)
}

pub fn if_node() -> SyntaxTree<NodeValue> {
    SyntaxTree::new(NodeValue::If)
}

pub fn for_node() -> SyntaxTree<NodeValue> {
    SyntaxTree::new(NodeValue::For)
}

pub fn do_while_node() -> SyntaxTree<NodeValue> {
    SyntaxTree::new(NodeValue::DoWhile)
}

pub fn while_node() -> SyntaxTree<NodeValue> {
    SyntaxTree::new(NodeValue::While)
}

pub fn print_node() -> SyntaxTree<NodeValue> {
    SyntaxTree::new(NodeValue::Print)
}

pub fn assign_node(symbol_type: SymbolType) -> SyntaxTree<NodeValue> {
    SyntaxTree::new(NodeValue::Assign(symbol_type))
}

pub fn return_node(symbol_type: SymbolType) -> SyntaxTree<NodeValue> {
    SyntaxTree::new(NodeValue::Return(symbol_type))
}

// Expressions
pub fn cast_node(symbol_type: SymbolType) -> SyntaxTree<NodeValue> {
    SyntaxTree::new(NodeValue::Cast(symbol_type))
}

pub fn plus_node(symbol_type: SymbolType) -> SyntaxTree<NodeValue> {
    SyntaxTree::new(NodeValue::Plus(symbol_type))
}

pub fn minus_node(symbol_type: SymbolType) -> SyntaxTree<NodeValue> {
    SyntaxTree::new(NodeValue::Minus(symbol_type))
}

pub fn times_node(symbol_type: SymbolType) -> SyntaxTree<NodeValue> {
    SyntaxTree::new(NodeValue::Times(symbol_type))
}

pub fn divide_node(symbol_type: SymbolType) -> SyntaxTree<NodeValue> {
    SyntaxTree::new(NodeValue::Divide(symbol_type))
}

pub fn u_minus_node(symbol_type: SymbolType) -> SyntaxTree<NodeValue> {
    SyntaxTree::new(NodeValue::UMinus(symbol_type))
}

pub fn log_or_node() -> SyntaxTree<NodeValue> {
    SyntaxTree::new(NodeValue::LogOr)
}

pub fn log_and_node() -> SyntaxTree<NodeValue> {
    SyntaxTree::new(NodeValue::LogAnd)
}

pub fn eq_node() -> SyntaxTree<NodeValue> {
    SyntaxTree::new(NodeValue::Eq)
}

pub fn neq_node() -> SyntaxTree<NodeValue> {
    SyntaxTree::new(NodeValue::Neq)
}

pub fn leq_node() -> SyntaxTree<NodeValue> {
    SyntaxTree::new(NodeValue::Leq)
}

pub fn geq_node() -> SyntaxTree<NodeValue> {
    SyntaxTree::new(NodeValue::Geq)
}

pub fn lst_node() -> SyntaxTree<NodeValue> {
    SyntaxTree::new(NodeValue::Lst)
}

pub fn grt_node() -> SyntaxTree<NodeValue> {
    SyntaxTree::new(NodeValue::Grt)
}

impl Display for NodeValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            NodeValue::Integer(v) => {
                format!("Integer Literal: {}", &v)
            }
            NodeValue::Float(v) => {
                format!("Float Literal: {}", &v)
            }
            NodeValue::Boolean(v) => {
                format!("Boolean Literal: {}", &v)
            }
            NodeValue::String(v) => {
                format!("String Literal: {}", &v)
            }
            NodeValue::FunctionDeclaration(n) => {
                format!("FunctionDeclaration: {}", &n)
            }
            NodeValue::FunctionCall(n, _) => {
                format!("FunctionCall: {}", &n)
            }
            _ => format!("{:?}", self),
        };
        write!(f, "{}", text)
    }
}
