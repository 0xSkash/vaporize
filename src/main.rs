use clap::Parser;

use crate::cli::args::{Command, GenerateORMKeysArgs, VaporizeArgs};

mod cli;

fn main() {
    let args = VaporizeArgs::parse();

    match args.command {
        Command::GenerateORMKeys(args) => {
            handle_generate_orm_keys(&args)
        }
    }
}

fn handle_generate_orm_keys(args: &GenerateORMKeysArgs) {
    if !args.project_directory.exists() || !args.project_directory.is_dir() {
        eprintln!("Error: The specified project directory is not a valid path.");
        return;
    }
    println!("Generating ORM keys using project directory: {:?}", args.project_directory);
}