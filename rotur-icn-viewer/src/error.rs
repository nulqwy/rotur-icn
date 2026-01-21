use std::fmt;

#[derive(Debug)]
pub enum FailureError {
    OpenFile(std::io::Error),
    ReadFile(std::io::Error),
    DisplayDiagnostics(codespan_reporting::files::Error),
}

impl fmt::Display for FailureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OpenFile(_) => write!(f, "failed to open the specified file"),
            Self::ReadFile(_) => write!(f, "failed to read"),
            Self::DisplayDiagnostics(_) => write!(f, "failed to display diagnostics"),
        }
    }
}

impl std::error::Error for FailureError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::OpenFile(err) => Some(err),
            Self::ReadFile(err) => Some(err),
            Self::DisplayDiagnostics(err) => Some(err),
        }
    }
}
