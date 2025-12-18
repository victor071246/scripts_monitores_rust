use std::fmt::format;

use mesh_core::{WaybarPayload, print_waybar};

fn read_cpu_temp_c() -> std::io::Result<f32> {
    let raw_temp = std::fs::read_to_string("/sys/class/hwmon/hwmon1/temp1_input")?;
    let processed_temp: f32 = raw_temp.trim().parse().unwrap_or(0.0);
    Ok(processed_temp / 1000.0_f32)
}

fn main(){

    let temperature = read_cpu_temp_c().unwrap_or(0.0_f32);

    let string: String = format!("{temperature}ºC 12.0V");


    let payload = WaybarPayload {
        text: string,
        description: Some("Usando 100% voltagem".into()),
        graphics: None
    };

    print_waybar(&payload);
}


/*
for d in /sys/class/hwmon/hwmon*; do
    echo "== $d =="
    cat "$d/name"
done 
*/