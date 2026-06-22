use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use sysinfo::{System, CpuRefreshKind, RefreshKind};


pub enum Event {
    Input(crossterm::event::KeyEvent),
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


