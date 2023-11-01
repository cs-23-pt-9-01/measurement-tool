use sysinfo::{CpuExt, ProcessExt, System, SystemExt};

fn main() {
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    // We display all disks' information:
    println!("=> disks:");
    for disk in sys.disks() {
        println!("{:?}", disk);
    }

    // Components temperature:
    println!("=> components:");
    for component in sys.components() {
        println!("{:?}", component);
    }

    println!("=> system:");
    // RAM and swap information:
    println!("total memory: {} bytes", sys.total_memory());
    println!("used memory : {} bytes", sys.used_memory());
    println!("total swap  : {} bytes", sys.total_swap());
    println!("used swap   : {} bytes", sys.used_swap());

    // Display system information:
    println!("System name:             {:?}", sys.name());
    println!("System kernel version:   {:?}", sys.kernel_version());
    println!("System OS version:       {:?}", sys.os_version());
    println!("System host name:        {:?}", sys.host_name());

    // Number of CPUs:
    println!("NB CPUs: {}", sys.cpus().len());

    // Sleep for 5 seconds, then update system information again:
    std::thread::sleep(std::time::Duration::from_secs(5));
    sys.refresh_processes();

    for (pid, process) in sys.processes() {
        if process.cpu_usage() > 0.0 {
            println!(
                "Pid: [{}], process name: {}, cpu usage: {}, memory usage: {}, disk usage: {:?}",
                pid,
                process.name(),
                process.cpu_usage(),
                process.memory(),
                process.disk_usage()
            );
        }
    }

    loop {
        sys.refresh_cpu(); // Refreshing CPU information.
        for cpu in sys.cpus() {
            println!("{}% ", cpu.cpu_usage());
        }
        println!();
        // Sleeping to let time for the system to run for long
        // enough to have useful information.
        std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
    }
}
