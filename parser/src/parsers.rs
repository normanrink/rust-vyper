use crate::internals::{
    IResult,
    State,
    ParseError,
};

fn take<'a>(n: usize) -> impl Fn(State<'a>) -> IResult<State<'a>, &'a str> {
    move |state: State| {
        let len = state.input.len();

        if len == 0 {
            ParseError::make_eof(state)
        } else if len < n {
            ParseError::make_return(state, format!("expected at least {} chars", n))
        } else {
            Ok((state.advance(n), &state.input[..n]))
        }
    }
}

fn char<'a>(expected: char) -> impl Fn(State<'a>) -> IResult<State<'a>, char> {
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

fn do_parse<'a, P, O>(parser: P, source: &'a str) -> IResult<State<'a>, O>
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
    fn scratch() {
        let s = &"asdf".to_string();
        if s.len() >= 4 {
            println!("{:?}", &s[..4]);
        } else {
        }
    }
}
