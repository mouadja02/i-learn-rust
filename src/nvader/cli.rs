use std::env;
use std::path::PathBuf;
use std::process;

use clap::{Parser, Subcommand};
use colored::*;

use super::chunking::chunk_document;
use super::embeddings::get_embedder;
use super::loaders::{load_markdown_file, load_text_file};
use super::vector_store::InMemoryVectorStore;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Parser)]
#[command(name = "nvader", about = "NVIDIA Agentic Research Engineer CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show project identity and current configuration
    Info,
    /// Print the two-month certification build roadmap
    Roadmap,
    /// Load a file, chunk it, and search with a query
    Search {
        /// File to index and search
        file: PathBuf,
        /// Search query
        query: String,
        /// Number of results to return
        #[arg(short = 'k', long = "top-k", default_value_t = 5)]
        top_k: usize,
        /// Characters per chunk
        #[arg(long, default_value_t = 500)]
        chunk_size: usize,
        /// Overlap between chunks
        #[arg(long, default_value_t = 100)]
        chunk_overlap: usize,
    },
}

pub fn banner() {
    let art = [
        "███╗   ██╗ ██╗   ██╗  █████╗  ██████╗  ███████╗ ██████╗",
        "████╗  ██║ ██║   ██║ ██╔══██╗ ██╔══██╗ ██╔════╝ ██╔══██╗",
        "██╔██╗ ██║ ██║   ██║ ███████║ ██║  ██║ █████╗   ██████╔╝",
        "██║╚██╗██║ ╚██╗ ██╔╝ ██╔══██║ ██║  ██║ ██╔══╝   ██╔══██╗",
        "██║ ╚████║  ╚████╔╝  ██║  ██║ ██████╔╝ ███████╗ ██║  ██║",
        "╚═╝  ╚═══╝   ╚═══╝   ╚═╝  ╚═╝ ╚═════╝  ╚══════╝ ╚═╝  ╚═╝",
    ];

    for line in &art {
        println!("{}", line.green());
    }
    println!("{}", "Your specialized data-engineering CLI agent".dimmed());
    println!("{}", format!("v{}  ·  {}", VERSION, PKG_NAME).dimmed());
    println!();
}

pub fn run(cli: Cli) {
    match cli.command {
        Commands::Info => cmd_info(),
        Commands::Roadmap => cmd_roadmap(),
        Commands::Search { file, query, top_k, chunk_size, chunk_overlap } => {
            cmd_search(file, &query, top_k, chunk_size, chunk_overlap);
        }
    }
}

fn cmd_info() {
    banner();
    println!("{}", "── Project Info ──".bold());
    println!("  Name:              {}", PKG_NAME.green().bold());
    println!("  Version:           {}", VERSION);
    println!("  Description:       Production-style Agentic AI Research and Engineering Assistant.");
    println!("                     Built for NVIDIA Agentic AI certification preparation.");
    println!();
}

fn cmd_roadmap() {
    println!("{}", "Two-Month Roadmap".bold());
    println!("  Week 1: Foundation and architecture");
    println!("  Week 2: RAG and knowledge integration");
    println!("  Week 3: ReAct and tool orchestration");
    println!("  Week 4: Planning and memory");
    println!("  Week 5: Multi-agent workflows");
    println!("  Week 6: Evaluation and tuning");
    println!("  Week 7: NVIDIA platform and deployment");
    println!("  Week 8: Monitoring, safety, HITL, final exam prep");
}

fn cmd_search(file: PathBuf, query: &str, top_k: usize, chunk_size: usize, chunk_overlap: usize) {
    let path_str = file.to_string_lossy();
    let extension = file.extension().and_then(|e| e.to_str()).unwrap_or("");

    let document = match extension {
        "md" | "markdown" => load_markdown_file(&path_str),
        "txt" => load_text_file(&path_str),
        _ => {
            eprintln!("{} Unsupported file type: .{}. Supported: .md, .txt", "Error:".red().bold(), extension);
            process::exit(1);
        }
    };

    let document = match document {
        Some(doc) => doc,
        None => {
            eprintln!("{} Failed to load file: {}", "Error:".red().bold(), path_str);
            process::exit(1);
        }
    };

    let chunks = chunk_document(document, chunk_size, chunk_overlap);
    if chunks.is_empty() {
        println!("{}", "No chunks produced — file may be empty.".yellow());
        return;
    }

    // Load .env file (silently ignore if missing)
    dotenv::dotenv().ok();
    let api_key = env::var("NVIDIA_API_KEY").ok();
    let embedder = get_embedder(None, api_key.as_deref(), None);

    let mut store = InMemoryVectorStore::new(embedder);
    println!("Embedding {} chunk(s)...", chunks.len());
    store.add_chunks(chunks);

    let results = store.search(query, top_k);
    if results.is_empty() {
        println!("{}", "No results found.".yellow());
        return;
    }

    // Print results table
    println!();
    println!("{}", format!("Search results for \"{}\"", query).bold());
    println!("{:-<80}", "");
    println!(
        "{:<6} {:<8} {:<20} {:<12} {}",
        "Rank".bold(),
        "Score".bold(),
        "Source".bold(),
        "Offsets".bold(),
        "Preview".bold(),
    );
    println!("{:-<80}", "");

    for r in &results {
        let preview: String = r.chunk.text.chars().take(120).collect::<String>().replace('\n', " ");
        let preview = if r.chunk.text.len() > 120 {
            format!("{}…", preview.trim())
        } else {
            preview.trim().to_string()
        };
        let offsets = format!("{}–{}", r.chunk.start_char, r.chunk.end_char);

        println!(
            "{:<6} {:<8.4} {:<20} {:<12} {}",
            r.rank,
            r.score,
            r.chunk.source,
            offsets,
            preview,
        );
    }
    println!("{:-<80}", "");
}
