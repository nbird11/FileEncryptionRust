# Lazy File Encryption in Rust!

## Overview

With many of my language projects, I am trying to improve my learning skills and
adaptability by coding in programming languages that are new to me.

With this project in particular, I wanted to try out Rust! This is my first Rust
project and I may have bitten off a bit more than I could chew. What can I say; I'm ambitious!

Using this terminal application you will be able to specify a path to a file and encrypt or
decrypt it (very unsecurely and lazily) using byte shifting.

Here is a video demonstrating its functionality:

[Lazy File Encryption in Rust - Demo](https://youtu.be/qgMK2szVrjY)

## Development Environment

As with virtually all Rust projects out there, this program was made using the `cargo`
development environment.

Of course the programming language I used was Rust; the only reason there's a python file in here
was to test the byte-shifting algorithm. I used some basic packages in Rust such as `fs` for file
system management, `io` for reading user input from the console, and `io::Write` for writing to a
file.

## Useful Websites

There was a lot of referencing documentation throughout the process of writing this program. Here
were a few good sites I found useful:

- [Rust Official Documentation](https://doc.rust-lang.org/std/index.html)
- [TUtorialsPoint - Rust](https://www.tutorialspoint.com/rust/index.htm)

## Future Work

### Features

- Option to create new file or overwrite passed file
- Option to specify by how many bytes to shift
- Refactor file reading + en/decrypting in separate function
- Turn into command-line application using `env::args().collect::<Vec<string>>();`
