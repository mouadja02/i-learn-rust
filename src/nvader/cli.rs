use std::path::PathBuf;
use std::process;

use clap::{Parser, Subcommand};
use colored::*;

use super::embeddings::get_embedder;

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
    /// Index and search files or a directory
    Search {
        /// File or directory to index and search
        path: PathBuf,
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
        /// Reconvert PDF files to Markdown
        #[arg(long, default_value_t = false)]
        reconvert: bool,   
        /// Optional output path for search results (JSON)
        #[arg(short, long)]
        output: Option<PathBuf>,
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
        Commands::Search { path, query, top_k, chunk_size, chunk_overlap, reconvert, output } => {
            cmd_search(path, &query, top_k, chunk_size, chunk_overlap, reconvert, output);
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

fn cmd_search(path: PathBuf, query: &str, top_k: usize, chunk_size: usize, chunk_overlap: usize, reconvert: bool, output: Option<PathBuf>) {
    if query.trim().is_empty() {
        eprintln!("{} Query cannot be empty.", "Error:".red().bold());
        process::exit(1);
    }

    // Load .env file (silently ignore if missing)
    dotenv::dotenv().ok();
    let api_key = std::env::var("NVIDIA_API_KEY").ok();
    let embedder = get_embedder(None, api_key.as_deref(), None);

    let path_str = path.to_string_lossy();
    let llm_model = "nvidia/nemotron-nano-12b-v2-vl";

    let mut results = super::pipeline::search_path(
        &path_str,
        query,
        top_k,
        chunk_size,
        chunk_overlap,
        embedder,
        llm_model,
        reconvert,
    ).unwrap_or_else(|| {
        eprintln!("{} No supported files found at: {}", "Error:".red().bold(), path_str);
        process::exit(1);
    });

    // Sort globally by score descending and rebuild rank
    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    for (i, r) in results.iter_mut().enumerate() {
        r.rank = i + 1;
    }
    let results = &results[..results.len().min(top_k)];

    if results.is_empty() {
        println!("{}", "No results found.".yellow());
        return;
    }

    // Print results table
    println!();
    println!("{}", format!("Top {} results for \"{}\"", top_k, query).bold());
    println!("{:-<80}", "");
    println!(
        "{:<6} {:<10} {:<30} {:<14} {}",
        "Rank".bold(),
        "Score".bold(),
        "Source".bold(),
        "Offsets".bold(),
        "Preview".bold(),
    );
    println!("{:-<80}", "");

    for r in results {
        // Strip \r and \n to prevent carriage-return overwrites on Windows terminals
        let clean_text = r.chunk.text.replace('\r', "").replace('\n', " ");
        let preview: String = clean_text.chars().take(120).collect();
        let preview = if r.chunk.text.chars().count() > 120 {
            format!("{}…", preview.trim())
        } else {
            preview.trim().to_string()
        };
        let offsets = format!("{}–{}", r.chunk.start_char, r.chunk.end_char);
        // Truncate source to fit the column without overflowing
        let source: String = r.chunk.source.chars().rev()
            .take(28).collect::<String>().chars().rev().collect();
        let source = if r.chunk.source.chars().count() > 28 {
            format!("…{}", source)
        } else {
            source
        };

        println!(
            "{:<6} {:<10.4} {:<30} {:<14} {}",
            r.rank,
            r.score,
            source,
            offsets,
            preview,
        );
    }
    println!("{:-<80}", "");

    if let Some(out_path) = output {
        if let Some(parent) = out_path.parent() {
            if !parent.as_os_str().is_empty() {
                let _ = std::fs::create_dir_all(parent);
            }
        }
        let payload: Vec<serde_json::Value> = results.iter().map(|r| {
            serde_json::json!({
                "rank": r.rank,
                "score": r.score,
                "source": r.chunk.source,
                "start_char": r.chunk.start_char,
                "end_char": r.chunk.end_char,
                "chunk_id": r.chunk.id,
                "document_id": r.chunk.document_id,
                "chunk_index": r.chunk.chunk_index,
                "text": r.chunk.text,
                "metadata": r.chunk.metadata,
            })
        }).collect();
        match serde_json::to_string_pretty(&payload) {
            Ok(json_str) => match std::fs::write(&out_path, json_str) {
                Ok(_) => println!("{}", format!("Results saved to {}", out_path.display()).dimmed()),
                Err(e) => eprintln!("{} Failed to write output: {}", "Error:".red().bold(), e),
            },
            Err(e) => eprintln!("{} Failed to serialize results: {}", "Error:".red().bold(), e),
        }
    }
}
