use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use sysinfo::{System, CpuRefreshKind, RefreshKind};


pub enum Event {
    Input(crossterm::event::KeyEvent),
    Progress(f64),
    CpuProgress(f64),
}

pub fn handle_input_events(tx: mpsc::Sender<Event>) {
    loop {
        match crossterm::event::read().unwrap() {
            crossterm::event::Event::Key(key_event) => tx.send(Event::Input(key_event)).unwrap(),
            _ => {}
        }
    }
}

pub fn run_background_thread(tx: mpsc::Sender<Event>) {
    let mut progress = 0_f64;
    let increment = 0.01_f64;
    loop {
        thread::sleep(Duration::from_millis(100));
        progress += increment;
        progress = progress.min(1_f64);
        tx.send(Event::Progress(progress)).unwrap();
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


