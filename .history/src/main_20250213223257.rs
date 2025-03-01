pub mod create_tokenizers;
pub mod embed;

use tokenizers::Tokenizer;
use std::fs::File;
use std::io::{self, Write};

fn main() {
    create_tokenizers::create().expect("create_tokenizers failed");
    println!("Tokenizer created successfully!");
    // Load the tokenizer from file
    let tokenizer = Tokenizer::from_file("tokenizer.json").expect("Failed to load tokenizer");

    // Example input sentence
    let sentence = "Hello, how are you?";

    embed::tokenize(&sentence).expect("embed failed");

    // // Decode the tokenized vector back to text
    // let decoded_text = tokenizer.decode(&token_ids, true).expect("Failed to decode tokens");

    // // Print decoded output
    // println!("Decoded text: {}", decoded_text);
}