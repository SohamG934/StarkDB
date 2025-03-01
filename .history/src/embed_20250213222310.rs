use crate::create_tokenizers;

use tokenizers::Tokenizer;
use std::fs::File;
use std::io::{self, Write};

pub fn tokenize(s: &str){
    // Load the tokenizer from file
    let tokenizer = Tokenizer::from_file("tokenizer.json").expect("Failed to load tokenizer");

    let sentence = s;

    // Tokenize the sentence
    let encoding = tokenizer.encode(sentence, true).expect("Failed to encode sentence");

    // Convert token IDs to a Rust vector
    let token_ids: Vec<u32> = encoding.get_ids().to_vec();

    // Print tokenized output
    println!("Tokenized IDs: {:?}", token_ids);

    // Save to a file
    let mut file = File::create("tokenized_vectors.txt").expect("Failed to create file");
    writeln!(file, "{:?}", token_ids).expect("Failed to write to file");

    println!("Embedded vectors saved to tokenized_vectors.txt");
}