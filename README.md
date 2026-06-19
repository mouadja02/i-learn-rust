# i-learn-rust

A hands-on Rust learning repository — from fundamentals to production-style CLI tools.

## Repository Structure

```
i-learn-rust/
├── Cargo.toml              # Workspace manifest
├── src/
│   ├── guessing_game/      # Beginner project (The Rust Book Ch.2)
│   ├── logpeek/            # Log file inspector CLI
│   ├── csvprof/            # CSV profiler CLI
│   └── nvader/             # RAG search engine CLI (NVIDIA embeddings)
├── rustlings/              # Rustlings exercises (24 topic modules)
├── data/                   # Sample CSV data
├── logs/                   # Sample log files
└── test_data/              # Documents for nvader indexing (.md, .txt, .pdf)
```

---

## Projects

### 1. Guessing Game

The classic introductory Rust project from *The Rust Programming Language* book.

| Concept practiced | Details |
|---|---|
| Ownership & mutability | `let mut guess` |
| Error handling | `match` on `parse()` result |
| External crates | `rand` for RNG |
| Control flow | `loop` + `break` |

```bash
cargo run --bin guessing_game
```

---

### 2. LogPeek

A CLI tool for quickly inspecting and filtering log files.

| Crate | Purpose |
|---|---|
| `clap` | Argument parsing (derive) |
| `anyhow` | Error handling |

**Features:**
- Filter lines by level: `--errors`, `--debug`
- Keyword search with `--contains`
- Tail output with `--last N`
- JSON summary of log-level counts with `--json-summary`

```bash
cargo run --bin logpeek -- logs/app.log --contains "timeout"
cargo run --bin logpeek -- logs/app.log --errors --last 50
```

---

### 3. CsvProf

A CLI tool for profiling and exploring CSV files.

| Crate | Purpose |
|---|---|
| `clap` | Argument parsing (derive) |
| `anyhow` | Error handling |
| `tempfile` | Test fixtures (dev) |

**Features:**
- Select columns with `--columns`
- Filter rows with `--filter`
- Sort by a column (`asc`/`desc`) with `--sort`
- Limit output rows with `--limit`
- Display schema with `--schema`

```bash
cargo run --bin csvprof -- data/sample.csv --schema
cargo run --bin csvprof -- data/sample.csv --sort "age:desc" --limit 10
```

---

### 4. Nvader — NVIDIA Agentic Research Engineer CLI

A production-style RAG (Retrieval-Augmented Generation) CLI that indexes documents, embeds them with the NVIDIA embedding API, and performs semantic search.

| Crate | Purpose |
|---|---|
| `clap` | CLI framework |
| `reqwest` | HTTP client (NVIDIA API) |
| `serde` / `serde_json` | Serialization |
| `sha2` / `hex` | Fallback hash embedder |
| `markitdown` | PDF → Markdown conversion |
| `colored` | Terminal styling |
| `dotenv` | Environment config |

**Features:**
- Indexes `.txt`, `.md`, and `.pdf` files (single file or full directory, recursively)
- PDF → Markdown conversion via `markitdown` with LLM-assisted image description
- Semantic search using NVIDIA `nv-embed-v1` embeddings (falls back to SHA-256 hash embedder when no API key is set)
- Global re-ranking across all indexed files
- JSON output with `--output`
- Project info and roadmap subcommands

**Environment:**
```
NVIDIA_API_KEY=<your key>   # in a .env file at the workspace root
```

```bash
cargo run --bin nvader -- search test_data "agentic AI certification" --top-k 5
cargo run --bin nvader -- search test_data/sample.md "RISC-V" --chunk-size 200 --chunk-overlap 40
cargo run --bin nvader -- search test_data "RAG pipeline" --top-k 10 --output results.json
cargo run --bin nvader -- info
cargo run --bin nvader -- roadmap
```

---

## Rustlings Exercises

The workspace includes the full [Rustlings](https://github.com/rust-lang/rustlings) course — 24 topic modules covering Rust from basics to advanced:

| # | Topic | # | Topic |
|---|---|---|---|
| 00 | Intro | 12 | Options |
| 01 | Variables | 13 | Error Handling |
| 02 | Functions | 14 | Generics |
| 03 | If | 15 | Traits |
| 04 | Primitive Types | 16 | Lifetimes |
| 05 | Vecs | 17 | Tests |
| 06 | Move Semantics | 18 | Iterators |
| 07 | Structs | 19 | Smart Pointers |
| 08 | Enums | 20 | Threads |
| 09 | Strings | 21 | Macros |
| 10 | Modules | 22 | Clippy |
| 11 | Hashmaps | 23 | Conversions |

Run the exercises:
```bash
cd rustlings
rustlings
```

---

## Getting Started

### Prerequisites

- [Rust toolchain](https://rustup.rs/) (stable)
- `rustlings` CLI (for exercises): `cargo install rustlings`

### Build all workspace members

```bash
cargo build
```

### Build & run a specific tool

```bash
cargo run --bin logpeek -- --help
cargo run --bin csvprof -- --help
cargo run --bin nvader -- --help
cargo run --bin guessing_game
```

### Run tests

```bash
cargo test --workspace
```

---

## Learning Path

```mermaid
graph LR
    A[Rustlings Exercises] --> B[Guessing Game]
    B --> C[LogPeek]
    C --> D[CsvProf]
    D --> E[Nvader]
    style A fill:#2d333b,stroke:#539bf5
    style E fill:#2d333b,stroke:#539bf5
```

1. **Rustlings** — master syntax, ownership, traits, lifetimes
2. **Guessing Game** — first complete program (I/O, crates, error handling)
3. **LogPeek** — file I/O, string processing, CLI with `clap`
4. **CsvProf** — structured data parsing, sorting, filtering
5. **Nvader** — async HTTP, API integration, embeddings, modular architecture

---

## License

MIT
