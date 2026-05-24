mod sysinfo;
mod utils;
mod cliArgPars;

use clap::Parser;
use cliArgPars::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    println!("Interval: {}s", cli.interval);

    match cli.command {
        None                            => println!("Kein Subcommand → Dashboard"),
        Some(Commands::Disk)            => println!("Disk-Ansicht"),
        Some(Commands::Procs { limit }) => println!("Top {} Prozesse", limit),
    }
}

