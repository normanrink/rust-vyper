use crate::utils::{
    char_offset,
};

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

#[derive(Debug, PartialEq, Clone)]
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
        let state = Self::from_str(source);
        state.advance(n)
    }

    /// Advance parsing cursor by `n` chars.
    pub fn advance(&self, n: usize) -> Self {
        let mut line = self.line;
        let mut column = self.column;

        // Modify line and column number appropriately
        let mut local_position = 0;
        for chr in self.source.chars().take(n) {
            if chr == '\n' {
                line += 1;
                column = 1;
            } else if chr == '\r' {
                // Don't modify line or column for carriage return
            } else {
                column += 1;
            }
            local_position += 1;
        }

        let off = char_offset(self.input, local_position).unwrap();

        Self {
            input: &self.input[off..],
            position: self.position + local_position,
            line,
            column,
            ..*self
        }
    }

    /// Advance parsing cursor to end of file.
    pub fn advance_to_end(&self) -> Self {
        let mut line = self.line;
        let mut column = self.column;

        // Modify line and column number appropriately
        let mut local_position = 0;
        for chr in self.source.chars() {
            if chr == '\n' {
                line += 1;
                column = 1;
            } else if chr == '\r' {
                // Don't modify line or column for carriage return
            } else {
                column += 1;
            }
            local_position += 1;
        }

        Self {
            input: &self.input[self.input.len()..],
            position: self.position + local_position,
            line,
            column,
            ..*self
        }
    }
}
