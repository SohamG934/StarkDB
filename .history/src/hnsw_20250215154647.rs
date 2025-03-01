use linfa_clustering::hnsw::{Hnsw, HnswParams};
use ndarray::Array1;
use crate::embed::EmbeddedChunk;

/// Stores embeddings in HNSW Index
pub fn build_hnsw_index(embeddings: Vec<EmbeddedChunk>) -> Hnsw<f32> {
    let dim = embeddings[0].vector.len(); // Vector dimension

    let mut hnsw = HnswParams::new(dim)
        .max_connections(16)
        .build();

    for (i, emb) in embeddings.iter().enumerate() {
        let vec = Array1::from(emb.vector.clone());
        hnsw.insert(vec, i);
    }

    hnsw
}

/// Finds the nearest document chunk for a given query
pub fn search_hnsw(hnsw: &Hnsw<f32>, query_vector: Vec<f32>, embeddings: &Vec<EmbeddedChunk>) -> String {
    let query = Array1::from(query_vector);
    let nearest = hnsw.nearest(&query, 1); // Retrieve 1 closest document

    if let Some((index, _)) = nearest.first() {
        embeddings[*index].text.clone()
    } else {
        "No relevant document found.".to_string()
    }
}
