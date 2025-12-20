use std::time::Duration;
use mesh_core::{WaybarPayload, print_waybar};

fn read_cpu_clock_threads() -> Vec<f32> {
    const MAX_CLOCK: f32 = 4.268572;
    const MIN_CLOCK: f32 = 2.391429;
    let mut clocks_usage_percent_from_threads = Vec::new();

    for cpu_id in 0..12 {

        //Leitura
        let path = format!("/sys/devices/system/cpu/cpu{cpu_id}/cpufreq/scaling_cur_freq");

        // Processamento
        let raw_freq = std::fs::read_to_string(&path).unwrap_or_else(|_error| panic!("failed to read thread {cpu_id} frequency"));
        let khz_freq: f32 = raw_freq.trim().parse().unwrap_or_else(|_error| panic!("failed to parse thread {cpu_id} frequency"));
        let mhz_freq = khz_freq / 1000.0;
        let percent = ((mhz_freq - MIN_CLOCK) / (MAX_CLOCK - MIN_CLOCK)) * 100.0;

        //Adicionando no array
        clocks_usage_percent_from_threads.push(percent);
    }

    clocks_usage_percent_from_threads
}

fn main(){}