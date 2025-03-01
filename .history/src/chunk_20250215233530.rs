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
use text_splitter::TextSplitter;

pub fn chunk_text(text: &str) -> Vec<String> {
    let splitter = TextSplitter::new(50)
    splitter.chunks(text)
}

