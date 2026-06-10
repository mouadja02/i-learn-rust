use sha2::{Digest, Sha256};
use serde::{Deserialize, Serialize};

pub trait Embedder {
    fn embed_texts(&self, texts: &[String]) -> Vec<Vec<f32>>;
    fn embed_query(&self, query: &str) -> Vec<f32>;
}

pub struct HashEmbedder;

pub struct NvidiaEmbedder {
    pub model: String,
    pub base_url: String,
    pub api_key: String,
    client: reqwest::blocking::Client,
}

// ── Request / response shapes (OpenAI-compatible) ─────────────────────────────

#[derive(Serialize)]
struct EmbedRequest {
    model: String,
    input: Vec<String>,
    encoding_format: String,
}

#[derive(Deserialize)]
struct EmbedResponse {
    data: Vec<EmbedData>,
}

#[derive(Deserialize)]
struct EmbedData {
    embedding: Vec<f32>,
}

// ── HashEmbedder ──────────────────────────────────────────────────────────────

impl HashEmbedder {
    const EMBEDDING_DIM: usize = 384;

    fn hash_embed_texts(&self, texts: &[String]) -> Vec<Vec<f32>> {
        let mut embeddings: Vec<Vec<f32>> = Vec::new();
        for text in texts {
            let hash: [u8; 32] = Sha256::digest(text.as_bytes()).into();
            // Expand seed to EMBEDDING_DIM bytes using successive SHA-256 blocks
            let mut raw: Vec<u8> = Vec::new();
            let mut counter: usize = 0;
            while raw.len() < Self::EMBEDDING_DIM {
                let block: [u8; 32] = {
                    let mut hasher = Sha256::new();
                    hasher.update(&hash);
                    hasher.update(counter.to_le_bytes());
                    hasher.finalize().into()
                };
                raw.extend_from_slice(&block);
                counter += 1;
            }
            // Normalise bytes [0, 255] → [-1.0, ~0.992].
            let normalized: Vec<f32> = raw[..Self::EMBEDDING_DIM]
                .iter()
                .map(|&b| b as f32 / 128.0 - 1.0)
                .collect();
            embeddings.push(normalized);
        }
        embeddings
    }
}

impl Embedder for HashEmbedder {
    fn embed_texts(&self, texts: &[String]) -> Vec<Vec<f32>> {
        self.hash_embed_texts(texts)
    }

    fn embed_query(&self, query: &str) -> Vec<f32> {
        self.hash_embed_texts(&[query.to_string()])[0].clone()
    }
}

impl NvidiaEmbedder {
    pub fn new(model: Option<&str>, api_key: Option<&str>, base_url: Option<&str>) -> Self {
        Self {
            model: model.unwrap_or("nvidia/nv-embed-v1").to_string(),
            base_url: base_url.unwrap_or("https://integrate.api.nvidia.com/v1").to_string(),
            api_key: api_key.unwrap_or("").to_string(),
            client: reqwest::blocking::Client::new(),
        }
    }

    fn call_api(&self, inputs: Vec<String>) -> Result<Vec<Vec<f32>>, String> {
        let url = format!("{}/embeddings", self.base_url);
        let body = EmbedRequest {
            model: self.model.clone(),
            input: inputs,
            encoding_format: "float".to_string(),
        };
        let resp = self.client
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .map_err(|e| format!("Request failed: {}", e))?;

        let status = resp.status();
        let text = resp.text().map_err(|e| format!("Failed to read body: {}", e))?;

        if !status.is_success() {
            return Err(format!("API error {}: {}", status, text));
        }

        let parsed: EmbedResponse = serde_json::from_str(&text)
            .map_err(|e| format!("Failed to parse response: {} — body: {}", e, text))?;

        Ok(parsed.data.into_iter().map(|d| d.embedding).collect())
    }
}

impl Embedder for NvidiaEmbedder {
    fn embed_texts(&self, texts: &[String]) -> Vec<Vec<f32>> {
        match self.call_api(texts.to_vec()) {
            Ok(vecs) => vecs,
            Err(e) => {
                eprintln!("NvidiaEmbedder error: {}", e);
                texts.iter().map(|_| vec![]).collect()
            }
        }
    }

    fn embed_query(&self, query: &str) -> Vec<f32> {
        match self.call_api(vec![query.to_string()]) {
            Ok(mut vecs) => vecs.pop().unwrap_or_default(),
            Err(e) => {
                eprintln!("NvidiaEmbedder error: {}", e);
                vec![]
            }
        }
    }
}

pub fn get_embedder(model: Option<&str>, api_key: Option<&str>, base_url: Option<&str>) -> Box<dyn Embedder> {
    if api_key.is_none() || model == Some("hash") {
        Box::new(HashEmbedder)
    } else {
        Box::new(NvidiaEmbedder::new(model, api_key, base_url))
    }
}