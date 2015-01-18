# unify.rs

Trait-based equality constraint for Rust

[![build status](https://api.travis-ci.org/darinmorrison/unify.rs.svg?branch=master)](https://travis-ci.org/darinmorrison/unify.rs)

## Synopsis

This crate provides a trait-based implementation of equality constraints. It is intended as a stopgap until proper equality predicates are implemented as described in the `where`-clause [RFC](https://github.com/nikomatsakis/rfcs/blob/where-clauses/active/0000-where.md).

Because it is a hack, there are some limitations with how it can be used. It is most effective as a helper for encoding additional trait invariants in order to rule out invalid implementations. Previously it was useful as a means to specify `Self` for static method invocation but this is no longer needed with [UFCS](https://github.com/rust-lang/rust/pull/21077). 

## Documentation

See the API documentation [here](http://darinmorrison.github.io/unify.rs/doc/unify/).

## Requirements

1.   [Rust](http://www.rust-lang.org/)
2.   [Cargo](http://crates.io/)

You can install both with the following:

```
$ curl -s https://static.rust-lang.org/rustup.sh | sudo sh
```

See [Installing Rust](http://doc.rust-lang.org/guide.html#installing-rust) for further details.

## Usage

```
$ cargo build       ## build library and binary
$ cargo test        ## run tests in ./tests
$ cargo bench       ## run benchmarks in ./benches
```

## Discussion

There is an IRC channel on [freenode](https://freenode.net) (chat.freenode.net) at [#epsilonz](http://webchat.freenode.net/?channels=%23epsilonz).
