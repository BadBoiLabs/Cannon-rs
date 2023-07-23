use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(bin_name = "cargo", version, author)]
pub enum CargoSubcommand {
    /// Tools for using Rust for the Optimism Cannon target
    #[command(subcommand, name = "cannon", version, author)]
    Cannon(Cannon),
}

#[derive(Subcommand)]
pub enum Cannon {
    /// Build a Cannon compatible program
    #[command(name = "build", version, author)]
    Build(Build),

    /// Create a new Cannon Rust project
    #[command(name = "new", version, author)]
    New(New),
}

#[derive(Parser, Debug)]
pub struct Build {
    /// Space-separated list of features to activate
    #[arg(long, value_name = "FEATURES")]
    pub features: Option<String>,

    /// Activate all available features
    #[arg(long)]
    pub all_features: bool,

    /// Do not activate the `default` feature
    #[arg(long)]
    pub no_default_features: bool,

    /// Directory for all generated artifacts
    #[arg(long, value_name = "DIRECTORY")]
    pub target_dir: Option<PathBuf>,

    /// Path to Cargo.toml
    #[arg(long, value_name = "PATH")]
    pub manifest_path: Option<PathBuf>,

    /// Package to expand
    #[arg(short, long, value_name = "SPEC", num_args = 0..=1)]
    pub package: Option<Option<String>>,
}

#[derive(Parser, Debug)]
pub struct New {
    /// Path to create the new Cannon project
    pub path: PathBuf,
}
