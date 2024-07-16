#[derive(Debug, Clone)]
pub enum ErrorKind {
    InvalidQuotedName(String),
    InvalidUnquotedName(String),
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::InvalidQuotedName(_) => todo!(),
            ErrorKind::InvalidUnquotedName(_) => todo!(),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    source: Option<Box<dyn std::error::Error>>,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl std::error::Error for Error {
    fn cause(&self) -> Option<&(dyn std::error::Error)> {
        self.source.as_ref().map(|u| u.as_ref())
    }
}

impl Error {
    pub fn invalid_quoted_name(value: String) -> Self {
        Self {
            kind: ErrorKind::InvalidQuotedName(value),
            source: None,
        }
    }

    pub fn invalid_unquoted_name(value: String) -> Self {
        Self {
            kind: ErrorKind::InvalidUnquotedName(value),
            source: None,
        }
    }
}
