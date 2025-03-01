pub mod create_tokenizer;
// pub mod embed;

// use tokenizers::Tokenizer;
// use std::fs::File;
// use std::io::{self, Write};


mod chunk;
mod embed;
mod hnsw;

use std::io;
use chunk::{read_text_from_file, chunk_text};
use embed::{embed_chunks, EmbeddedChunk};
use hnsw::{build_hnsw_index, search_hnsw};
use std::path::Path;

fn main() {
    // // Example input sentence
    // let sentence = "Hello, how are you?";
    // let filename = "data.vecbin";

    // embed::tokenize(&sentence, &filename);
    // embed::load_tokenized_data(&filename);

    // // Decode the tokenized vector back to text
    // let decoded_text = tokenizer.decode(&token_ids, true).expect("Failed to decode tokens");

    // // Print decoded output
    // println!("Decoded text: {}", decoded_text);

    let filename = "story.txt";
    let tokenizer_path = "tokenizer.json";
    let chunk_size = 50;
    let overlap = 10;

    if Path::new(tokenizer_path).exists(){
        // let tokenizer = Tokenizer::from_file("tokenizer.json").expect("Failed to load tokenizer");
        println!("tokenizer already exists, loading from file....")
    }
    else{
        create_tokenizer::create().expect("create_tokenizers failed");
        println!("Tokenizer created successfully!");
        // Load the tokenizer from file
        // let tokenizer = Tokenizer::from_file("tokenizer.json").expect("Failed to load tokenizer");
    }

    // Step 1: Read text and chunk it
    let text = read_text_from_file(filename);
    let chunks = chunk_text(&text, chunk_size, overlap);

    // Step 2: Embed chunks
    let embeddings = embed_chunks(chunks, tokenizer_path);

    // Step 3: Store in HNSW
    let hnsw_index = build_hnsw_index(embeddings.clone());

    // Step 4: Accept User Query
    println!("Enter your query:");
    let mut query = String::new();
    io::stdin().read_line(&mut query).expect("Failed to read input");

    // Step 5: Embed Query
    let query_embedding = embed_chunks(vec![query.clone()], tokenizer_path);

    // Step 6: Search for the best match
    let best_response = search_hnsw(&hnsw_index, query_embedding[0].vector.clone(), &embeddings);

    println!("Best response: {}", best_response);

}



// fn main() {
//     println!("{}", Path::new("tokenizer.json").exists());
// }