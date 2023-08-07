use std::fmt::Debug;

#[derive(PartialEq)]
pub enum SchierkeError {
    ParseError,
    UnknownExpression,
}

impl Debug for SchierkeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SchierkeError::ParseError => write!(f, "Error while parsing passed expression"),
            SchierkeError::UnknownExpression => write!(f, "The following expression is unknown"),
        }
    }
}