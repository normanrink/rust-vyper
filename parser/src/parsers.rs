use crate::errors::{PResult, ParseError};
use crate::traits::Input;

/// Parse a specific item `expected`.  Fail for EOF.  Error for character mismatch.
pub fn item<I>(expected: I::Item) -> impl Fn(I) -> PResult<I::Item, I>
where
    I: Input,
{
    move |input: I| {
        let mut iter = input.iter_indices();

        match iter.next() {
            None => ParseError::eof(input),
            Some((_, actual)) => {
                if actual != expected {
                    ParseError::error(input, format!("expected {:?}", expected))
                } else {
                    if let Some((i, _)) = iter.next() {
                        Ok((actual, input.take_last(i)))
                    } else {
                        Ok((actual, input.take_last(input.input_len())))
                    }
                }
            }
        }
    }
}

/// Parse `n` items.  Fail for EOF.  Error for too few items.
pub fn take<I>(n: usize) -> impl Fn(I) -> PResult<I, I>
where
    I: Input,
{
    move |input: I| match input.slice_index(n) {
        None => {
            if input.input_len() == 0 {
                ParseError::eof(input)
            } else {
                ParseError::error(input, format!("expected at least {} chars", n))
            }
        }
        Some(i) => Ok(input.take_split(i)),
    }
}

/// Parse leading items matched by `predicate`.  Fail for EOF.  Error for no matching items.
pub fn take_while<P, I>(predicate: P, fail_msg: &'static str) -> impl Fn(I) -> PResult<I, I>
where
    P: Fn(I::Item) -> bool,
    I: Input,
{
    move |input: I| {
        match input.position(|c| !predicate(c)) {
            None => {
                let len = input.input_len();
                if len == 0 {
                    // No failing index found because EOF
                    ParseError::eof(input)
                } else {
                    // No failing index found because all of input matched
                    Ok((input.take_last(len), input))
                }
            }
            // Input was non-empty and no matching index found
            Some(0) => ParseError::error(input, fail_msg.to_string()),
            // Input was non-empty and no matching index found
            Some(i) => Ok(input.take_split(i)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char() {
        // Success
        assert_eq!(item('c')("c"), Ok(('c', "")));

        // Char not found
        assert_eq!(
            item('c')("d"),
            ParseError::error("d", "expected 'c'".to_string()),
        );

        // EOF
        assert_eq!(item('c')(""), ParseError::eof(""),);
    }

    #[test]
    fn test_take() {
        // Success
        assert_eq!(take(4)("asdf"), Ok(("asdf", "")));
        assert_eq!(take(4)("asdfzxcv"), Ok(("asdf", "zxcv")));

        // Not enough input
        assert_eq!(
            take(4)("c"),
            ParseError::error("c", "expected at least 4 chars".to_string()),
        );

        // EOF
        assert_eq!(take(4)(""), ParseError::eof(""));
    }

    #[test]
    fn test_take_while() {
        // Success
        let take_while_alphabetic = take_while(
            |c: char| c.is_ascii_alphabetic(),
            "expected alphabetic chars",
        );

        assert_eq!(
            take_while_alphabetic("asdfASDF1234"),
            Ok(("asdfASDF", "1234"))
        );

        // Expected chars
        assert_eq!(
            take_while_alphabetic("1234"),
            ParseError::error("1234", "expected alphabetic chars".to_string()),
        );

        // EOF
        assert_eq!(take_while_alphabetic(""), ParseError::eof(""));
    }

    #[test]
    fn scratch() {
        let s = &"asdf".to_string();
        //println!("{:?}", s.());
    }
}
