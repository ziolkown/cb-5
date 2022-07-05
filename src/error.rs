use crate::parser::syntax_c1::NodeValue;
use crate::SyntaxTree;
use std::fmt::{Display, Formatter};

pub type AnalysisResult = Result<SyntaxTree<NodeValue>, Vec<Error>>;

#[derive(Debug, Clone)]
pub enum Error {
    Lexical(String),
    Syntactical(String),
    Semantic(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Lexical(message) => {
                write!(f, "Lexical Error: {}", message)
            }
            Error::Syntactical(message) => {
                write!(f, "Syntactical Error: {}", message)
            }
            Error::Semantic(message) => {
                write!(f, "Semantic Error: {}", message)
            }
        }
    }
}
