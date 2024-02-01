use thiserror::Error;

/// An error caused during preprocessing of C code
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum PreprocessorError {
    // TODO
}

/// A syntax error while parsing preprocessed C code
#[non_exhaustive]
#[derive(Error, Debug)]
#[error("{inner}")]
pub struct SyntaxError {
    // lang_c::driver::SyntaxError doesn't implement std::error::Error,
    // so we need to do some manual impls
    inner: lang_c::driver::SyntaxError,
}

impl From<lang_c::driver::SyntaxError> for SyntaxError {
    fn from(value: lang_c::driver::SyntaxError) -> Self {
        Self { inner: value }
    }
}

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum Error {
    #[error("preprocessor error")]
    Preprocessor(#[from] PreprocessorError),
    #[error("syntax error")]
    SyntaxError(#[from] SyntaxError),
}
