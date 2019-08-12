pub type IResult<I, O> = Result<(I, O), ParseError<I>>;

#[derive(Debug, PartialEq)]
pub struct ParseError<I> {
    pub input: I,
    pub reason: Reason,
}

impl<I> ParseError<I> {
    pub fn make_return<O>(input: I, reason: String) -> IResult<I, O> {
        Err(ParseError {
            input: input,
            reason: Reason::Return(reason),
        })
    }

    pub fn make_exit<O>(input: I, reason: String) -> IResult<I, O> {
        Err(ParseError {
            input: input,
            reason: Reason::Exit(reason),
        })
    }

    pub fn make_eof<O>(input: I) -> IResult<I, O> {
        Self::make_exit(input, "EOF".to_string())
    }
}

#[derive(Debug, PartialEq)]
pub enum Reason {
    Return(String),
    Exit(String),
}

#[derive(Debug, PartialEq)]
pub struct State<'a> {
    pub source: &'a str,
    pub input: &'a str,
    pub position: usize,
    pub line: usize,
    pub column: usize,
}

impl<'a> State<'a> {
    pub fn from_str(source: &'a str) -> Self {
        Self {
            source: source,
            input: source,
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn from_str_n(source: &'a str, n: usize) -> Self {
        let s = Self::from_str(source);
        s.advance(n)
    }

    pub fn advance(&self, n: usize) -> Self {
        let mut line = self.line;
        let mut column = self.column;

        // Modify line and column number appropriately
        for c in self.source.chars().take(n) {
            if c == '\n' {
                line += 1;
                column = 1;
            } else if c == '\r' {
                // Do nothing
            } else {
                column += 1;
            }
        }

        let p = self.position + n;

        Self {
            input: &self.input[p..],
            position: p,
            line,
            column,
            ..*self
        }
    }
}
