// pub mod create_tokenizers;

// use tokenizers::Tokenizer;
// use std::fs::File;
// use std::io::{Write};

// pub fn tokenize(s: &str){
//     // Load the tokenizer from file
//     let tokenizer = Tokenizer::from_file("tokenizer.json").expect("Failed to load tokenizer");

//     let sentence = s;

//     // Tokenize the sentence
//     let encoding = tokenizer.encode(sentence, true).expect("Failed to encode sentence");

//     // Convert token IDs to a Rust vector
//     let token_ids: Vec<u32> = encoding.get_ids().to_vec();

//     // Print tokenized output
//     println!("Tokenized IDs: {:?}", token_ids);

//     // Save to a file
//     let mut file = File::create("tokenized_vectors.txt").expect("Failed to create file");
//     writeln!(file, "{:?}", token_ids).expect("Failed to write to file");

//     println!("Embedded vectors saved to tokenized_vectors.txt");
// }

use tokenizers::Tokenizer;
use std::fs::File;
use std::io::{Write, Read};
use bincode; // Binary encoding crate
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct TokenizedData {
    pub token_ids: Vec<u32>,
}

pub fn tokenize(s: &str, filename: &str) {
    // Load the tokenizer from file
    let tokenizer = Tokenizer::from_file("tokenizer.json").expect("Failed to load tokenizer");

    // Tokenize the sentence
    let encoding = tokenizer.encode(s, true).expect("Failed to encode sentence");

    // Convert token IDs to a Rust vector
    let token_ids: Vec<u32> = encoding.get_ids().to_vec();

    // Print tokenized output
    println!("Tokenized IDs: {:?}", token_ids);

    // Store tokenized vector as binary data
    let data = TokenizedData { token_ids };

    let encoded: Vec<u8> = bincode::serialize(&data).expect("Failed to serialize tokenized data");
    let mut file = File::create(filename).expect("Failed to create file");
    file.write_all(&encoded).expect("Failed to write binary data");

    println!("Tokenized data saved to {}", filename);
}
