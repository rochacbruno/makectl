mod config;

use structopt::StructOpt;

use crate::config::{Config, Command};

fn main() {
    let conf = Config::from_args();

    match conf.command {
        Command::Add(ref add_params) => {
            println!("Add command invoked with: {:?}", add_params.templates);
        }
    }
}
