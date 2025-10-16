mod info;
use info::*;
use std::{thread, time::Duration};
use std::io::{self, Write};
use sysinfo::{System, SystemExt};
use crossterm::{execute, terminal, cursor};
use colored::Colorize;

fn main() -> io::Result<()> {
    let mut sys = System::new_all();
    let mut stdout = io::stdout();

    loop {
        execute!(
            stdout,
            terminal::Clear(terminal::ClearType::All), // clear screen
            cursor::MoveTo(0, 0), // move cursor to top-left
        )?;

        writeln!(stdout, "{}", "System monitor".yellow())?;
        writeln!(stdout, "press Ctrl+C to quit")?;
        writeln!(stdout, "==================================================================")?;
        sys.refresh_all();
        println!("OS name: {:?}", get_os_name(&mut sys));
        println!("OS version: {:?}", get_os_version(&mut sys));
        println!("Kernel version: {:?}", get_kernel_version(&mut sys));
        println!("Host: {:?}\n", get_hostname(&mut sys));
        print_disk_info(&mut sys);
        println!("\nMemory(RAM): {} / {}", get_used_memory(&mut sys).yellow(), get_total_memory(&mut sys).red());
        println!("{}: {:?} - {:.2} MHz - {} usage\n",  "CPU".yellow(),
                 get_cpu_brand(&mut sys),
                 get_cpu_frequency(&mut sys),
                 get_cpu_usage(&mut sys).yellow());
        println!("CPU cores:");
        print_cpu_cores_info(&mut sys);

        stdout.flush()?;
        thread::sleep(Duration::from_secs(1));
    }
}
