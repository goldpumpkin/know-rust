use thiserror::Error;

#[derive(Error, Debug)]
pub enum GrepError {
    #[error("Glob Pattern error")]
    GlobPatternError(#[from] glob::GlobError),

    #[error("Regex Pattern error")]
    RegexPatternError(#[from] regex::Error),

    #[error("I/O error")]
    IOError(#[from] std::io::Error),
}