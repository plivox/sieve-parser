use clap::Parser;
use std::io::{self, prelude::*};
use std::path::PathBuf;

use lib::grammar::ast::node::tree;
use lib::grammar::parser::parse;

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
pub struct Cli {
    /// Sieve file path
    #[arg(short, long, value_name = "FILE", required = true)]
    file: PathBuf,
}

fn read_pipe() -> Option<String> {
    let mut buf = String::new();

    if atty::isnt(atty::Stream::Stdin) {
        std::io::stdin().read_to_string(&mut buf).ok()?;
    }

    (!buf.is_empty()).then_some(buf.trim().into())
}

fn main() {
    let mut contents = String::new();

    if let Some(pipe) = read_pipe() {
        contents = pipe;
    } else {
        let cli = Cli::parse();

        if !cli.file.is_file() {
            println!("File '{}' not found", cli.file.display());
            std::process::exit(1);
        }

        if let Ok(mut file) = std::fs::File::open(&cli.file) {
            file.read_to_string(&mut contents).unwrap();
        }
    }

    let pairs = parse(&contents).unwrap_or_else(|e| panic!("{}", e));
    let nodes = tree(pairs.clone(), vec![]);

    io::stdout()
        .write_all(serde_json::to_string_pretty(&nodes).unwrap().as_bytes())
        .unwrap();
}
