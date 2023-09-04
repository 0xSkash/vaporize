use std::path::PathBuf;
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct VaporizeArgs {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Generate Swift-Enums holding keys for the ORM PropertyWrapper Keys based on model property names
    GenerateORMKeys(GenerateORMKeysArgs),
}

#[derive(Debug, Args)]
pub struct GenerateORMKeysArgs {
    #[arg(short, long)]
    /// Path pointing to a Vapor-Project directory
    pub project_directory: PathBuf,
}

