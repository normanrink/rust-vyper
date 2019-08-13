#[inline]
pub fn char_offset(s: &str, i: usize) -> Option<usize> {
    let mut j = 0;
    for (index, _) in s.char_indices() {
        if j == i {
            return Some(index);
        }
        j += 1;
    }
    if j == i {
        return Some(s.len());
    }
    None
}

#[inline]
pub fn len_chars(s: &str) -> usize {
    let mut i = 0;
    for _ in s.chars() {
        i += 1;
    }
    i
}

#[inline]
pub fn take_chars(s: &str, i: usize) -> Option<&str> {
    char_offset(s, i).map(|idx| &s[..idx])
}

#[inline]
pub fn split_bytes(s: &str, i: usize) -> (&str, &str) {
    (&s[i..], &s[..i])
}

#[inline]
pub fn split_chars(s: &str, i: usize) -> Option<(&str, &str)> {
    char_offset(s, i).map(|idx| (&s[idx..], &s[..idx]))
}
