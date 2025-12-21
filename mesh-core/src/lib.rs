use serde::Serialize;
use serde_json;

pub fn percent_to_bars(number_float: &f64) -> String {
    let number_rounded = (number_float / 10.0).round() * 10.0;
    let number_int: i32 = number_rounded as i32;
    let max_tiles: i32 = 10;
    let tiles: i32 = number_int / max_tiles;
    let mesh = max_tiles - tiles;

    let filled_tile = "█".repeat(tiles as usize);         
    let mesh_tile = "░".repeat(mesh as usize);
    format!("{filled_tile}{mesh_tile}")
}

#[derive(Serialize)]
pub struct WaybarPayload {
    pub text: String,
    pub description: Option<String>,
    pub graphics: Option<Vec<String>>,
}

pub fn print_waybar(payload: &WaybarPayload) {
    let json = serde_json::to_string(payload).unwrap();
    println!("{json}");
}

