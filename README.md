An experimental library that provides some common statistical functions with
some support for computing them efficiently on *streams* of data. The intent
is to permit parallel computation of statistics on large data sets.

[![Build status](https://api.travis-ci.org/BurntSushi/rust-stats.png)](https://travis-ci.org/BurntSushi/rust-stats)
[![](http://meritbadge.herokuapp.com/streaming-stats)](https://crates.io/crates/streaming-stats)

Dual-licensed under MIT or the [UNLICENSE](http://unlicense.org).


### Documentation

Some documentation exists here:
[http://burntsushi.net/rustdoc/stats/](http://burntsushi.net/rustdoc/stats/).


### Installation

This crate works with Cargo. Assuming you have Rust and
[Cargo](http://crates.io/) installed, simply check out the source and run
tests:

```bash
git checkout git://github.com/BurntSushi/rust-stats
cd rust-stats
cargo test
```

You can also add `rust-stats` as a dependency to your project's `Cargo.toml`:

```toml
[dependencies.rust-stats]
git = "git://github.com/BurntSushi/rust-stats"
```

