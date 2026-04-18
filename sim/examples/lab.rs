use garden_sim::systems::garden_lab::classify;

fn main() {
    let args: Vec<String> = std::env::args().collect();
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
