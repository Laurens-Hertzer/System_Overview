use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use sysinfo::{System, CpuRefreshKind, RefreshKind};
use nvml_wrapper::Nvml;
use nvml_wrapper::struct_wrappers::device::{MemoryInfo, Utilization};
use crate::utils::{bytes_to_gb, logo_rata, logo_print};
use nvml_wrapper::enum_wrappers::device::Brand;

pub enum Event {
    Input(crossterm::event::KeyEvent),
    CpuProgress(f64),
    RamProgress(f64),
    GpuProgress {
        utilization: f64,
        brand: Brand,
        fan_speed: u32,
        power_limit: u32,
        memory_info: MemoryInfo,
    },
    GpuAvailable,
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

pub fn gpu_background_thread(tx: mpsc::Sender<Event>) {
    let nvml = match Nvml::init() {
        Ok(instance) => instance,
        Err(_) => {
            tx.send(Event::GpuAvailable).ok();
            return;
        }
    };

    let device = match nvml.device_by_index(0) {
        Ok(dev) => dev,
        Err(_) => {
            tx.send(Event::GpuAvailable).ok();
            return;
        }
    };

    loop {
        let rates = match device.utilization_rates() {
            Ok(r) => r,
            Err(e) => { eprintln!("utilization_rates: {e}"); thread::sleep(Duration::from_millis(1000)); continue; }
        };

        let brand = match device.brand() {
            Ok(b) => b,
            Err(e) => { eprintln!("brand: {e}"); thread::sleep(Duration::from_millis(1000)); continue; }
        };

        let fan_speed = match device.fan_speed(0) {
            Ok(f) => f,
            Err(e) => { 0 }
        };

        let power_limit = match device.enforced_power_limit() {
            Ok(p) => p,
            Err(e) => { eprintln!("power_limit: {e}"); thread::sleep(Duration::from_millis(1000)); continue; }
        };

        let memory_info = match device.memory_info() {
            Ok(m) => m,
            Err(e) => { eprintln!("memory_info: {e}"); thread::sleep(Duration::from_millis(1000)); continue; }
        };

        if tx.send(Event::GpuProgress {
            utilization: rates.gpu as f64,
            brand,
            fan_speed,
            power_limit,
            memory_info,
        }).is_err() { break; }

        thread::sleep(Duration::from_millis(1000));
    }
}

