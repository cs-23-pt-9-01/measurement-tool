use sysinfo::{ProcessExt, System, SystemExt};

fn main() {
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    // We display all disks' information:
    println!("=> disks:");
    for disk in sys.disks() {
        println!("{:?}", disk);
    }

    println!("total memory: {} bytes", sys.total_memory());
    println!("used memory : {} bytes", sys.used_memory());
    println!("total swap  : {} bytes", sys.total_swap());
    println!("used swap   : {} bytes", sys.used_swap());

    // Sleep for 5 seconds.
    std::thread::sleep(std::time::Duration::from_secs(5));

    // Sys refresh only processes' information.
    sys.refresh_processes();

    for (pid, process) in sys.processes() {
        if process.cpu_usage() > 0.0 {
            println!(
                "Pid: [{}], process name: {}, cpu usage: {}, disk usage: {:?}",
                pid,
                process.name(),
                process.cpu_usage(),
                process.disk_usage()
            );
        }
    }
}
