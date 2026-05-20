pub fn bytes_to_terabytes(bytes: u64) -> f64 {
    bytes as f64 / 1_099_511_627_776.0
}
pub fn bytes_to_gb(bytes: u64) -> f64 {
    bytes as f64 / 1_073_741_824.0
}

pub fn bytes_to_mb(bytes: u64) -> f64 {
    bytes as f64 / 1_048_576.0
}