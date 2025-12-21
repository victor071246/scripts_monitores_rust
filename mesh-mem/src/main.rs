use mesh_core::{WaybarPayload, print_waybar, percent_to_bars};
use tokio::time::{sleep, Duration};

struct MemoryUsageSnapshot {
    total_kb: u64,
    used_kb: u64,
}

fn read_mem_usage() -> MemoryUsageSnapshot {
    let memory_info = std::fs::read_to_string("/proc/meminfo").expect("failed to read /proc/meminfo");

    let mut total_memory :u64 = 0u64;
    let mut available_memory:u64 =  0u64;

    for line in memory_info.lines(){
        if line.starts_with("MemTotal:"){
        let line_parts: Vec<_> = line.split_whitespace().collect();
        total_memory = line_parts[1].parse().unwrap_or(0);

        } else if line.starts_with("MemAvailable:") {
            let line_parts: Vec<_> = line.split_whitespace().collect();
            available_memory = line_parts[1].parse().unwrap_or(0);
        }
    }

    let used_memory = total_memory - available_memory;

    MemoryUsageSnapshot { total_kb: total_memory, used_kb: used_memory }
}

#[tokio::main]
async fn main(){

    loop {
    let memory_snapshot = read_mem_usage();
    let total_memory_kb = memory_snapshot.total_kb as f64;
    let used_memory_kb = memory_snapshot.used_kb as f64;

    let memory_usage_percent = used_memory_kb/total_memory_kb * 100.0;
    let bars_string = percent_to_bars(&memory_usage_percent);

    let percent_string = format!("{memory_usage_percent:05.2}%");

    let formatted_string = format!("{percent_string} {bars_string} ");

    let payload:WaybarPayload = WaybarPayload { text: (formatted_string), description: (None), graphics: (None) };

    print_waybar(&payload);
    sleep(Duration::from_millis(200)).await;
    }

}