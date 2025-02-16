Divide slices into portions of same size
========================================

Divide_slice provides two additional methods to the primitive type [slice](https://doc.rust-lang.org/std/primitive.slice.html):
* `divide`: divide a slice into `n` non-overlapping portions, returning an iterator.
* `divide_mut`: divide a slice into `n` mutable non-overlapping portions, returning an iterator.

# Difference with `slice::chunks`
The standard library provides the methods [`chunks`](https://doc.rust-lang.org/std/primitive.slice.html#method.chunks) and [`chunks_mut`](https://doc.rust-lang.org/std/primitive.slice.html#method.chunks_mut), which return a (mutable) iterator over a given number of elements of a slice at the same time.

The difference between `chunks` and `divide` is that you determine the size of the chunks with `chunks`, and the number of subslices you want with `divide`:
```rust
let slice = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let mut iter = slice.chunks(3);
assert_eq!(iter.next(), Some(&[1, 2, 3][..]));
assert_eq!(iter.next(), Some(&[4, 5, 6][..]));
assert_eq!(iter.next(), Some(&[7, 8, 9][..]));
assert_eq!(iter.next(), Some(&[10][..]));
assert_eq!(iter.next(), None);

let mut iter = slice.divide(4);
assert_eq!(iter.next(), Some(&[1, 2, 3][..]));
assert_eq!(iter.next(), Some(&[4, 5, 6][..]));
assert_eq!(iter.next(), Some(&[7, 8][..]));
assert_eq!(iter.next(), Some(&[9, 10][..]));
assert_eq!(iter.next(), None);
```

`Divide` is more appropriate when you want to split the work equally among different threads.