use tokenizers::Tokenizer;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Write, Read};
use bincode; // For binary storage

#[derive(Serialize, Deserialize, Clone)]
pub struct EmbeddedChunk {
    pub vector: Vec<f32>,
    pub text: String,
}

/// Loads tokenizer and embeds text chunks into vectors
pub fn embed_chunks(chunks: Vec<String>, tokenizer_path: &str) -> Vec<EmbeddedChunk> {
    let tokenizer = Tokenizer::from_file(tokenizer_path).expect("Failed to load tokenizer");
    let mut embeddings = Vec::new();

    for chunk in chunks {
        let encoding = tokenizer.encode(&chunk, true).expect("Failed to encode");
        let token_ids: Vec<f32> = encoding.get_ids().iter().map(|&id| id as f32).collect();
        
        embeddings.push(EmbeddedChunk {
            vector: token_ids,
            text: chunk.clone(),
        });
    }

    embeddings
}

/// Saves embedded vectors to a binary file
pub fn save_embeddings(filename: &str, embeddings: &Vec<EmbeddedChunk>) {
    let encoded: Vec<u8> = bincode::serialize(embeddings).expect("Failed to serialize embeddings");
    let mut file = File::create(filename).expect("Failed to create file");
    file.write_all(&encoded).expect("Failed to write embeddings to disk");

    println!("Embeddings saved to {}", filename);
}

/// Loads embedded vectors from a binary file
pub fn load_embeddings(filename: &str) -> Vec<EmbeddedChunk> {
    let mut file = File::open(filename).expect("Failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");
    
    bincode::deserialize(&buffer).expect("Failed to deserialize embeddings")
}