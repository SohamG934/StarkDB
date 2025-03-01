use std::fs::File;
use std::io::{BufRead, BufReader};

/// Reads text from a .txt file
pub fn read_text_from_file(filename: &str) -> String {
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut text = String::new();
    for line in reader.lines() {
        text.push_str(&line.unwrap());
        text.push(' '); // Preserve spacing
    }

    text
}

/// Splits text into overlapping chunks
pub fn chunk_text(text: &str, chunk_size: usize, overlap: usize) -> Vec<String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut chunks = Vec::new();
    let mut start = 0;

    while start < words.len() {
        let end = std::cmp::min(start + chunk_size, words.len());
        let chunk = words[start..end].join(" ");
        chunks.push(chunk);

        if end == words.len() {
            break;
        }

        start += chunk_size - overlap;
    }

    chunks
}
