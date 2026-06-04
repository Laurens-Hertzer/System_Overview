use std::collections::VecDeque;

pub fn bytes_to_terabytes(bytes: u64) -> f64 {
    bytes as f64 / 1_099_511_627_776.0
}
pub fn bytes_to_gb(bytes: u64) -> f64 {
    bytes as f64 / 1_073_741_824.0
}

pub fn bytes_to_mb(bytes: u64) -> f64 {
    bytes as f64 / 1_048_576.0
}

pub fn push_value(history: &mut VecDeque<u64>, value: u64) {
    history.push_back(value);
    if history.len() > 10 {
        history.pop_front();
    }
}
pub fn logo () {
    println!(r#" ____            _
/ ___| _   _ ___| |_ ___ _ __ ___
\___ \| | | / __| __/ _ \ '_ ` _ \
 ___) | |_| \__ \ ||  __/ | | | | |
|____/ \__, |___/\__\___|_| |_| |_|
 / _ \_|___/____ _ ____   _(_) _____      __
| | | \ \ / / _ \ '__\ \ / / |/ _ \ \ /\ / /
| |_| |\ V /  __/ |   \ V /| |  __/\ V  V /
 \___/  \_/ \___|_|    \_/ |_|\___| \_/\_/  "#);
}
