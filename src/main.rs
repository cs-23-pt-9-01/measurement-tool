use serde::Serialize;
use std::{fs::OpenOptions, io::Write};
use sysinfo::SystemExt;
use sysinfo::{CpuExt, DiskUsage, PidExt, ProcessExt, System};
use systemctl::UnitList;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

#[derive(Debug, Serialize, PartialEq)]
struct MeasurementData {
    timestamp: String,
    used_memory: u64,
    used_swap: u64,
    process_data: Option<Vec<ProcessData>>,
    cpu_data: Option<Vec<CpuData>>,
    units: Option<Vec<UnitList>>,
}

#[derive(Debug, Serialize, PartialEq, Clone)]
struct CpuData {
    cpu_usage: f32,
    frequency: u64,
}

#[derive(Debug, Serialize, PartialEq, Clone)]
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

    let mut bak_data = MeasurementData {
        timestamp: OffsetDateTime::now_utc().format(&Rfc3339).unwrap(),
        used_memory: 0,
        used_swap: 0,
        process_data: None,
        cpu_data: None,
        units: None,
    };

    loop {
        let curr_time = OffsetDateTime::now_utc().format(&Rfc3339).unwrap();
        let mut output_data = MeasurementData {
            timestamp: bak_data.timestamp.clone(),
            used_memory: sys.used_memory(),
            used_swap: sys.used_swap(),
            process_data: None,
            cpu_data: None,
            units: None,
        };

        // Get processes
        get_processes(&mut sys, &bak_data, &mut output_data);

        // Get CPU data
        get_cpu_data(&mut sys, &bak_data, &mut output_data);

        // Get systemd units
        #[cfg(target_os = "linux")]
        get_systemd_units(&bak_data, &mut output_data);

        // Write to file
        if output_data != bak_data {
            // Replace timestamp with current time
            output_data.timestamp = curr_time;

            let mut output_data_string = serde_json::to_string(&output_data).unwrap();
            output_data_string.push('\n');

            file.write_all(output_data_string.as_bytes()).unwrap();

            bak_data = output_data;
        }
    }
}

#[cfg(target_os = "linux")]
fn get_systemd_units(bak_data: &MeasurementData, output_data: &mut MeasurementData) {
    let units = systemctl::list_units_full(None, None, None).unwrap();
    if let Some(bak_units_ref) = &bak_data.units {
        if bak_units_ref != &units {
            output_data.units = Some(units);
        }
    } else {
        output_data.units = Some(units);
    }
}

fn get_cpu_data(sys: &mut System, bak_data: &MeasurementData, output_data: &mut MeasurementData) {
    let mut cpu_data = Vec::new();
    sys.refresh_cpu();
    for cpu in sys.cpus() {
        cpu_data.push(CpuData {
            cpu_usage: cpu.cpu_usage(),
            frequency: cpu.frequency(),
        });
    }

    if let Some(bak_cpu_data_ref) = &bak_data.cpu_data {
        if bak_cpu_data_ref != &cpu_data {
            output_data.cpu_data = Some(cpu_data);
        }
    } else {
        output_data.cpu_data = Some(cpu_data);
    }
}

fn get_processes(sys: &mut System, bak_data: &MeasurementData, output_data: &mut MeasurementData) {
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

    if let Some(bak_process_data_ref) = &bak_data.process_data {
        if bak_process_data_ref != &process_data {
            output_data.process_data = Some(process_data);
        }
    } else {
        output_data.process_data = Some(process_data);
    }
}

// TODO: Fix CPU usage being 0.0, not working yet
/*
use std::thread;
thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
*/

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
