/// Result type for outputs `O`, inputs `I`, and errors `E`.
pub type PResult<O, I, E = ParseErrors<I, String>> = Result<(O, I), E>;

pub type ParseErrors<I, C> = Vec<ParseError<I, C>>;

#[derive(Debug, PartialEq)]
pub struct ParseError<I, C> {
    reason: Reason,
    input: I,
    context: Option<C>,
}

#[derive(Debug, PartialEq)]
pub enum Reason {
    Error,
    Failure,
    Eof,
}

impl<I, C> ParseError<I, C> {
    pub fn new(reason: Reason, input: I, context: Option<C>) -> Self {
        Self {
            reason,
            input,
            context,
        }
    }

    pub fn error<O>(input: I, context: C) -> PResult<O, I, ParseErrors<I, C>> {
        Err(vec![ParseError::new(Reason::Error, input, Some(context))])
    }

    pub fn failure<O>(input: I, context: C) -> PResult<O, I, ParseErrors<I, C>> {
        Err(vec![ParseError::new(Reason::Failure, input, Some(context))])
    }

    pub fn eof<O>(input: I) -> PResult<O, I, ParseErrors<I, C>> {
        Err(vec![ParseError::new(Reason::Failure, input, None)])
    }
}
