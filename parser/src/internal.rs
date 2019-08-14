use nom::{Err, IResult, Needed};

pub type PResult<I, O> = IResult<I, O, ParseError<I, String>>;

/// Error container struct with a vector of input-reason pairs.
#[derive(Debug, PartialEq)]
pub struct ParseError<I, R>(Vec<(I, R)>);

impl<I> ParseError<I, String> {
    pub fn failure<O>(input: I, reason: String) -> PResult<I, O> {
        Err(Err::Failure(ParseError(vec![(input, reason)])))
    }

    pub fn eof<O>(input: I) -> PResult<I, O> {
        Err(Err::Failure(ParseError(vec![(input, "eof".to_string())])))
    }

    pub fn error<O>(input: I, reason: String) -> PResult<I, O> {
        Err(Err::Error(ParseError(vec![(input, reason)])))
    }

    pub fn incomplete<O>(needed: Needed) -> PResult<I, O> {
        Err(Err::Incomplete(needed))
    }
}
