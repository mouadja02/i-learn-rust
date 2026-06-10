#![allow(dead_code)]
mod core;
mod loaders;
mod chunking;
mod embeddings;
mod vector_store;
mod cli;
mod pdf2md;
mod pipeline;

use clap::Parser;
use cli::Cli;

fn main() {
    let args = Cli::parse();
    cli::run(args);
}