use std::io::{self, prelude::*, Write};
use xz2::read::XzDecoder;

fn main() {
    // load the compressed dictionary files (embedded in compile-time)
    let wordlist_cmp: &[u8] = include_bytes!("dict/wordlist.txt.xz");
    let wordlist_sorted_cmp: &[u8] = include_bytes!("dict/wordlist_sorted.txt.xz");

    // decompress the dictionary files
    let mut decompressor = XzDecoder::new(wordlist_cmp);
    let mut decompressor_sorted = XzDecoder::new(wordlist_sorted_cmp);
    let mut wordlist = String::new();
    let mut wordlist_sorted = String::new();
    decompressor.read_to_string(&mut wordlist).unwrap();
    decompressor_sorted
        .read_to_string(&mut wordlist_sorted)
        .unwrap();

    // some formatting of the dictionary data
    let wordlist = &[" ", &wordlist.replace("\n", " ")[..]].join("")[..];
    let wordlist_sorted = &[" ", &wordlist_sorted.replace("\n", " ")[..]].join("")[..];

    // get into a loop so that we can run it multiple times
    loop {
        // get input
        let mut input = String::new();
        print!("Please enter the scrambled word: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string().to_lowercase();
        match input.find(" ") {
            None => (),
            _ => {
                println!("Please enter only one scrambled word.");
                continue;
            }
        }

        // process input and sort it
        let input = &input[..];
        let mut broken_input: Vec<char> = input.chars().collect();
        broken_input.sort_by(|a, b| a.cmp(b));
        let input = &[" ", &String::from_iter(broken_input)[..], " "].join("")[..];

        // find in dictionary
        let indices: Vec<usize> = wordlist_sorted
            .match_indices(input)
            .map(|(i, _)| i)
            .collect();

        // print output
        if indices.len() == 0 {
            println!("No matches found!");
        } else {
            println!("The matched words are:");
            for index in indices {
                println!(
                    "{}",
                    sentence_case(&wordlist[index + 1..index - 1 + input.len()])
                );
            }
        }

        // ask if we want to go again
        print!("Would you like to do it again? (y/N)");
        io::stdout().flush().unwrap();
        let mut response = String::new();
        io::stdin().read_line(&mut response).unwrap();
        if !(response.contains("Y") || response.contains("y")) {
            break;
        }
    }
}

// function for conversion into sentence case
fn sentence_case(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
