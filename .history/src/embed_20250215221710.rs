use tokenizers::Tokenizer;
use ndarray::Array1;
use serde::{Serialize, Deserialize};

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
        let encoding = tokenizer.encode(chunk.as_str(), true).expect("Failed to encode");
        let token_ids: Vec<f32> = encoding.get_ids().to_vec().iter().map(|&id| id as f32).collect();
        
        embeddings.push(EmbeddedChunk {
            vector: token_ids,
            text: chunk.clone(),
        });
    }

    embeddings
}
