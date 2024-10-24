use std::fmt::Display;

#[derive(Debug)]
pub enum ErrorKind {
    UndefinedWordError(String),
    StackUnderFlowError,
    DivisionByZero,
    UnexpectedToken(String),
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::StackUnderFlowError => write!(f, "Error: stack underflow"),
            ErrorKind::UndefinedWordError(w) => write!(f, "Undefined word: {}", w),
            ErrorKind::DivisionByZero => write!(f, "Error: division by zero"),
            ErrorKind::UnexpectedToken(t) => write!(f, "Error: unexpected token: {}", t),
        }
    }
}
