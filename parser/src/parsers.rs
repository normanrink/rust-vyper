use crate::internals::{
    IResult,
    State,
    ParseError,
};
use crate::utils::{
    len_chars,
};

/// Parse a specific char `expected`.  Exit for EOF.  Return for character mismatch.
pub fn char<'a>(expected: char) -> impl Fn(State<'a>) -> IResult<State<'a>, char> {
    move |state: State| {
        match state.input.chars().next() {
            None => ParseError::make_eof(state),
            Some(actual) => if actual != expected {
                ParseError::make_return(state, format!("expected {:?}", expected))
            } else {
                Ok((state.advance(1), expected))
            },
        }
    }
}

/// Parse `n` characters.  Exit for EOF.  Return for too few chars.
pub fn take<'a>(n: usize) -> impl Fn(State<'a>) -> IResult<State<'a>, &'a str> {
    move |state: State| {
        let len = len_chars(state.input);

        if len == 0 {
            ParseError::make_eof(state)
        } else if len < n {
            ParseError::make_return(state, format!("expected at least {} chars", n))
        } else {
            Ok((state.advance(n), &state.input[..n]))
        }
    }
}

/// Parse leading characters matched by `predicate`.  Exit for EOF.  Return for no matching chars.
pub fn take_while<'a, P>(predicate: P, fail_msg: &'static str)-> impl Fn(State<'a>) -> IResult<State<'a>, &'a str>
where P: Fn(char) -> bool
{
    move |state: State| {
        match state.input.find(|c| !predicate(c)) {
            None => {
                let len = state.input.len();

                if len == 0 {
                    // No failing index found because EOF
                    ParseError::make_eof(state)
                } else {
                    // No failing index found because all of input matched
                    let input = state.input;
                    Ok((state.advance(len), input))
                }
            },
            // Input was non-empty and no matching index found
            Some(0) => ParseError::make_return(state, fail_msg.to_string()),
            // Input was non-empty and no matching index found
            Some(i) => Ok((state.advance(i), &state.input[..i])),
        }
    }
}

/// Execute a parsing operation with `parser` and an initial state created from `source`.
pub fn do_parse<'a, P, O>(parser: P, source: &'a str) -> IResult<State<'a>, O>
where P: Fn(State<'a>) -> IResult<State<'a>, O>
{
    parser(State::from_str(source))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char() {
        // Success
        assert_eq!(
            do_parse(char('c'), "c"),
            Ok((State::from_str_n("c", 1), 'c')),
        );

        // Char not found
        assert_eq!(
            do_parse(char('c'), "d"),
            ParseError::make_return(State::from_str("d"), "expected 'c'".to_string()),
        );

        // EOF
        assert_eq!(
            do_parse(char('c'), ""),
            ParseError::make_exit(State::from_str(""), "EOF".to_string()),
        );
    }

    #[test]
    fn test_take() {
        // Success
        assert_eq!(
            do_parse(take(4), "asdf"),
            Ok((State::from_str_n("asdf", 4), "asdf"))
        );
        assert_eq!(
            do_parse(take(4), "asdfzxcv"),
            Ok((State::from_str_n("asdfzxcv", 4), "asdf"))
        );

        // Not enough input
        assert_eq!(
            do_parse(take(4), "c"),
            ParseError::make_return(State::from_str("c"), "expected at least 4 chars".to_string()),
        );

        // EOF
        assert_eq!(
            do_parse(take(4), ""),
            ParseError::make_eof(State::from_str("")),
        );
    }

    #[test]
    fn test_take_while() {
        // Success
        let take_while_alphabetic = take_while(
            |c| c.is_ascii_alphabetic(),
            "expected alphabetic chars",
        );
        assert_eq!(
            do_parse(take_while_alphabetic, "asdfASDF1234"),
            Ok((State::from_str_n("asdfASDF1234", 8), "asdfASDF"))
        );

        // Expected chars
        let take_while_alphabetic = take_while(
            |c| c.is_ascii_alphabetic(),
            "expected alphabetic chars",
        );
        assert_eq!(
            do_parse(take_while_alphabetic, "1234"),
            ParseError::make_return(State::from_str("1234"), "expected alphabetic chars".to_string()),
        );

        // EOF
        let take_while_alphabetic = take_while(
            |c| c.is_ascii_alphabetic(),
            "expected alphabetic chars",
        );
        assert_eq!(
            do_parse(take_while_alphabetic, ""),
            ParseError::make_eof(State::from_str("")),
        );
    }

    #[test]
    fn scratch() {
        let s = &"asdf".to_string();
        println!("{:?}", s.find(|c: char| !c.is_ascii_alphabetic()));
    }
}
