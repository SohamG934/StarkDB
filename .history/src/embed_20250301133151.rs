use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Write, Read};
use bincode;
use rust_bert::pipelines::sentence_embeddings::{SentenceEmbeddingsModel, SentenceEmbeddingsBuilder};
use tch::Device;
use rust_bert::pipelines::sentence_embeddings::SentenceEmbeddingsModelType;
// use tch::{Device, Tensor};

#[derive(Serialize, Deserialize, Clone)]
pub struct EmbeddedChunk {
    pub vector: Vec<f32>,
    pub text: String,
}

// fn normalize_embedding(embedding: Vec<f32>) -> Vec<f32> {
//     let tensor = Tensor::of_slice(&embedding);
//     let norm = tensor.norm();
//     tensor.div(norm).to_vec1().unwrap()
// }


/// Loads transformer model in Rust
fn load_model() -> SentenceEmbeddingsModel {
    SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2)
        .with_device(Device::cuda_if_available())
        .create_model()
        .expect("Failed to load model")
}

/// Generates sentence embeddings using Rust-BERT
pub fn embed_chunks(chunks: Vec<String>) -> Vec<EmbeddedChunk> {
    let model = load_model();
    let embeddings = model.encode(&chunks).expect("Failed to encode text");

    // normalize_embedding(embeddings[0].clone()); // Apply normalization

    chunks
        .into_iter()
        .zip(embeddings.into_iter())
        .map(|(text, vector)| EmbeddedChunk { vector, text })
        .collect()
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