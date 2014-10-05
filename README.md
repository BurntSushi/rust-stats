An experimental library that provides a `Crdt` trait with some common
implementations of statistical functions (mean, standard deviation, frequency, 
etc.).

[![Build status](https://api.travis-ci.org/BurntSushi/rust-crdt.png)](https://travis-ci.org/BurntSushi/rust-crdt)

Licensed under the [UNLICENSE](http://unlicense.org).


### Documentation

Some documentation exists here:
[http://burntsushi.net/rustdoc/crdt/](http://burntsushi.net/rustdoc/crdt/).


### Installation

This crate works with Cargo. Assuming you have Rust and
[Cargo](http://crates.io/) installed, simply check out the source and run 
tests:

```bash
git checkout git://github.com/BurntSushi/rust-crdt
cd rust-crdt
cargo test
```

You can also add `rust-crdt` as a dependency to your project's `Cargo.toml`:

```toml
[dependencies.rust-crdt]
git = "git://github.com/BurntSushi/rust-crdt"
```

