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
    let mut terminal = ratatui::init();
    let mut app = tui::App::new();

    let (event_tx, event_rx) = mpsc::channel::<backend::Event>();

    let tx_cpu = event_tx.clone();

    thread::spawn(move || {
        backend::cpu_background_thread(tx_cpu);
    });

    let app_result = app.run(&mut terminal, event_rx);
    ratatui::restore();
    app_result.expect("TUI failed");
}
