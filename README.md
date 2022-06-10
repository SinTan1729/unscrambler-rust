# Unscrambler written in Rust

I'm learning Rust, so this is just a rewrite of a simple old project into Rust.

[Link to old C++ project.](https://github.com/SinTan1729/Unscrambler)

## Usage

Simply download the `unscrambler` binary from the latest release and run it. The interface is self-explanatory.

### Note

The main `src/wordlist` was pulled from [words_alpha.txt by dwyl](https://github.com/dwyl/english-words/).

In order to use a different `wordlist.txt`, place the file inside `src/` and delete the `*.xz` files there. Then run `cargo build` or `cargo build --release`.
