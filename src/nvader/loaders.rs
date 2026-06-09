use super::core::{Document, DocumentType};
use std::fs;
use std::str;

fn read_file(path: &str) -> Result<Vec<u8>, std::io::Error> {
    match fs::read(path) {
        Ok(data) => Ok(data),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            panic!("File not found: '{}'", path)
        }
        Err(e) => Err(e),
    }
}

pub fn load_text_file(path: &str) -> Option<Document> {
    match read_file(path) {
        Ok(data) => {
            println!("File read successfully. Size: {} bytes", data.len());
            match str::from_utf8(&data){
                Ok(text) => {
                    let doc = Document::new(
                        path.to_string(),
                        text.to_string(),
                        "local".to_string(),
                        DocumentType::TEXT,
                        std::collections::HashMap::new(),
                    );
                    Some(doc)
                },
                Err(_) => {
                    println!("File content is not valid UTF-8.");
                    None
                },
            }
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            None
        }
    }
}


pub fn load_markdown_file(path: &str) -> Option<Document> {
    match read_file(path) {
        Ok(data) => {
            println!("File read successfully. Size: {} bytes", data.len());
            match str::from_utf8(&data){
                Ok(text) => {
                    let doc = Document::new(
                        path.to_string(),
                        text.to_string(),
                        "local".to_string(),
                        DocumentType::MARKDOWN,
                        std::collections::HashMap::new(),
                    );
                    Some(doc)
                },
                Err(_) => {
                    println!("File content is not valid UTF-8.");
                    None
                },
            }
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            None
        }
    }
}
