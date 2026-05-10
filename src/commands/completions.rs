use anyhow::Result;
use clap::CommandFactory;
use clap_complete::generate;
use clap_complete::Shell;

use crate::cli::Cli;

pub fn execute(shell: Shell) -> Result<()> {
    let mut cmd = Cli::command();
    generate(shell, &mut cmd, "makectl", &mut std::io::stdout());
    Ok(())
}
