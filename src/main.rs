use std::io::{self, Write};

fn main() {
    // read the dictionary files
    let wordlist = include_str!("data/wordlist.txt");
    let wordlist_sorted = include_str!("data/wordlist_sorted.txt");

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
                    sentence_case(&wordlist[index + 1..index + input.len() - 1])
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
        Some(f) => f.to_uppercase().collect::<String>() + &c.as_str()[..].to_lowercase(),
    }
}
