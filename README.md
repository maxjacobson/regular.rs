# regular.rs

Playing around with using Rust procedural macros.
This adds a `Regular` trait that structs can derive automatically.
A `Regular` struct can save itself to a database (a text file).

## Usage

* [Install](https://rustup.rs/) rust 1.15 or newer.
* `cargo run`

## References

* <http://words.steveklabnik.com/an-overview-of-macros-in-rust> - context on how
  rust arrived where it is with macros
* <https://doc.rust-lang.org/book/procedural-macros.html> - good guide that I
  very much followed while tinkering here
