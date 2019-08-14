use std::fmt::Debug;
use std::str::{CharIndices, Chars};

pub trait Input: Sized {
    type Item: PartialEq + Debug;
    type Iter: Iterator<Item = (usize, Self::Item)>;
    type IterElem: Iterator<Item = Self::Item>;

    /// Return an iterator over the elements in an object including their unstructured indices.
    fn iter_indices(&self) -> Self::Iter;

    /// Return an iterator over the elements in an object.
    fn iter_elements(&self) -> Self::IterElem;

    /// Find the unstructured index of the first element matched by `predicate`.
    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool;

    /// Get the unstructured index of the `n`th element in an object.
    fn slice_index(&self, n: usize) -> Option<usize>;

    /// Return the initial unstructured items in an object up to but not including index 'i'.
    /// Panics if i > length.
    fn take_first(&self, i: usize) -> Self;

    /// Return the remaining unstructured items in an object beginning at index 'i'.  Panics if i >
    /// length.
    fn take_last(&self, i: usize) -> Self;

    /// Split an object at unstructured index `i` into a result-like tuple (remaining items first,
    /// output last).  Panics if i > length.
    fn take_split(&self, i: usize) -> (Self, Self) {
        (self.take_last(i), self.take_first(i))
    }

    /// Return the unstructured length of an object.
    fn input_len(&self) -> usize;
}

impl<'a> Input for &'a str {
    type Item = char;
    type Iter = CharIndices<'a>;
    type IterElem = Chars<'a>;

    #[inline]
    fn iter_indices(&self) -> Self::Iter {
        self.char_indices()
    }

    #[inline]
    fn iter_elements(&self) -> Self::IterElem {
        self.chars()
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        for (i, c) in self.char_indices() {
            if predicate(c) {
                return Some(i);
            }
        }
        None
    }

    #[inline]
    fn slice_index(&self, n: usize) -> Option<usize> {
        let mut count = 0;
        for (i, _) in self.char_indices() {
            if count == n {
                return Some(i);
            }
            count += 1;
        }
        if count == n {
            return Some(self.len());
        }
        None
    }

    #[inline]
    fn take_first(&self, i: usize) -> Self {
        &self[..i]
    }

    #[inline]
    fn take_last(&self, i: usize) -> Self {
        &self[i..]
    }

    #[inline]
    fn input_len(&self) -> usize {
        self.len()
    }
}
