use std::io::{self};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Need to input a valid string");
    
    let result = first_word(&input);
    println!("The first word is: {}", result);
}

fn first_word(s: &String) -> &str {
    let mut words = s.split(' ');
    match words.next() {
        Some(word) => word,
        None => &s,
    }
}