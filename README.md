# Snake

It's snake. Currently only supports command line play. Control your snake with the arrow keys. Eat the apples and don't crash.

## Installation

This Rust project is bundled (is that the word?) as a Cargo crate. You'll want to have Rust installed and `cargo run` or equivalent.
The docs <sup>[[1](https://doc.rust-lang.org/book/ch01-01-installation.html)]</sup> <sup>[[2](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)]</sup> might help.

## Changelog

### 11 Dec 2023

First commit, the entire thing is uh. Questionable. The map is stored as a massive String, queued movements are processed by 
reading and writing to a file because I couldn't figure out how to do it in the intuitive way :skull: It functions, though.