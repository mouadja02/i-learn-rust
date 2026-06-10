use markitdown::{model::ConversionOptions, MarkItDown};
use std::env;

pub fn pdf_to_markdown(pdf_path: &str, llm_model: &str) -> Option<String> {
    let api_key = env::var("NVIDIA_API_KEY").unwrap_or_default();

    // NVIDIA's API is OpenAI-compatible; expose credentials where rig's openai provider expects them.
    // SAFETY: this runs in a single-threaded CLI context; no other threads read env vars concurrently.
    if !api_key.is_empty() {
        unsafe {
            env::set_var("OPENAI_API_KEY", &api_key);
            env::set_var("OPENAI_BASE_URL", "https://integrate.api.nvidia.com/v1");
        }
    }

    let md = MarkItDown::new();

    let options = ConversionOptions {
        file_extension: Some(".pdf".to_string()),
        url: None,
        llm_client: Some("openai".to_string()),
        llm_model: Some(llm_model.to_string()),
    };

    let result = md.convert(pdf_path, Some(options)).ok()??;
    Some(result.text_content)
}