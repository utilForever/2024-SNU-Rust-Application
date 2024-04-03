# Exercise: Counter

In this exercise you will take a very simple data structure and make it generic.
It uses a
[`std::collections::HashMap`](https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html)
to keep track of which values have been seen and how many times each one has
appeared.

The initial version of `Counter` is hard coded to only work for `u32` values.
Make the struct and its methods generic over the type of value being tracked,
that way `Counter` can track any type of value.

If you finish early, try using the
[`entry`](https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html#method.entry)
method to halve the number of hash lookups required to implement the `count`
method.
