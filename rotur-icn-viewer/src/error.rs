use std::fmt;

pub const BASE_ERROR_EXIT_CODE: i32 = 90;
pub const EXIT_CODE_FOUND_ERRORS: i32 = BASE_ERROR_EXIT_CODE;
pub const EXIT_CODE_FAILED_OPEN_FILE: i32 = BASE_ERROR_EXIT_CODE + 1;
pub const EXIT_CODE_FAILED_READ_FILE: i32 = BASE_ERROR_EXIT_CODE + 2;
pub const EXIT_CODE_FAILED_WRITE_FILE: i32 = BASE_ERROR_EXIT_CODE + 3;
pub const EXIT_CODE_FAILED_DISPLAY_DIAGNOSTICS: i32 = BASE_ERROR_EXIT_CODE + 4;

#[derive(Debug)]
pub enum FailureError {
    OpenFile(std::io::Error),
    ReadFile(std::io::Error),
    WriteFile(std::io::Error),
    DisplayDiagnostics(codespan_reporting::files::Error),
}

impl fmt::Display for FailureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OpenFile(_) => write!(f, "failed to open the specified file"),
            Self::ReadFile(_) => write!(f, "failed to read from a file"),
            Self::WriteFile(_) => write!(f, "failed to write to a file"),
            Self::DisplayDiagnostics(_) => write!(f, "failed to display diagnostics"),
        }
    }
}

impl std::error::Error for FailureError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::OpenFile(err) => Some(err),
            Self::ReadFile(err) => Some(err),
            Self::WriteFile(err) => Some(err),
            Self::DisplayDiagnostics(err) => Some(err),
        }
    }
}
