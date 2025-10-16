mod info;
use info::*;
use std::{thread, time::Duration};
use std::io::{self, Write};
use sysinfo::{System, SystemExt};
use crossterm::{execute, terminal, cursor};

fn main() -> io::Result<()> {
    let mut sys = System::new_all();
    let mut stdout = io::stdout();
    loop {
        sys.refresh_all();

        execute!(
            stdout,
            terminal::Clear(terminal::ClearType::All), // clear screen
            cursor::MoveTo(0, 0),                     // move cursor to top-left
        )?;

        println!("System monitor");
        println!("==================================================================");
        println!("OS name: {:?}", get_os_name(&mut sys));
        println!("OS version: {:?}", get_os_version(&mut sys));
        println!("Kernel version: {:?}", get_kernel_version(&mut sys));
        println!("Host: {:?}", get_hostname(&mut sys));
        println!("\nCPU usage: {:.0}%     Memory: ({:.2} GB / {:.2} GB)", get_cpu_usage(&mut sys),
                                                                        get_used_memory(&mut sys),
                                                                        get_total_memory(&mut sys));

        thread::sleep(Duration::from_secs(1));
    }
}
