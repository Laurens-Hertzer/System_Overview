mod sysinfo;
mod utils;
mod cliArgPars;
mod tui;

use clap::Parser;
use cliArgPars::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    println!("Interval: {}s", cli.interval);

    /*match cli.command {
        None                            => println!("Kein Subcommand → Dashboard"),
        Some(Commands::Disk)            => println!("Disk-Ansicht"),
        Some(Commands::Procs { limit }) => println!("Top {} Prozesse", limit),

    }*/
    use dialoguer::Select;

    let optionen = vec!["Dashboard", "Disk", "Prozesse", "Beenden"];

    let auswahl = Select::new()
        .with_prompt("Was möchtest du anzeigen?")
        .items(&optionen)
        .default(0)           // vorausgewählter Index
        .interact()
        .unwrap();

    match auswahl {
        0 => tui::tui().expect("TUI failed"),
        1 => println!("Disk"),
        2 => println!("Prozesse"),
        3 => println!("Beenden"),
        _ => {}
    }
}


