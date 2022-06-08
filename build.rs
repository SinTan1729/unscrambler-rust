use std::{fs, io::Read};
use xz2::read::XzEncoder;

fn main() {
    // check if the compressed dictionary files exist, run if missing
    // so, in order to rebuild the compressed files, just delete them
    if !fs::metadata("src/dict/wordlist.txt.xz").is_ok() {
        compress_wordlist();
    }
    if !fs::metadata("src/dict/wordlist_sorted.txt.xz").is_ok() {
        compress_sorted_wordlist();
    }
}

fn compress_wordlist() {
    // specify location for dictionary files and read wordlist.txt
    let dict_dir = "src/dict/";
    let wordlist = fs::read_to_string([dict_dir, "wordlist.txt"].join(""))
        .expect("The file wordlist.txt is missing!");
    // compress wordlist.txt using xz compression and save it
    let wordlist_bytes = wordlist.as_bytes();
    let mut compressor = XzEncoder::new(wordlist_bytes, 9);
    let mut compressed_wordlist = Vec::new();
    compressor.read_to_end(&mut compressed_wordlist).unwrap();
    fs::write([dict_dir, "wordlist.txt.xz"].join(""), compressed_wordlist).unwrap();
}

fn compress_sorted_wordlist() {
    // specify location for dictionary files
    let dict_dir = "src/dict/";

    // create wordlist_sorted from wordlist.txt
    let wordlist = fs::read_to_string([dict_dir, "wordlist.txt"].join(""))
        .expect("The file wordlist.txt is missing!");
    let mut wordlist_sorted = String::new();
    for word in wordlist.split_terminator("\n") {
        wordlist_sorted = [wordlist_sorted, sorted(word), "\n".to_string()].join("");
    }

    //compress wordlist_sorted using xz compression and save it
    let wordlist_sorted_bytes = wordlist_sorted.as_bytes();
    let mut compressor_sorted = XzEncoder::new(wordlist_sorted_bytes, 9);
    let mut compressed_wordlist_sorted = Vec::new();
    compressor_sorted
        .read_to_end(&mut compressed_wordlist_sorted)
        .unwrap();
    fs::write(
        [dict_dir, "wordlist_sorted.txt.xz"].join(""),
        compressed_wordlist_sorted,
    )
    .unwrap();
}

// function for sorting
fn sorted(word: &str) -> String {
    let mut word_chars: Vec<char> = word.chars().collect();
    word_chars.sort_by(|a, b| a.cmp(b));
    String::from_iter(word_chars)
}
