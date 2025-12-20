use std::time::Duration;
use mesh_core::{WaybarPayload, print_waybar, percent_to_bars};

fn main() {
    let number: f32 = 30.42;

    let string: String = percent_to_bars(&number);
    let payload = WaybarPayload { text: (string), description: (None), graphics: (None) };
    print_waybar(&payload);
}

