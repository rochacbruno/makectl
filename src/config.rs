use structopt::StructOpt;

/// Generate and Manage targets in your makefiles.
///
/// Makectl is a command line tool to generate and manage general use targets in your makefiles.
#[derive(StructOpt)]
#[structopt(name = "makectl")]
pub struct Config {
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(StructOpt)]
#[structopt()]
pub enum Command {
    /// Add the provided templates into the Makefile.
    Add(AddParams)
}

#[derive(StructOpt)]
#[structopt(about = "makectl add command parameters")]
pub struct AddParams {
    /// List of templates to apply into the Makefile.
    #[structopt(name = "templates", long = "template", short = "t", required = true)]
    pub templates: Vec<String>,
}
