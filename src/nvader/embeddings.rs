use sha2::{Digest, Sha256};
use openai_api_rust::*;
use openai_api_rust::embeddings::*;

pub trait Embedder {
    fn embed_texts(&self, texts: &[String]) -> Vec<Vec<f32>>;
    fn embed_query(&self, query: &str) -> Vec<f32>;
}

pub struct HashEmbedder;

pub struct NvidiaEmbedder {
    pub model: String,
    pub base_url: String,
    pub api_key: String,
    pub client: OpenAI,
}

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
        let resolved_key = api_key.unwrap_or("");
        let resolved_base = base_url.unwrap_or("https://integrate.api.nvidia.com/v1");
        let auth = Auth::new(resolved_key);
        Self {
            model: model.unwrap_or("nvidia/nv-embed-v1").to_string(),
            base_url: resolved_base.to_string(),
            api_key: resolved_key.to_string(),
            client: OpenAI::new(auth, resolved_base)
        }
    }
}

impl Embedder for NvidiaEmbedder {
    fn embed_texts(&self, texts: &[String]) -> Vec<Vec<f32>> {
        let mut results: Vec<Vec<f32>> = Vec::new();
        for text in texts {
            let embed_body = EmbeddingsBody {
                model: self.model.clone(),
                input: vec![text.clone()],
                user: None,
            };
            let response: ApiResult<Embeddings> = self.client.embeddings_create(&embed_body);
            match response {
                Ok(res) => {
                    let data = res.data.expect("embedding response should contain data");
                    let embedding = data.first().expect("embedding data should not be empty");
                    let vec = embedding.embedding.as_ref().expect("embedding vector should be present");
                    results.push(vec.iter().map(|&v| v as f32).collect());
                }
                Err(e) => {
                    eprintln!("Error embedding text: {}", e);
                    results.push(vec![]);
                }
            }
        }
        results
    }

    fn embed_query(&self, query: &str) -> Vec<f32> {
        let req_body = EmbeddingsBody {
            model: self.model.clone(),
            input: vec![query.to_string()],
            user: None,
        };
        let response: ApiResult<Embeddings> = self.client.embeddings_create(&req_body);
        match response {
            Ok(res) => {
                let data = res.data.expect("embedding response should contain data");
                let embedding = data.first().expect("embedding data should not be empty");
                embedding.embedding.as_ref().expect("embedding vector should be present")
                    .iter().map(|&v| v as f32).collect()
            }
            Err(e) => {
                eprintln!("Error embedding query: {}", e);
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