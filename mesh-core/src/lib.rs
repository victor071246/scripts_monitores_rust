use serde::Serialize;
use serde_json;

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