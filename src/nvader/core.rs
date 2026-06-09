#![allow(dead_code)]

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, BTreeMap};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocumentType {
    TEXT,
    MARKDOWN,
    HTML,
    JSON,
    PDF
}

impl DocumentType {
    fn mime_type(&self) -> &'static str {
        match self {
            DocumentType::TEXT => "text/plain",
            DocumentType::MARKDOWN => "text/markdown",
            DocumentType::HTML => "text/html",
            DocumentType::JSON => "application/json",
            DocumentType::PDF => "application/pdf",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub title: String,
    pub content: String,
    pub source: String,
    pub document_type: DocumentType,
    pub metadata: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
}

impl Document {
    pub fn new(title: String, content: String, source: String, document_type: DocumentType, metadata: HashMap<String, String>) -> Self {
        let mut hash_map: BTreeMap<&str, String> = BTreeMap::new(); // use BTreeMap instead of HashMap for consistent ordering
        hash_map.insert("title", title.clone());
        hash_map.insert("content", content.clone());
        hash_map.insert("source", source.clone());
        hash_map.insert("document_type", format!("{:?}", document_type));
        hash_map.insert("metadata", serde_json::to_string(&metadata).unwrap_or_default());

        let hash_input = serde_json::to_string(&hash_map).unwrap_or_default();
        let id = format!("{:x}", Sha256::digest(hash_input.as_bytes()));

        Document {
            id,
            title,
            content,
            source,
            document_type,
            metadata,
            created_at: Utc::now(),
        }
    }

    pub fn short_preview(&self, max_chars: usize) -> String {
        let cleaned = self.content
            .split_whitespace().collect::<Vec<_>>()
            .join(" ");
        if cleaned.len() > max_chars {
            format!("{}...", &cleaned[..max_chars])
        }
        else {
            cleaned
        }
        
    }


}

#[derive(Debug, Clone)]
pub struct DocumentChunk {
    pub id: String,
    pub document_id: String,
    pub text: String,
    pub chunk_index: usize,
    pub source: String,
    pub metadata: HashMap<String, String>,
    pub start_char: usize,
    pub end_char: usize,
}