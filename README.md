
# 🧠 Vectura: A Simple Vector Database in Rust

Vectura is a lightweight vector database implemented in Rust, designed to handle high-dimensional data for efficient similarity search. This project showcases the integration of Rust's performance capabilities with retrieval, making it a valuable resource for the ever increasing demand for higher retrieval speeds.

---


https://github.com/user-attachments/assets/db950384-7eb4-4301-96b2-62b2d2d0211a




## 📁 Project Structure

```
.
├── demo/                     # Demo video of working
├── models/                   # Directory containing model files
├── src/                      # Source code directory
│   ├── main.rs               # Entry point of the application
│   ├── embed.rs              # Module for embedding logic
│   ├── hnsw.rs               # Module implementing HNSW algorithm
│   └── utils.rs              # Utility functions
├── tokenizer/                # Directory containing tokenizer files
├── .gitignore                # Git ignore file
├── Cargo.lock                # Cargo lock file
├── Cargo.toml                # Cargo configuration file
├── embeddings.bin            # Binary file storing embeddings
├── story.txt                 # Sample text file for embedding
└── tokenizer.json            # JSON file for tokenizer configuration
```

---

## 🛠️ Setup and Installation

1. **Prerequisites**:
   - Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your system.

2. **Clone the Repository**:
   ```bash
   git clone https://github.com/SohamG934/vectura.git
   cd vectura
   ```

3. **Build the Project**:
   ```bash
   cargo build --release
   ```

4. **Run the Application**:
   ```bash
   cargo run --release
   ```

---

## 📊 How It Works

Vectura works by converting input text into vector embeddings using the `rust-bert` library, which are then stored on disk in binary format. For similarity search, it utilizes the **Hierarchical Navigable Small World (HNSW)** algorithm to efficiently retrieve the most relevant data points.

---

## 🧩 Key Features

- **Text Embedding**: Uses `rust-bert` to generate dense vector embeddings for input text.
- **Efficient Similarity Search**: Implements the HNSW algorithm for fast nearest-neighbor search.
- **Persistent Storage**: Stores embeddings on disk (`embeddings.bin`) for efficient reuse and retrieval. Embeddings are created once at the beginning.
- **Customizable Parameters**: Supports adjustable chunk sizes and overlap for text chunking.
- **Custom tokenizer Script**: A sample script to create your own rust tokenizer using the GPT2-vocab. (In case ALl-mini-LM-L12V2 is boring😊)

---

## 📌 Usage

### 1. Embedding Text and Storing Vectors

You can embed a text file and store the resulting vectors on disk.

1. Place your text data in `story.txt`.
2. Run the application:
   ```bash
   cargo run --release
   ```

The embeddings will be saved in `embeddings.bin`.

---

### 2. Querying with HNSW

After generating and saving embeddings, you can query the database to find the most similar responses.

Example:
```
Enter your query:
What did the journal do?

Best response:
"The journal detailed his experiments, his growing isolation, and a haunting tale of a clock that began to tick backwards, signaling an impending doom."
```

---

## ⚙️ Configuration

You can customize the chunking size and overlap for text processing:

- **Chunk Size**: Controls how many characters are processed per chunk.
- **Overlap**: Determines the overlap between consecutive chunks to preserve context.

Update these values in `embed.rs`:
```rust
const CHUNK_SIZE: usize = 512;
const OVERLAP: usize = 50;
```

---

## 📦 Dependencies

The main dependencies of Vectura are:

```toml
[dependencies]
rust-bert = "0.19.0"
tokenizers = "0.13.3"
ndarray = "0.15.4"
rand = "0.8.5"
bincode = "1.3.3"
```

---

## 📚 Example Output

```
Enter your query:
What did Silas do?

Best response:
"The journal detailed his experiments, his growing isolation, and a haunting tale of a clock that began to tick backwards, signaling an impending doom."
```

---

## 📊 Understanding HNSW

The **Hierarchical Navigable Small World (HNSW)** algorithm allows for fast and efficient similarity searches by building a multi-layer graph:

1. **Insertion**: Each new point is inserted by traversing from the highest layer downwards.
2. **Search**: For a given query, the algorithm performs a greedy search to find the closest neighbors.

---

## 🧪 Running Tests

To ensure everything works correctly, run:

```bash
cargo test
```

---

## 🤝 Contributing

Contributions are welcome! Feel free to submit a pull request or open an issue.

---

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

## ☕ Support Me

If you find Vectura helpful, consider supporting my work!

[![Buy Me a Coffee](https://img.shields.io/badge/Buy%20Me%20a%20Coffee-Support%20My%20Work-orange?style=for-the-badge&logo=buymeacoffee)]([https://www.buymeacoffee.com/sohamg934](https://buymeacoffee.com/soham_ghadge))

---
```
