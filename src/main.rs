mod cli;
mod config;
mod system;

use clap::{command, Command};

fn main() {
    let cli = cli::build_cli();
    let matches = cli::get_matches(cli);
}
