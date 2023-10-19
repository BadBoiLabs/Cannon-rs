use crate::cli::CargoSubcommand;
use cargo_generate::{generate, GenerateArgs, TemplatePath, Vcs};
use clap::Parser;
use io::Write;
use std::ffi::OsString;
use std::{env, io, process};

mod cli;

fn main() {
    let result: Result<i32, _> = cargo_cannon();
    process::exit(match result {
        Ok(code) => code,
        Err(err) => {
            let _ = writeln!(io::stderr(), "{}", err);
            1
        }
    });
}

fn cargo_binary() -> OsString {
    env::var_os("CARGO").unwrap_or_else(|| "cargo".to_owned().into())
}

fn new(args: cli::New) {
    let cli::New { path } = args;

    let template_args = GenerateArgs {
        name: Some(path.to_string()),
        vcs: Some(Vcs::Git),
        template_path: TemplatePath {
            git: Some("https://github.com/BadBoiLabs/Cannon-rs.git".to_string()),
            ..TemplatePath::default()
        },
        ..GenerateArgs::default()
    };

    let path = generate(template_args).unwrap();
    println!("Created new Cannon project at {}", path.display());
}

fn build(_args: cli::Build) {}

fn cargo_cannon() -> Result<i32, anyhow::Error> {
    let CargoSubcommand::Cannon(args) = CargoSubcommand::parse();
    match args {
        cli::Cannon::Build(args) => build(args),
        cli::Cannon::New(args) => new(args),
    }

    // Cross compile the MIPS32 using our docker image

    // Patch the resulting elf and convert to a Cannon json file

    Ok(0)
}
