mod chunk;
mod embed;
mod hnsw;

use std::io;
use chunk::{read_text_from_file, chunk_text};
use embed::{embed_chunks, save_embeddings, load_embeddings};
use hnsw::HNSW;

fn main() {
    let filename = "textfile.txt";
    let model_path = "models/all-MiniLM-L6-v2.onnx"; // ✅ NEW: Use Transformer Model
    let embeddings_file = "embeddings.bin";
    let chunk_size = 50;
    let overlap = 10;

    // Step 1: Read text and chunk it
    let text = read_text_from_file(filename);
    let chunks = chunk_text(&text, chunk_size, overlap);

    // Step 2: Embed chunks using Transformer Model
    let embeddings = embed_chunks(chunks, model_path);

    // Step 3: Save embeddings to disk
    save_embeddings(embeddings_file, &embeddings);

    // Step 4: Load embeddings from disk
    let loaded_embeddings = load_embeddings(embeddings_file);

    // Step 5: Build HNSW index from loaded embeddings
    let hnsw = HNSW::from_embeddings(loaded_embeddings.clone());

    // Step 6: Accept User Query
    println!("Enter your query:");
    let mut query = String::new();
    io::stdin().read_line(&mut query).expect("Failed to read input");

    // Step 7: Embed Query using Transformer Model
    let query_embedding = embed_chunks(vec![query.clone()], model_path);

    // Step 8: Search for the best match
    if let Some(best_response) = hnsw.search(&query_embedding[0].vector) {
        println!("Best response: {}", best_response);
    } else {
        println!("No relevant document found.");
    }
}
