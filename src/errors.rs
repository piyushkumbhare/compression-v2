use std::error::Error;

use strum::Display;

// TODO: Move this to a different file

#[derive(Debug, Display)]
pub enum CompressError<'a> {
    ParseError(&'a str),
    BwtError(&'a str),
    MtfError(&'a str),
    RleError(&'a str),
    HuffError(&'a str),
}

impl Error for CompressError<'_> {}