# i-learn-rust

A learning repository for mastering Rust through building small projects and tools.

## Overview

This repository contains various CLI tools and utilities built with Rust, designed as hands-on learning exercises to deepen knowledge of the Rust programming language.

## Projects

### 1. LogPeek

A CLI tool for quickly inspecting and filtering log files.

**Features:**
- Filter lines by level: `--errors`, `--debug`
- Keyword search with `--contains`
- Tail output with `--last N`
- JSON summary of log-level counts with `--json-summary`

**Run:**
```bash
cargo run --bin logpeek -- logs/app.log --contains "timeout"
cargo run --bin logpeek -- logs/app.log --errors --last 50
```

---

### 2. CsvProf

A CLI tool for profiling and exploring CSV files.

**Features:**
- Select columns with `--columns`
- Filter rows with `--filter`
- Sort by a column (with optional `asc`/`desc`) using `--sort`
- Limit output rows with `--limit`
- Display schema with `--schema`

**Run:**
```bash
cargo run --bin csvprof -- data/sample.csv --schema
cargo run --bin csvprof -- data/sample.csv --sort "age:desc" --limit 10
```

---

### 3. Nvader — NVIDIA Agentic Research Engineer CLI

A production-style RAG (Retrieval-Augmented Generation) CLI that indexes documents, embeds them with the NVIDIA embedding API, and performs semantic search.

**Features:**
- Indexes `.txt`, `.md`, and `.pdf` files (single file or full directory, recursively)
- PDF → Markdown conversion via `markitdown` with LLM-assisted image description
- Semantic search using NVIDIA `nv-embed-v1` embeddings (falls back to a SHA-256 hash embedder when no API key is set)
- Global re-ranking across all indexed files
- JSON output with `--output`
- Project info and roadmap subcommands

**Environment:**
```
NVIDIA_API_KEY=<your key>   # in a .env file at the workspace root
```

**Run:**
```bash
cargo run --bin nvader -- search test_data "agentic AI certification" --top-k 5
cargo run --bin nvader -- search test_data/sample.md "RISC-V" --chunk-size 200 --chunk-overlap 40
cargo run --bin nvader -- search test_data "RAG pipeline" --top-k 10 --output results.json
cargo run --bin nvader -- info
cargo run --bin nvader -- roadmap
```

---

## Getting Started

Each project lives under `src/<name>/` with its own `Cargo.toml`.

### Build all

```bash
cargo build          # from workspace root
```

### Build a specific tool

```bash
cargo build --bin nvader
cargo build --bin logpeek
cargo build --bin csvprof
```

## Goal

Learn and practice Rust by building practical tools that solve real-world problems in software development and AI engineering.

## License

MIT
