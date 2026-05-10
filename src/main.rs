mod cli;
mod commands;
mod error;
mod linter;
mod parser;
mod templates;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let interactive = cli.interactive;
    match cli.command {
        Commands::Init { lang, force } => {
            commands::init::execute(&cli.file, lang, force, interactive)
        }
        Commands::Add { templates } => commands::add::execute(&cli.file, &templates, interactive),
        Commands::Remove { targets } => commands::remove::execute(&cli.file, &targets, interactive),
        Commands::List { targets, lang } => commands::list::execute(&cli.file, targets, lang),
        Commands::Lint => commands::lint::execute(&cli.file),
        Commands::Fmt { check } => commands::fmt::execute(&cli.file, check),
        Commands::Validate => commands::validate::execute(&cli.file),
        Commands::Tips => commands::tips::execute(),
        Commands::Completions { shell } => commands::completions::execute(shell),
    }
}
