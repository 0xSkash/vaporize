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
    /// Create models, migrations and controller classes
    Make(MakeCommand),
    /// Generate models based on a given DB schema
    GenerateModelsFromSchema(GenerateModelsFromSchemaArgs),
}

#[derive(Debug, Args)]
pub struct MakeCommand {
    #[clap(subcommand)]
    pub command: MakeSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum MakeSubcommand {
    /// Create a new model class
    Model(MakeArgs),
    /// Create a new migration class
    Migration(MakeArgs),
    /// Create a new controller class
    Controller(MakeArgs),
    /// Create a new model, migration and controller class
    All(MakeArgs),
    /// Generate a Vapor-Model from an existing migration
    ModelFromMigration(MakeArgs),
    /// Generate a Vapor-Migration from an existing model
    MigrationFromModel(MakeArgs)
}

#[derive(Debug, Args)]
pub struct MakeArgs {
    #[arg(short, long)]
    /// Path pointing to a Vapor-Project directory
    pub project_directory: PathBuf,
    #[arg(short, long)]
    /// The name of the class
    pub name: String
}

#[derive(Debug, Args)]
pub struct GenerateORMKeysArgs {
    #[arg(short, long)]
    /// Path pointing to a Vapor-Project directory
    pub project_directory: PathBuf,
}

#[derive(Debug, Args)]
pub struct GenerateModelsFromSchemaArgs {
    #[arg(short, long)]
    /// Path pointing to a Vapor-Project directory
    pub project_directory: PathBuf,
}

#[derive(Debug, Args)]
pub struct GenerateSwaggerDocsArgs {
    #[arg(short, long)]
    /// Path pointing to a Vapor-Project directory
    pub project_directory: PathBuf,
}
