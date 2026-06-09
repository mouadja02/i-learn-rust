use super::core::{Document, DocumentChunk};
use std::{cmp::{min}};

fn chunk_text(text: &str, chunk_size: usize, chunk_overlap: usize) -> Vec<(String, usize, usize)> {
    if chunk_size == 0 {
        panic!("chunk_size must be greater than 0");
    }
    if chunk_overlap >= chunk_size {
        panic!("chunk_overlap must be smaller than chunk_size");
    }

    let chars: Vec<char> = text.chars().collect();
    let text_len = chars.len();
    let length = text.len();
    print!("text.length {} chars.length {}", length, text_len);
    let mut chunks = Vec::new();
    let mut start = 0;

    while start < text_len {
        let end = min(start + chunk_size, text_len);
        if start == end {
            break;
        }
        let text_chunk: String = chars[start..end].iter().collect();
        chunks.push((text_chunk, start, end));

        start += chunk_size - chunk_overlap;
    }
    chunks
}

pub fn chunk_document(document: Document, chunk_size: usize , chunk_overlap: usize) -> Vec<DocumentChunk> {
    let text = document.content.as_str();
    let raw_chinks = chunk_text(text, chunk_size, chunk_overlap);

    let mut document_chunks: Vec<DocumentChunk> = Vec::new();
    for (index, (text_chunk, start_char, end_char)) in raw_chinks.into_iter().enumerate() {
        let chunk = DocumentChunk {
            id: format!("{}_chunk_{}", document.id, index),
            document_id: document.id.clone(),
            text: text_chunk,
            chunk_index: index,
            source: document.source.clone(),
            metadata: document.metadata.clone(),
            start_char,
            end_char,
        };
        document_chunks.push(chunk);
    }
    document_chunks
}

pub fn chunk_documents(documents: Vec<Document>, chunk_size: usize, chunk_overlap: usize) -> Vec<DocumentChunk> {
    let mut all_chunks: Vec<DocumentChunk> = Vec::new();
    for document in documents {
        let chunks = chunk_document(document, chunk_size, chunk_overlap);
        all_chunks.extend(chunks);
    }
    all_chunks
}
