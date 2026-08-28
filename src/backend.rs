use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use sysinfo::{System, CpuRefreshKind, RefreshKind, Disks, DiskUsage, Disk, ProcessRefreshKind, ProcessesToUpdate};
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
    GpuNotAvailable (bool),
    DiskProgress {
        disk_name: String,
        total_disk_space: u64,
        available_disk_space: u64,
        used_disk_space: u64,
        read_bytes_per_sec: u64,
        write_bytes_per_sec: u64,
    },
    Error (String),
}

pub fn handle_input_events(tx: mpsc::Sender<Event>) {
    loop {
        match crossterm::event::read().unwrap() {
            crossterm::event::Event::Key(key_event) => tx.send(Event::Input(key_event)).unwrap(),
            _ => {}
        }
    }
}

pub fn cpu_background_thread(tx: mpsc::Sender<Event>) {

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

pub fn ram_background_thread(tx: mpsc::Sender<Event>) {

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
        Err(e) => {
            let _ = tx.send(Event::GpuNotAvailable(true));
            let _ = tx.send(Event::Error(format!("NVML Init Fehler: {e}")));
            return;
        }
    };

    let device = match nvml.device_by_index(0) {
        Ok(dev) => dev,
        Err(e) => {
            let _ = tx.send(Event::GpuNotAvailable(true));
            let _ = tx.send(Event::Error(format!("GPU Device 0 nicht gefunden: {e}")));
            return;
        }
    };

    loop{
        let utilization = match device.utilization_rates() {
            Ok(rates) => rates.gpu as f64,
            Err(e) => {
                let _ = tx.send(Event::Error(format!("GPU Auslastung Fehler: {e}")));
                0.0 // Fallback Value
            }
        };

        let brand = match device.brand() {
            Ok(brand) => brand,
            Err(e) => {
                let _ = tx.send(Event::Error(format!("GPU Marken Fehler: {e}")));
                Brand::Unknown // Fallback Value
            }
        };

        let fan_speed = match device.fan_speed(0) {
            Ok(speed) => speed,
            Err(e) => {
                let _ = tx.send(Event::Error(format!("GPU Lüftergeschwindigkeit Fehler: {e}")));
                0
            }
        };

        let power_limit = match device.enforced_power_limit() {
            Ok(limit) => limit,
            Err(e) => {
                let _ = tx.send(Event::Error(format!("GPU Power Limit Fehler: {e}")));
                0
            }
        };

        match device.memory_info() {
            Ok(memory_info) => {
                if tx.send(Event::GpuProgress {
                    utilization,
                    brand,
                    fan_speed,
                    power_limit,
                    memory_info,
                }).is_err() {
                    break;
                }
            }
            Err(e) => {
                let _ = tx.send(Event::Error(format!("GPU Memory Info Fehler: {e}")));
                }
        }

        thread::sleep(Duration::from_millis(1000));
    }
}

pub fn disk_background_thread(tx: mpsc::Sender<Event>) {
    let mut disks = Disks::new_with_refreshed_list();
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing().with_processes(ProcessRefreshKind::nothing().with_disk_usage())
    );

    let mut prev_read_bytes: u64 = 0;
    let mut prev_write_bytes: u64 = 0;

    loop {
        disks.refresh(true);
        sys.refresh_processes(ProcessesToUpdate::All, true);

        let (disk_name, total_disk_space, available_disk_space, used_disk_space) = disks
            .list()
            .first()
            .map(|disk| {
                let total = disk.total_space();
                let available = disk.available_space();
                let used = total.saturating_sub(available);
                (
                    disk.name().to_string_lossy().to_string(),
                    total,
                    available,
                    used,
                )
            })
            .unwrap_or_default();

        let mut total_read_bytes: u64 = 0;
        let mut total_write_bytes: u64 = 0;
        for process in sys.processes().values() {
            let usage = process.disk_usage();
            total_read_bytes += usage.read_bytes;
            total_write_bytes += usage.written_bytes;
        }

        let read_bytes_per_sec = total_read_bytes.saturating_sub(prev_read_bytes);
        let write_bytes_per_sec = total_write_bytes.saturating_sub(prev_write_bytes);
        prev_read_bytes = total_read_bytes;
        prev_write_bytes = total_write_bytes;

        if tx.send(Event::DiskProgress {
            read_bytes_per_sec,
            write_bytes_per_sec,
            disk_name,
            total_disk_space,
            available_disk_space,
            used_disk_space,
        }).is_err() {
            return;
        }

        thread::sleep(Duration::from_millis(1000));
    }
}