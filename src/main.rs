use std::env::consts::{ARCH, OS};
use sysinfo::{System, SystemExt};
fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("total memory: {} bytes", sys.total_memory());
    println!("used memory : {} bytes", sys.used_memory());
    println!("total swap  : {} bytes", sys.total_swap());
    println!("used swap   : {} bytes", sys.used_swap());

    println!(
        "System name:             {}",
        sys.name().unwrap_or("NONE".into())
    );
    println!(
        "System kernel version:   {}",
        sys.kernel_version().unwrap_or("NONE".into())
    );
    println!("CPU Architecture: {}", ARCH);
    println!("System OS version:       {}", OS);
    println!(
        "System host name:        {}",
        sys.host_name().unwrap_or("NONE".into())
    );

    println!("NB CPUs: {}", sys.cpus().len());
}
