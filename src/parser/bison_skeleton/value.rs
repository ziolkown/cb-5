use super::super::syntax_c1::NodeValue;
use crate::parser::bison_skeleton::token::Token;
use crate::parser::structures::symbol_table::SymbolType;
use crate::SyntaxTree;
use std::fmt::{Display, Formatter};

/// Enum that represents all kinds of values that can be returned
/// from parser derivations.
///
/// This values has to be in a single enum, because LALR parsers
/// have a stack, and it's better for it to be heterogeneous.
#[derive(Clone, Debug)]
pub enum Value {
    /// Required variant, parser expects it to be defined
    None,
    /// Required variant, parser expects it to be defined
    Uninitialized,
    /// Required variant, parser expects it to be defined
    Stolen,

    /// Required variant, parser expects it to be defined.
    /// Represents a token that is returned from a Lexer
    Token(Token),

    /// Variant for wrapping SyntaxTrees so that ASTs can be passed between bison rules
    Tree(SyntaxTree<NodeValue>),

    /// Variant for wrapping SymbolTypes so that symbols can be passed between bison rules
    SymbolType(SymbolType),

    /// Variant for wrapping symbol names so that they can be passed between bison rules
    Name(String),
}

impl Default for Value {
    fn default() -> Self {
        Self::Stolen
    }
}

impl Value {
    /// Required method, parser expects it to be defined.
    ///
    /// Constructor for `Value::Token(token)` variant.
    pub(crate) fn from_token(value: Token) -> Self {
        Self::Token(value)
    }

    pub(crate) fn new_uninitialized() -> Self {
        Self::Uninitialized
    }

    pub(crate) fn is_uninitialized(&self) -> bool {
        matches!(self, Self::Uninitialized)
    }

    /// Return the SyntaxTree wrapped by this Value. This method panics if the Value instance is not
    /// a Tree variant.
    pub fn unwrap_tree(self) -> SyntaxTree<NodeValue> {
        match self {
            Value::Tree(tree) => tree,
            _ => {
                panic!("Tried to unwrap {:?} into a SyntaxTree", self)
            }
        }
    }

    /// Return the Token wrapped by this Value. This method panics if the Value instance is not
    /// a Token variant.
    pub fn unwrap_token(self) -> Token {
        match self {
            Value::Token(token) => token,
            _ => {
                panic!("Tried to unwrap {:?} into a Token", self)
            }
        }
    }

    /// Return the String wrapped by this Value. This method panics if the Value instance is not
    /// a Name or Token variant.
    pub fn unwrap_name(self) -> String {
        match self {
            Value::Token(token) => token.text,
            Value::Name(name) => name,
            _ => panic!("Expected symbol name, found {}", self),
        }
    }

    /// Return the SymbolType wrapped by this Value. This method panics if the Value instance is not
    /// a SymbolType variant.
    pub fn unwrap_type(self) -> SymbolType {
        if let Value::SymbolType(t) = self {
            t
        } else {
            panic!("Expected symbol type, found {}", self);
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
