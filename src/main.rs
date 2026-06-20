mod backend;
mod cliArgPars;
mod sysinfo;
mod tui;
mod utils;

use color_eyre::Result;
use clap::Parser;
use cliArgPars::{Cli, Commands};
use std::sync::mpsc;
use std::thread;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let (event_tx, event_rx) = mpsc::channel::<backend::Event>();
    let tx_cpu = event_tx.clone();

    thread::spawn(move || {
        backend::handle_input_events(event_tx);
    });

    thread::spawn(move || {
        backend::cpu_background_thread(tx_cpu);
    });

    let mut app = tui::App::new();

    let app_result = app.run(&mut terminal, event_rx);

    ratatui::restore();

    app_result?;

    Ok(())
}