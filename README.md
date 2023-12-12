# Snake

It's snake. Currently only supports command line play. Control your snake with the arrow keys. Eat the apples and don't crash.

## Installation

This Rust project is bundled (is that the word?) as a Cargo crate. You'll want to have Rust installed and `cargo run` or equivalent.
The docs <sup>[[1](https://doc.rust-lang.org/book/ch01-01-installation.html)]</sup> <sup>[[2](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)]</sup> might help.

## Changelog

### 12 Dec 2023

Version 0.3.0. Minor changes: movement of snake is now in O(1) instead of of O(n), where n is the length of the snake. 

Major changes:
Apples now spawn in randomly with time complexity O(m(s+a)) 
where m, s, and a is the area of the map, the length of the snake and the number of apples, respectively.
With some tomfoolery, I might be able to get it down to still O(m).

### 12 Dec 2023

Version 0.2.0. Queued movements are now much more sane. I had to figure out how to work Arcs and Mutexes to make them work.
You really do learn so much by working on projects, who would've known? There's plenty more I want to implement, but I'm already so happy with how
this is turning out and how much I'm learning. I want to add a menu screen, a scoreboard that persists over game sessions, colors, some way to distinguish
where the path of your snake is. 

Hmm. It would be a pretty neat gimmick to be able to play snake on your screen, like the gimmick for the game [windowkill](https://torcado.itch.io/windowkill).
Unfortunately, the two crates that I'm looking at for window control are pretty dated (three years since last updated) and don't have cross platform support. Hmmmm...

### 11 Dec 2023

Version 0.1.0. First commit, the entire thing is uh. Questionable. The map is stored as a massive String, queued movements are processed by 
reading and writing to a file because I couldn't figure out how to do it in the intuitive way :skull: It functions, though.