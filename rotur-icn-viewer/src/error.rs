use std::fmt;

pub const BASE_ERROR_EXIT_CODE: i32 = 90;
pub const EXIT_CODE_FOUND_ERRORS: i32 = BASE_ERROR_EXIT_CODE;
pub const EXIT_CODE_FAILED_OPEN_FILE: i32 = BASE_ERROR_EXIT_CODE + 1;
pub const EXIT_CODE_FAILED_READ_FILE: i32 = BASE_ERROR_EXIT_CODE + 2;
pub const EXIT_CODE_FAILED_WRITE_PNG: i32 = BASE_ERROR_EXIT_CODE + 3;
pub const EXIT_CODE_FAILED_DISPLAY_DIAGNOSTICS: i32 = BASE_ERROR_EXIT_CODE + 4;
pub const EXIT_CODE_FAILED_OVERWRITE_CHECK: i32 = BASE_ERROR_EXIT_CODE + 5;
pub const EXIT_CODE_FAILED_OVERWRITE_FORBIDDEN: i32 = BASE_ERROR_EXIT_CODE + 6;

#[derive(Debug)]
pub enum FailureError {
    OpenFile(std::io::Error),
    ReadFile(std::io::Error),
    WritePng(png::EncodingError),
    DisplayDiagnostics(codespan_reporting::files::Error),
    Overwrite(std::io::Error),
    OverwriteForbidden,
}

impl fmt::Display for FailureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OpenFile(_) => write!(f, "failed to open the specified file"),
            Self::ReadFile(_) => write!(f, "failed to read from a file"),
            Self::WritePng(_) => write!(f, "failed to write a PNG"),
            Self::DisplayDiagnostics(_) => write!(f, "failed to display diagnostics"),
            Self::Overwrite(_) => write!(f, "failed to check the overwrite protection"),
            Self::OverwriteForbidden => {
                write!(f, "the destination path already exists; not overwriting")
            }
        }
    }
}

impl std::error::Error for FailureError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::OpenFile(err) | Self::ReadFile(err) | Self::Overwrite(err) => Some(err),
            Self::WritePng(err) => Some(err),
            Self::DisplayDiagnostics(err) => Some(err),
            Self::OverwriteForbidden => None,
        }
    }
}
