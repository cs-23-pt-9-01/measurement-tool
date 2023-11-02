use serde::Serialize;
use std::{
    fs::OpenOptions,
    io::Write,
    time::{SystemTime, UNIX_EPOCH},
};
use sysinfo::{CpuExt, DiskUsage, PidExt, ProcessExt, System, SystemExt};
use time::{format_description, OffsetDateTime, UtcOffset};

#[derive(Debug, Serialize)]
struct OutputData {
    timestamp: OffsetDateTime,
    used_memory: u64,
    used_swap: u64,
    process_data: Vec<ProcessData>,
    cpu_data: Vec<CpuData>,
}

#[derive(Debug, Serialize)]
struct CpuData {
    cpu_usage: f32,
    frequency: u64,
}

#[derive(Debug, Serialize)]
struct ProcessData {
    pid: u32,
    name: String,
    cpu_usage: f32,
    memory_usage: u64,
    disk_usage: DiskUsage,
}

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
    std::thread::sleep(std::time::Duration::from_secs(1));
    sys.refresh_processes();

    let mut process_data: Vec<ProcessData> = Vec::new();
    for (pid, process) in sys.processes() {
        if process.cpu_usage() > 0.0 {
            process_data.push(ProcessData {
                pid: pid.as_u32(),
                name: process.name().to_string(),
                cpu_usage: process.cpu_usage(),
                memory_usage: process.memory(),
                disk_usage: process.disk_usage(),
            });
        }
    }

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("idle-log.txt")
        .unwrap();

    let mut cpu_data = Vec::new();
    sys.refresh_cpu();
    for cpu in sys.cpus() {
        cpu_data.push(CpuData {
            cpu_usage: cpu.cpu_usage(),
            frequency: cpu.frequency(),
        });
    }

    let format = format_description::parse(
        "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour \
             sign:mandatory]:[offset_minute]:[offset_second]",
    )
    .unwrap();
    let aweraewr = OffsetDateTime::now_utc().format(&format).unwrap();
    println!("{}", aweraewr);

    let output_data = OutputData {
        timestamp: OffsetDateTime::now_utc(),
        used_memory: sys.used_memory(),
        used_swap: sys.used_swap(),
        process_data,
        cpu_data,
    };

    //let mut testy = serde_json::to_string(&sys).unwrap();
    let mut testy = serde_json::to_string(&output_data).unwrap();
    testy.push('\n');

    file.write_all(testy.as_bytes()).unwrap();

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
