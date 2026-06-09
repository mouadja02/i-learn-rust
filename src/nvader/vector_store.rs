use super::core::DocumentChunk;
use super::embeddings::Embedder;


pub struct RetrievalResult {
    pub chunk: DocumentChunk,
    pub score: f32,
    pub rank: usize,
}

pub fn cosine_similarity(vec_a: &[f32], vec_b: &[f32]) -> f32 {
    if vec_a.len() != vec_b.len() {
        panic!("Vectors must be of the same length for cosine similarity");
    }

    let dot_product: f32 = vec_a.iter().zip(vec_b).map(|(a, b)| a * b).sum();
    let norm_a = vec_a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b = vec_b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0; // Avoid division by zero
    }

    dot_product / (norm_a * norm_b)
}

pub struct InMemoryVectorStore {
    embedder: Box<dyn Embedder>,
    items: Vec<(DocumentChunk, Vec<f32>)>,
}

impl InMemoryVectorStore {
    pub fn new(embedder: Box<dyn Embedder>) -> Self {
        InMemoryVectorStore {
            items: Vec::new(),
            embedder,
        }
    }

    pub fn add_chunks(&mut self, chunks: Vec<DocumentChunk>) {
        let texts: Vec<String> = chunks.iter().map(|c| c.text.clone()).collect();
        let embeddings: Vec<Vec<f32>> = self.embedder.embed_texts(&texts);
        for (chunk, embedding) in chunks.into_iter().zip(embeddings.into_iter()) {
            self.items.push((chunk, embedding));
        }
    }

    pub fn search(&self, query: &str, top_k: usize) -> Vec<RetrievalResult> {
        if top_k == 0 {
            panic!("top_k must be greater than 0");
        }
        if self.items.is_empty() {
            return Vec::new();
        }
        let query_embedding = self.embedder.embed_query(query);
        let mut scored = Vec::new();
        for (chunk, embedding) in self.items.iter() {
            let score = cosine_similarity(&query_embedding, embedding);
            scored.push((chunk, score));
        }
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)); // Sort by score descending
        scored.into_iter().take(top_k).enumerate().map(|(i, (chunk, score))| RetrievalResult {
            chunk: chunk.clone(),
            score,
            rank: i + 1,
        }).collect()
    }

}