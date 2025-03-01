use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Write, Read};
use bincode; // For binary storage
use ort::{Environment, Session, Tensor};
use ndarray::Array1;

#[derive(Serialize, Deserialize, Clone)]
pub struct EmbeddedChunk {
    pub vector: Vec<f32>,
    pub text: String,
}

/// Loads the ONNX model for embedding
fn load_model(model_path: &str) -> Session {
    let environment = Environment::default().unwrap();
    Session::new(&environment, model_path).expect("Failed to load ONNX model")
}

/// Generates sentence embeddings using ONNX model
pub fn embed_chunks(chunks: Vec<String>, model_path: &str) -> Vec<EmbeddedChunk> {
    let session = load_model(model_path);
    let mut embeddings = Vec::new();

    for chunk in chunks {
        // Convert text into tokenized input (use tokenizer JSON if required)
        let input_tensor = Tensor::from_array(&[1, chunk.len()], chunk.as_bytes());
        let output = session.run(vec![input_tensor]).expect("Model inference failed");

        let vector = output[0].as_slice::<f32>().unwrap().to_vec();

        embeddings.push(EmbeddedChunk {
            vector,
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
