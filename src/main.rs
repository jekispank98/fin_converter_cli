mod action_resolver;
mod args;

use crate::action_resolver::resolve_action;
use crate::args::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();
    resolve_action(args);
}
