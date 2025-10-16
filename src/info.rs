use sysinfo::{CpuExt, System, SystemExt};

pub fn get_os_name(sys: &System) -> String {
    sys.name().unwrap_or_else(|| "Unknown OS".to_string())
}

pub fn get_os_version(sys: &System) -> String {
    sys.os_version().unwrap_or_else(|| "Unknown OS version".to_string())
}

pub fn get_kernel_version(sys: &System) -> String {
    sys.kernel_version().unwrap_or_else(|| "Unknown kernel version".to_string())
}

pub fn get_hostname(sys: &System) -> String {
    sys.host_name().unwrap_or_else(|| "Unknown host".to_string())
}

pub fn get_total_memory(sys: &System) -> f64 {
    sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0
}

pub fn get_used_memory(sys: &mut System) -> f64 {
    sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0
}

pub fn get_cpu_usage(sys: &mut System) -> i8 {
    sys.global_cpu_info().cpu_usage() as i8
}