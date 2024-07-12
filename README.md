# An spike implementing binary VSA in Rust

Based on: Kanerva (2009), [Hyperdimensional Computing: An Introduction to Computing in Distributed Representation with High-Dimensional Random Vectors](https://link.springer.com/article/10.1007/s12559-009-9009-8), _Cognitive Computation_ **1**.


## To run

```
cargo run
```

There are only `assert`s in there, so don't expect to see any output!

## Clues

I've defined a "hyperspace" as a store of vectors.

To avoid having to print out huge vectors, the hyperspace also
associates a label you supply with each vector.

The hyperspace is abstract over the type of vectors, but I've used the `bitvec` crate for this.

The last piece in an `Algebra` trait, which implements addition and multiplication for the vector of choice. I've implemented this for `bitvec`.

