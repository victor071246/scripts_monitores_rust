use std::{thread, time::Duration};
use mesh_core::{WaybarPayload, print_waybar, percent_to_bars};


struct CpuSnapshot {
    total: u64,
    idle: u64,
}

fn create_cpu_snapshot() -> CpuSnapshot {
    let process_lines = std::fs::read_to_string("/proc/stat").expect("failed to read /proc/stat");
    let first_line = process_lines.lines().find(|line| line.starts_with("cpu ")).expect("no global cpu line in /proc/stat");
    let mut line_parts = first_line.split_whitespace();

    //pula a linha da cpu
    line_parts.next();

    // processamento das partes
    let user: u64 = line_parts.next().unwrap().parse().unwrap();
    let nice: u64 = line_parts.next().unwrap().parse().unwrap();
    let system: u64 = line_parts.next().unwrap().parse().unwrap();
    let idle: u64 = line_parts.next().unwrap().parse().unwrap();
    let iowait: u64 = line_parts.next().unwrap().parse().unwrap();
    let irq: u64  = line_parts.next().unwrap().parse().unwrap();
    let softirq: u64 = line_parts.next().unwrap().parse().unwrap();
    let steal: u64 = line_parts.next().unwrap().parse().unwrap_or(0);

    let idle_lines_count: u64 = idle + iowait;
    let total_lines: u64 = user + nice + system + idle + iowait + irq + softirq + steal;
    
    CpuSnapshot {total: total_lines, idle: idle_lines_count}
}

fn diff_between_snapshots(snapshot1: CpuSnapshot, snapshot2: CpuSnapshot) -> CpuSnapshot {
    let diff_idle_process: u64 = snapshot2.idle - snapshot1.idle;
    let diff_total_process: u64 = snapshot2.total - snapshot1.total;

    CpuSnapshot { total: (diff_total_process), idle: (diff_idle_process) }
}

fn take_snapshots_and_return_value() -> f32{

    let snapshot1: CpuSnapshot = create_cpu_snapshot();
    thread::sleep(Duration::from_millis(200));
    let snapshot2: CpuSnapshot = create_cpu_snapshot();

    let diff_snapshot: CpuSnapshot = diff_between_snapshots(snapshot1, snapshot2);
    
    let total_process_count: f32 = diff_snapshot.total as f32;
    let idle_process_count: f32 = diff_snapshot.idle as f32;

    let busy_process_count: f32 = total_process_count - idle_process_count;
    let percent: f32 = (busy_process_count / total_process_count) * 100.0;


    println!("{busy_process_count} {total_process_count} {percent:.2}");
    
    percent
}

fn main() {

    loop {  

    let cpu_in_use_percentage: f32 = take_snapshots_and_return_value();
    let bars_string: String = percent_to_bars(&cpu_in_use_percentage);
    let formatted_cpu_usage= format!{"{cpu_in_use_percentage:05.2}"}.replace(".", ",");
    let formatted_string  = format!{"{formatted_cpu_usage}% {bars_string}"};
    let payload: WaybarPayload = WaybarPayload { text: (formatted_string), description: (None), graphics: (None) };
    print_waybar(&payload); 
  
    }
  
}

                     