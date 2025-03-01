use crate::embed::EmbeddedChunk;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use ndarray::Array1;

/// Structure for HNSW Index
pub struct HNSW {
    pub vectors: Vec<EmbeddedChunk>,
    pub graph: HashMap<usize, Vec<usize>>,
}

impl HNSW {
    /// Creates a new empty HNSW index
    pub fn new() -> Self {
        Self {
            vectors: Vec::new(),
            graph: HashMap::new(),
        }
    }

    /// Adds an embedded vector to the HNSW graph
    pub fn add(&mut self, embedding: EmbeddedChunk) {
        let index = self.vectors.len();
        self.vectors.push(embedding);

        // Randomly connect to 3 other nodes (simulating HNSW layers)
        let mut rng = thread_rng();
        let neighbors: Vec<usize> = (0..self.vectors.len())
            .filter(|&i| i != index)
            .collect::<Vec<_>>()
            .choose_multiple(&mut rng, 3)
            .cloned()
            .collect();

        self.graph.insert(index, neighbors);
    }

    /// Computes cosine similarity between two vectors
    fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            return 0.0;
        }
        dot_product / (norm_a * norm_b)
    }

    /// Finds the nearest neighbor using HNSW search
    pub fn search(&self, query_vector: &[f32]) -> Option<String> {
        if self.vectors.is_empty() {
            return None;
        }

        let mut best_index = 0;
        let mut best_score = -1.0;

        // Start with a random node in the graph
        let mut rng = thread_rng();
        let mut current_node = **self.graph.keys().collect::<Vec<&usize>>().choose(&mut rng).unwrap();

        // ✅ Fix: Define an empty vector separately
        let empty_vec = Vec::new();

        // Perform greedy search
        for _ in 0..10 {
            let neighbors = self.graph.get(&current_node).unwrap_or(&empty_vec);
            let mut found_better = false;

            for &neighbor in neighbors {
                let similarity = Self::cosine_similarity(query_vector, &self.vectors[neighbor].vector);
                if similarity > best_score {
                    best_score = similarity;
                    best_index = neighbor;
                    current_node = neighbor;
                    found_better = true;
                }
            }

            if !found_better {
                break;
            }
        }

        Some(self.vectors[best_index].text.clone())
    }
}
