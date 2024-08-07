use core::fmt::{Debug, Display, Formatter};

pub type Result<T> = core::result::Result<T, CompilerError>;

// Allow of Box dyn error any returns (primitive anyhow)
pub trait Error: Debug + Display {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub enum CompilerError {
    DiagnosticError,
    DiagnosticWarning,
    ReadSource,
}

impl Error for CompilerError {}

// Rough tmp solution. Implement correctly later
impl Display for CompilerError {
    fn fmt(&self, f: &mut Formatter) -> core::result::Result<(), core::fmt::Error> {
        let message = match self {
            CompilerError::DiagnosticError => "diagnostic errors found",
            CompilerError::DiagnosticWarning => "diagnostic warnings found",
            CompilerError::ReadSource => "failed to read source file",
        };

        write!(f, "compiler error: {}", message)
    }
}
