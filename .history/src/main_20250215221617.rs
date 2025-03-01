pub mod create_tokenizer;
mod chunk;
mod embed;
mod hnsw;

use std::io;
use chunk::{read_text_from_file, chunk_text};
use embed::{embed_chunks, EmbeddedChunk};
use hnsw::HNSW;
use std::path::Path;

fn main() {
    let filename = "story.txt";
    let tokenizer_path = "tokenizer.json";
    let chunk_size = 50;
    let overlap = 10;

    if Path::new(tokenizer_path).exists(){
        println!("tokenizer already exists, loading from file....")
    }
    else{
        create_tokenizer::create().expect("create_tokenizers failed");
        println!("Tokenizer created successfully!");
    }

    // Step 1: Read text and chunk it
    let text = read_text_from_file(filename);
    let chunks = chunk_text(&text, chunk_size, overlap);

    // Step 2: Embed chunks
    let embeddings = embed_chunks(chunks, tokenizer_path);

    // Step 3: Build HNSW index
    let mut hnsw = HNSW::new();
    for emb in embeddings.clone() {
        hnsw.add(emb);
    }

    // Step 4: Accept User Query
    println!("Enter your query:");
    let mut query = String::new();
    io::stdin().read_line(&mut query).expect("Failed to read input");

    // Step 5: Embed Query
    let query_embedding = embed_chunks(vec![query.clone()], tokenizer_path);

    // Step 6: Search for the best match
    if let Some(best_response) = hnsw.search(&query_embedding[0].vector) {
        println!("Best response: {}", best_response);
    } else {
        println!("No relevant document found.");
    }

}