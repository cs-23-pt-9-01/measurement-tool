use serde::Serialize;
use std::{fs::OpenOptions, io::Write};
use sysinfo::SystemExt;
use sysinfo::{CpuExt, DiskUsage, PidExt, ProcessExt, System};
use systemctl::UnitList;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

#[derive(Debug, Serialize)]
struct MeasurementData {
    timestamp: String,
    used_memory: u64,
    used_swap: u64,
    process_data: Vec<ProcessData>,
    cpu_data: Vec<CpuData>,
    units: Vec<UnitList>,
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
    sys.refresh_all();

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("idle-log.txt")
        .unwrap();

    loop {
        // Refresh processes
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

        // Get CPU data
        let mut cpu_data = Vec::new();
        sys.refresh_cpu();
        for cpu in sys.cpus() {
            cpu_data.push(CpuData {
                cpu_usage: cpu.cpu_usage(),
                frequency: cpu.frequency(),
            });
        }

        #[cfg(target_os = "linux")]
        let output_data = {
            //let enabled_services = systemctl::list_enabled_services().unwrap();
            let units = systemctl::list_units_full(None, None, None).unwrap();

            MeasurementData {
                timestamp: OffsetDateTime::now_utc().format(&Rfc3339).unwrap(),
                used_memory: sys.used_memory(),
                used_swap: sys.used_swap(),
                process_data,
                cpu_data,
                units,
            }
        };

        #[cfg(target_os = "windows")]
        let output_data = MeasurementData {
            timestamp: OffsetDateTime::now_utc().format(&Rfc3339).unwrap(),
            used_memory: sys.used_memory(),
            used_swap: sys.used_swap(),
            process_data,
            cpu_data,
            units: Vec::new(),
        };

        //let mut testy = serde_json::to_string(&sys).unwrap();
        let mut output_data_string = serde_json::to_string(&output_data).unwrap();
        output_data_string.push('\n');

        file.write_all(output_data_string.as_bytes()).unwrap();

        // TODO: Fix CPU usage being 0.0, not working yet
        /*
        use std::thread;
        thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
        */
    }
}

/*
loop {
    sys.refresh_cpu(); // Refreshing CPU information.
    for cpu in sys.cpus() {
        println!("{}% ", cpu.cpu_usage());
    }
    println!();
    // Sleeping to let time for the system to run for long
    // enough to have useful information.
    thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
}
*/

// We display all disks' information:
/*
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
*/

// Sleep for 5 seconds, then update system information again:
