//! Divide_slice provides two additional methods to the primitive type [slice]:
//!  * [`divide`]: divide a slice into `n` non-overlapping portions, returning an iterator.
//!  * [`divide_mut`]: divide a slice into `n` mutable non-overlapping portions, returning an iterator.
//! 
//! [slice]: slice
//! [`divide`]: Divide::divide
//! [`divide_mut`]: Divide::divide_mut

#![feature(raw_slice_split)]

use std::marker::PhantomData;

pub trait Divide<T> {
    fn divide(&self, n: usize) -> Portion<'_, T>;
    fn divide_mut(&mut self, n: usize) -> PortionMut<'_, T>;
}

impl<T> Divide<T> for [T] {
    /// Divides a slice into `n` non-overlapping portions, returning an iterator.
    ///
    /// The portions are computed by distributing elements as evenly as possible.
    /// If the length of the slice is not evenly divisible by `n`, the first portions may
    /// have one more element than the others. If the length of the slice is smaller than
    /// `n`, the last portions will be empty.
    ///
    /// # Panics
    ///
    /// Panics if `n` is 0.
    ///
    /// # Example
    ///
    /// ```
    /// use divide_slice::Divide;
    ///
    /// let slice = [1, 2, 3, 4, 5, 6];
    /// let mut iter = slice.divide(3);
    /// assert_eq!(iter.next(), Some(&[1, 2][..]));
    /// assert_eq!(iter.next(), Some(&[3, 4][..]));
    /// assert_eq!(iter.next(), Some(&[5, 6][..]));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn divide(&self, n: usize) -> Portion<'_, T> {
        assert!(n != 0, "cannot divide into zero portions");
        Portion::new(self, n)
    }

    /// Divides a slice into `n` mutable non-overlapping portions, returning an iterator.
    ///
    /// The portions are computed by distributing elements as evenly as possible.
    /// If the length of the slice is not evenly divisible by `n`, the first portions may
    /// have one more element than the others. If the length of the slice is smaller than
    /// `n`, the last portions will be empty.
    ///
    /// # Panics
    ///
    /// Panics if `n` is 0.
    ///
    /// # Example
    ///
    /// ```
    /// use divide_slice::Divide;
    ///
    /// let mut slice = [1, 2, 3, 4, 5, 6];
    /// slice.divide_mut(3).for_each(|e| e[0] += 1);
    /// let mut iter = slice.divide_mut(3);
    /// assert_eq!(iter.next(), Some(&mut [2, 2][..]));
    /// assert_eq!(iter.next(), Some(&mut [4, 4][..]));
    /// assert_eq!(iter.next(), Some(&mut [6, 6][..]));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn divide_mut(&mut self, n: usize) -> PortionMut<'_, T> {
        assert!(n != 0, "cannot divide into zero portions");
        PortionMut::new(self, n)
    }
}

/// An iterator over a slice in `n` non-overlapping portions, starting at the beginning of the slice.
///
/// The portions are slices and do not overlap. If the length of the slice is not evenly divided
/// by `n`, the first portions may have one more element than the others.
///
/// This struct is created by the [`divide`] method on [slices].
///
/// # Example
///
/// ```
/// use divide_slice::Divide;
///
/// let slice = ['a', 'b', 'c', 'd', 'e'];
/// let mut iter = slice.divide(3);
/// assert_eq!(iter.next(), Some(&['a', 'b'][..]));
/// assert_eq!(iter.next(), Some(&['c', 'd'][..]));
/// assert_eq!(iter.next(), Some(&['e'][..]));
/// assert_eq!(iter.next(), None);
/// ```
///
/// [`divide`]: Divide::divide
/// [slices]: slice
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Portion<'a, T: 'a> {
    v: &'a [T],
    n: usize,
}

impl<'a, T: 'a> Portion<'a, T> {
    #[inline]
    pub fn new(slice: &'a [T], n: usize) -> Self {
        Self { v: slice, n }
    }
}

impl<'a, T> Iterator for Portion<'a, T> {
    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<&'a [T]> {
        if self.n == 0 {
            None
        } else {
            let portionsz = self.v.len().div_ceil(self.n);
            self.n -= 1;
            let (fst, snd) = self.v.split_at(portionsz);
            self.v = snd;
            Some(fst)
        }
    }
}

/// An iterator over a slice in `n` mutable non-overlapping portions, starting at the beginning of the slice.
///
/// The portions are mutable slices and do not overlap. If the length of the slice is not evenly divided
/// by `n`, the first portions may have one more element than the others.
///
/// This struct is created by the [`divide_mut`] method on [slices].
///
/// # Example
///
/// ```
/// use divide_slice::Divide;
///
/// let mut slice = ['a', 'b', 'c', 'd', 'e'];
/// let mut iter = slice.divide_mut(3);
/// assert_eq!(iter.next(), Some(&mut ['a', 'b'][..]));
/// assert_eq!(iter.next(), Some(&mut ['c', 'd'][..]));
/// assert_eq!(iter.next(), Some(&mut ['e'][..]));
/// assert_eq!(iter.next(), None);
/// ```
///
/// [`divide_mut`]: Divide::divide_mut
/// [slices]: slice
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct PortionMut<'a, T: 'a> {
    v: *mut [T],
    n: usize,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T: 'a> PortionMut<'a, T> {
    #[inline]
    pub fn new(slice: &'a mut [T], n: usize) -> Self {
        Self {
            v: slice,
            n,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for PortionMut<'a, T> {
    type Item = &'a mut [T];

    #[inline]
    fn next(&mut self) -> Option<&'a mut [T]> {
        if self.n == 0 {
            None
        } else {
            let portionsz = self.v.len().div_ceil(self.n);
            self.n -= 1;
            let (fst, snd) = unsafe { self.v.split_at_mut(portionsz) };
            self.v = snd;
            Some(unsafe { &mut *fst })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divide_no_remainder() {
        let slice = ['a', 'b', 'c', 'd', 'e', 'f'];
        let mut iter = slice.divide(3);
        assert_eq!(iter.next(), Some(&['a', 'b'][..]));
        assert_eq!(iter.next(), Some(&['c', 'd'][..]));
        assert_eq!(iter.next(), Some(&['e', 'f'][..]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn divide_with_reminder() {
        let slice = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'];
        let mut iter = slice.divide(4);
        assert_eq!(iter.next(), Some(&['a', 'b', 'c'][..]));
        assert_eq!(iter.next(), Some(&['d', 'e'][..]));
        assert_eq!(iter.next(), Some(&['f', 'g'][..]));
        assert_eq!(iter.next(), Some(&['h', 'i'][..]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn divide_smaller_size() {
        let slice = ['a', 'b'];
        let mut iter = slice.divide(3);
        assert_eq!(iter.next(), Some(&['a'][..]));
        assert_eq!(iter.next(), Some(&['b'][..]));
        assert_eq!(iter.next(), Some(&[][..]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn divide_empty_slice() {
        let slice: [char; 0] = [];
        let mut iter = slice.divide(3);
        assert_eq!(iter.next(), Some(&[][..]));
        assert_eq!(iter.next(), Some(&[][..]));
        assert_eq!(iter.next(), Some(&[][..]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    #[should_panic]
    fn divide_by_zero() {
        let slice = ['a', 'b'];
        let _ = slice.divide(0);
    }

    #[test]
    fn divide_mut_no_remainder() {
        let mut slice = [1, 2, 3, 4, 5, 6];
        slice.divide_mut(3).for_each(|e| e[0] += 1);
        let mut iter = slice.divide_mut(3);
        assert_eq!(iter.next(), Some(&mut [2, 2][..]));
        assert_eq!(iter.next(), Some(&mut [4, 4][..]));
        assert_eq!(iter.next(), Some(&mut [6, 6][..]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn divide_mut_with_remainder() {
        let mut slice = [1, 2, 3, 4, 5];
        slice.divide_mut(3).for_each(|e| e[0] += 1);
        let mut iter = slice.divide_mut(3);
        assert_eq!(iter.next(), Some(&mut [2, 2][..]));
        assert_eq!(iter.next(), Some(&mut [4, 4][..]));
        assert_eq!(iter.next(), Some(&mut [6][..]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn divide_mut_smaller_size() {
        let mut slice = [1, 2];
        slice.divide_mut(3).for_each(|e| if e.len() > 0 { e[0] += 1 });
        let mut iter = slice.divide_mut(3);
        assert_eq!(iter.next(), Some(&mut [2][..]));
        assert_eq!(iter.next(), Some(&mut [3][..]));
        assert_eq!(iter.next(), Some(&mut [][..]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn divide_mut_empty() {
        let mut slice: [char; 0] = [];
        let mut iter = slice.divide_mut(3);
        assert_eq!(iter.next(), Some(&mut [][..]));
        assert_eq!(iter.next(), Some(&mut [][..]));
        assert_eq!(iter.next(), Some(&mut [][..]));
        assert_eq!(iter.next(), None);
    }
}
