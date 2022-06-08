# Unscrambler written in Rust

I'm learning Rust, so this is just a rewrite of an simple old project in Rust.

[Link to old project.](https://github.com/SinTan1729/Unscrambler)

### Note

The main `src/wordlist` was pulled from [words_alpha.txt by dwyl](https://github.com/dwyl/english-words/) and processed using Rust. Processing code was really simple, so didn't put it up here. The processing included pre-sorting the each line in `src/wordlist` to create `src/wordlist_sorted` and then compressing both using `xz`.