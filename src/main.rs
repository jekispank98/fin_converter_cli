mod file_handler;
mod args;

use clap::Parser;
use std::io;
use std::path::{Path, PathBuf};
use crate::args::Args;

fn main() {
let args = Args::parse();
resolve_action(args);    
}
