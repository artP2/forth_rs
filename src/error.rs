use std::fmt::Display;

pub enum ErrorKind {
    UndefinedWordError(String),
    ExecError,
    StackUnderFlowError,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::StackUnderFlowError => write!(f, "Error: stack underflow"),
            ErrorKind::UndefinedWordError(w) => write!(f, "Undefined word: {}", w),
            ErrorKind::ExecError => write!(f, "Exec error"),
        }
    }
}
