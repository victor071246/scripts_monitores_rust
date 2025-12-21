use mesh_core::{WaybarPayload, print_waybar};
use tokio::time::{sleep, Duration};

struct Thread {
    level: u16,
    relative_clock_percentage: f32,
}

fn read_cpu_clock_threads() -> Vec<f32> {
    const MAX_CLOCK: f32 = 4268.572;
    const MIN_CLOCK: f32 = 2391.429;
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

fn print_thread_icons_line(clock_threads_vector: &Vec<f32>) -> Vec<String> {

    let mut  thread_vector: Vec<Thread> = Vec::new();

    for (indice, thread_clock) in clock_threads_vector.iter().enumerate() {

        let mut level: u16 = 0;


        if *thread_clock >= 70.0 {
            level = 2;       
        }
        else if *thread_clock > 40.0 {
            level = 1;
        }
        

        let thread = Thread{
            level,
            relative_clock_percentage: *thread_clock as f32,
        };

        thread_vector.push(thread);
    }

    let mut icons_vector:Vec<String> = Vec::new();

    for thread in thread_vector {
        let icon: char = match thread.level{
            0 => '□',
            1 => '▣',
            2 => '■',
            _ => '□',
        };

        let icon_string: String = icon.to_string();

        icons_vector.push(icon_string);
    }

    icons_vector

}

#[tokio::main]
async fn main(){
    
    loop {
    // Uso de clock
    let clock_usage_percent_from_threads = read_cpu_clock_threads();
    let line_to_print = print_thread_icons_line(&clock_usage_percent_from_threads);
    let mut formatted_string = String::new();

    for (indice, char) in line_to_print.iter().enumerate(){
        if indice > 0 {
            formatted_string.push(' ');
        }
        else if indice == 3{
            formatted_string.push('\n');
        }

        formatted_string.push_str(char);

    }

    println!("{formatted_string}");
    let payload: WaybarPayload = WaybarPayload { text: (formatted_string), description: (None), graphics: (None) };
    print_waybar(&payload);
    sleep(Duration::from_millis(200)).await;
    }

}