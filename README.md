Divide slices into portions of same size
========================================

Divide_slice provides two additional methods to the primitive type [slice](https://doc.rust-lang.org/std/primitive.slice.html):
* `divide`: divide a slice into `n` non-overlapping portions, returning an iterator.
* `divide_mut`: divide a slice into `n` mutable non-overlapping portions, returning an iterator.