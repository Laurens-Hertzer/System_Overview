mod backend;
mod cliArgPars;
mod sysinfo;
mod tui;
mod utils;

use clap::Parser;
use cliArgPars::{Cli, Commands};
use std::sync::mpsc;
use std::thread;

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
        .default(0) // vorausgewählter Index
        .interact()
        .unwrap();

    match auswahl {
        0 => {
            let mut terminal = ratatui::init();
            let mut app = tui::App::new();

            let (event_tx, event_rx) = mpsc::channel::<backend::Event>();

            let tx_to_input_events = event_tx.clone();

            thread::spawn(move || {
                backend::handle_input_events(tx_to_input_events);
            });

            let tx_cpu = event_tx.clone();

            thread::spawn(move || {
                backend::cpu_background_thread(tx_cpu);
            });

            let tx_to_background_progress_events = event_tx.clone();
            thread::spawn(move || backend::run_background_thread(tx_to_background_progress_events));

            let app_result = app.run(&mut terminal, event_rx);
            ratatui::restore();
            app_result.expect("TUI failed");
        }
        1 => println!("Disk"),
        2 => println!("Prozesse"),
        3 => println!("Beenden"),
        _ => {}
    }
}
