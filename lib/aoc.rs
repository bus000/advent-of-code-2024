use std::fmt::Debug;
use nom::Err;

/// Errors that happens in AOC code.
///
/// Can be constructed from IO errors and nom parser errors automatically.
#[derive(Debug, Eq, PartialEq)]
pub enum AocError {

    /// We could not parse the input given.
    ParseInputError(String),

    /// Some IO operation failed.
    IoError(String),

    /// An unexpected error occurred.
    UnexpectedError(String)

}

impl<'a, I> From<Err<I>> for AocError
where
    I: PartialEq + Debug,
{
    fn from(error: Err<I>) -> Self {
        AocError::ParseInputError(format!("Parsing error: {:?}", error))
    }
}

impl From<std::io::Error> for AocError {

    fn from(err: std::io::Error) -> Self {
        AocError::IoError(format!("IO error: {:?}", err))
    }

}

