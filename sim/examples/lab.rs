use garden_sim::systems::garden_lab::{CAN_INTERACTIVE, can_tilt_angle, classify, water_reaches};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("hi");
    // Mode: "check <distance_px>" → watering physics
    if args.get(1).map(|s| s.as_str()) == Some("check") {
        let dist: f64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(100.0);
        let tilt        = can_tilt_angle(dist, 180.0);
        let reaches     = water_reaches(dist);
        let interactive = CAN_INTERACTIVE;
        println!("{{\"tilt\":{:.2},\"reaches\":{},\"interactive\":{}}}", tilt, reaches, interactive);
        return;
    }

    // Mode: "config" → static config flags (no distance needed)
    if args.get(1).map(|s| s.as_str()) == Some("config") {
        println!("{{\"interactive\":{}}}", CAN_INTERACTIVE);
        return;
    }

    // Default mode: "m f t" → flower classification
    let moisture:    f64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0.5);
    let fertilizer:  f64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(1.0);
    let temperature: f64 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(22.0);

    match classify(moisture, fertilizer, temperature) {
        Some(f) => println!(
            "{{\"name\":\"{}\",\"color\":\"{}\",\"tip\":\"{}\",\"petals\":{},\"size\":{},\"center\":\"{}\",\"leaf_size\":{},\"leaf_color\":\"{}\",\"stem_color\":\"{}\",\"leaf_pos\":{},\"leaf_above\":{}}}",
            f.name, f.color, f.tip, f.petals, f.size, f.center, f.leaf_size, f.leaf_color, f.stem_color, f.leaf_pos, f.leaf_above
        ),
        None => println!("{{\"color\":null}}"),
    }
}
