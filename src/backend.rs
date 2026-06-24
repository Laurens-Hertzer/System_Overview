use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use sysinfo::{System, CpuRefreshKind, RefreshKind};
use crate::utils::bytes_to_gb;

pub enum Event {
    Input(crossterm::event::KeyEvent),
    CpuProgress(f64),
    RamProgress(f64),
}

pub fn handle_input_events(tx: mpsc::Sender<Event>) {
    loop {
        match crossterm::event::read().unwrap() {
            crossterm::event::Event::Key(key_event) => tx.send(Event::Input(key_event)).unwrap(),
            _ => {}
        }
    }
}

pub fn cpu_background_thread(tx: mpsc::Sender<Event>) { ;
    let mut sys = System::new_all();

    loop {
        sys.refresh_cpu_all();
        let cpu_usage = sys.global_cpu_usage();
        let cpu_ratio = (cpu_usage as f64) / 100.0;
        if tx.send(Event::CpuProgress(cpu_ratio)).is_err() {
            break;
        }
        thread::sleep(Duration::from_millis(1000));
    }

}

pub fn ram_background_thread(tx: mpsc::Sender<Event>) { ;
let mut sys = System::new_all();

    loop {
        sys.refresh_memory();
        let ram_bytes = sys.used_memory();
        let ram_gb = bytes_to_gb(ram_bytes);
        if tx.send(Event::RamProgress(ram_gb)).is_err() {
            break;
        }
        thread::sleep(Duration::from_millis(1000));
    }
}


