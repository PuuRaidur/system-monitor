use sysinfo::{System, SystemExt, DiskExt, CpuExt};
use colored::Colorize;

pub fn get_os_name(sys: &System) -> String {
    sys.name().unwrap_or_else(|| "Unknown OS".to_string())
}

pub fn get_os_version(sys: &System) -> String {
    sys.long_os_version().unwrap_or_else(|| "Unknown OS version".to_string())
}

pub fn get_kernel_version(sys: &System) -> String {
    sys.kernel_version().unwrap_or_else(|| "Unknown kernel version".to_string())
}

pub fn get_hostname(sys: &System) -> String {
    sys.host_name().unwrap_or_else(|| "Unknown host".to_string())
}

pub fn print_disk_info(sys: &System) {
    for disk in sys.disks() {
        let used_space = format!("{} GB", (disk.total_space() - disk.available_space()) / 1024 / 1024 / 1024).yellow();
        let total_space = format!("{} GB", disk.total_space() / 1024 / 1024 / 1024).red();
        println!("Disk {:?} mounted on {:?} - used {} / {}", disk.name(),
                                                                disk.mount_point(),
                                                                used_space,
                                                                total_space);
    }
}

pub fn get_used_memory(sys: &System) -> String {
    format!("{} GB", (sys.used_memory() as f32 / 1024.0 / 1024.0 / 1024.0).round())
}

pub fn get_total_memory(sys: &System) -> String {
    format!("{} GB", (sys.total_memory() as f32 / 1024.0 / 1024.0 / 1024.0).round())
}

pub fn get_cpu_brand(sys: &System) -> String {
    sys.global_cpu_info().brand().to_string()
}

pub fn get_cpu_usage(sys: &System) -> String {
    format!("{}%",sys.global_cpu_info().cpu_usage().round())
}

pub fn get_cpu_frequency(sys: &System) -> u64 {
    sys.global_cpu_info().frequency()
}

pub fn print_cpu_cores_info(sys: &System) {
    for (i, core) in sys.cpus().iter().enumerate() {
        println!("Core {} - {:.2} MHz - {} usage", format!("{}", i).yellow(), core.frequency(),
        format!("{}%", core.cpu_usage().round()).yellow());
    }
}