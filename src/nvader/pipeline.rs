use std::path::{Path, PathBuf};

use super::loaders::load_file;
use super::embeddings::Embedder;
use super::chunking::chunk_document;
use super::vector_store::InMemoryVectorStore;
use super::pdf2md::pdf_to_markdown;

fn collect_files_recursive(dir: &Path, supported_suffixes: &[&str]) -> Vec<PathBuf> {
    let mut result = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                result.extend(collect_files_recursive(&path, supported_suffixes));
            } else if path.is_file() {
                let ext = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .map(|e| format!(".{}", e.to_lowercase()))
                    .unwrap_or_default();
                if supported_suffixes.contains(&ext.as_str()) {
                    result.push(path);
                }
            }
        }
    }
    result
}

fn supported_files(path: &str) -> Result<Vec<PathBuf>, String> {
    let path = Path::new(path);
    let supported_suffixes = [".txt", ".md", ".pdf"];

    if !path.exists() {
        return Err(format!("Path does not exist: {}", path.display()));
    }

    let files: Vec<PathBuf> = if path.is_file() {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| format!(".{}", e.to_lowercase()))
            .unwrap_or_default();
        if supported_suffixes.contains(&ext.as_str()) {
            vec![path.to_path_buf()]
        } else {
            vec![]
        }
    } else {
        let mut files = collect_files_recursive(path, &supported_suffixes);
        files.sort();
        files
    };

    if files.is_empty() {
        return Err(format!("No supported files found at: {}", path.display()));
    }

    Ok(files)
}

pub fn build_vector_store_from_path(path: &str, chunking_size: usize, chunk_overlap: usize, embedder: Box<dyn Embedder>, llm_model: &str, reconvert: bool) -> Option<InMemoryVectorStore> {
    let files = supported_files(path).ok()?;
    let mut store = InMemoryVectorStore::new(embedder);

    for file in files {
        let load_path: PathBuf = if file
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.eq_ignore_ascii_case("pdf"))
            .unwrap_or(false)
        {
            let md_path = file.with_extension("md");
            if reconvert || !md_path.exists() {
                if let Some(md_content) = pdf_to_markdown(file.to_str()?, llm_model) {
                    let _ = std::fs::write(&md_path, md_content);
                }
            }
            md_path
        } else {
            file
        };

        if let Some(document) = load_file(load_path.to_str()?) {
            let chunks = chunk_document(document, chunking_size, chunk_overlap);
            store.add_chunks(chunks);
        }
    }

    Some(store)
}

pub fn search_path(path: &str, query: &str, top_k: usize, chunking_size: usize, chunk_overlap: usize, embedder: Box<dyn Embedder>, llm_model: &str, reconvert: bool) -> Option<Vec<super::vector_store::RetrievalResult>> {
    let store = build_vector_store_from_path(path, chunking_size, chunk_overlap, embedder, llm_model, reconvert)?;
    Some(store.search(query, top_k))
}