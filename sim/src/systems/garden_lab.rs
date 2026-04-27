// ─────────────────────────────────────────────────────────────────────────────
// TASK 3 — Plant Lab
//
// In this task you will teach the simulator which flowers can grow and under
// what conditions. The dashboard's "Plant Lab" tab has three sliders:
//
//   moisture      0.0  to 1.0    (0 = bone-dry   →   1 = waterlogged)
//   fertilizer    1.0  to 3.0    (1 = none        →   3 = very heavy)
//   temperature   0.0  to 40.0   degrees Celsius
//
// `classify` returns a Flower describing how the plant looks, or None if
// nothing grows under those conditions.
//
// ─── Flower fields you can change ────────────────────────────────────────────
//
//   name    text — what to call your flower (shown in the dashboard when it blooms)
//   color   hex string — petal base color
//   tip     hex string — petal tip color (set the same as color for solid petals,
//                        or pick a different color for a two-tone gradient)
//   petals  whole number — how many petals
//   size      decimal — how large the flower head is
//   center    hex string — color of the center dot
//   leaf_size  decimal — how large the leaves are
//   leaf_color hex string — leaf color 
//   stem_color hex string — stem color 
//   leaf_pos   decimal — where on the stem the leaves appear:
//               0.0 = near the soil
//               0.5 = mid-stem (default)
//               1.0 = just below the flower head
//   leaf_above bool — whether the leaves appear in front of or behind the flower:
//               false = leaves behind the flower (default, natural look)
//               true  = leaves in front of the flower (exotic / unusual)
//
// ─── Hex color palette ───────────────────────────────────────────────────────
//
//   ── Reds & Pinks ──────────────────────────────────────────────────────────
//   Hot pink          "#ff66aa"     Rose              "#ff4d6d"
//   Crimson           "#c1121f"     Scarlet           "#ff2400"
//   Blush             "#ffb3c6"     Flamingo          "#fc8eac"
//   Raspberry         "#c0165a"     Punch             "#dd2255"
//   Candy             "#ff6eb4"     Deep rose         "#9b1b30"
//
//   ── Oranges & Yellows ─────────────────────────────────────────────────────
//   Sunshine yellow   "#ffd23f"     Tangerine         "#ff8c00"
//   Amber             "#f4a261"     Peach             "#ffb347"
//   Gold              "#ffc300"     Coral             "#ff7f50"
//   Marigold          "#fca044"     Apricot           "#fbceb1"
//   Saffron           "#f4a100"     Lemon             "#fff44f"
//
//   ── Purples & Pinks ───────────────────────────────────────────────────────
//   Lavender          "#b497ff"     Lilac             "#c8a2c8"
//   Fuchsia           "#e040fb"     Magenta           "#ff00cc"
//   Plum              "#8e4585"     Mauve             "#c18aae"
//   Indigo            "#3a0ca3"     Violet            "#7b2d8b"
//   Orchid            "#da70d6"     Periwinkle        "#ccccff"
//   Wisteria          "#c9a0dc"     Heather           "#9e7bb5"
//
//   ── Blues ─────────────────────────────────────────────────────────────────
//   Sky blue          "#48cae4"     Dusty blue        "#6b9ab8"
//   Electric blue     "#0050ff"     Mint              "#90e0ef"
//   Teal              "#008080"     Turquoise         "#40e0d0"
//   Navy              "#001f5b"     Cornflower        "#6495ed"
//   Powder blue       "#b0e0e6"     Cerulean          "#007ba7"
//   Steel blue        "#4682b4"     Ice blue          "#d0eeff"
//
//   ── Greens ────────────────────────────────────────────────────────────────
//   Forest green      "#2d6a4f"     Sage              "#8faf8f"
//   Chartreuse        "#a8d500"     Olive             "#6b6b00"
//   Lime              "#32cd32"     Moss              "#8a9a5b"
//   Emerald           "#50c878"     Jade              "#00a86b"
//   Fern              "#4f7942"     Seafoam           "#93e9be"
//   Pistachio         "#93c572"     Hunter green      "#355e3b"
//
//   ── Neutrals & Earth tones ────────────────────────────────────────────────
//   Cream             "#fffdd0"     Salmon            "#fa8072"
//   Burgundy          "#800020"     Terracotta        "#cc4e2a"
//   Sand              "#c2b280"     Clay              "#b66c55"
//   Rust              "#b7410e"     Mahogany          "#c04000"
//   Taupe             "#8b8589"     Slate             "#708090"
//   Charcoal          "#36454f"     Pearl             "#eae0c8"
//
//
// ─── YOUR TASK ───────────────────────────────────────────────────────────────
//
//   Define AT LEAST TWO new plant variants below the Sunflower example.
//   Each one must use a DIFFERENT combination of conditions and appearance.
//
//   To test: save this file → adjust the sliders → click Run.
//
//   Ideas:
//     - Desert bloom : dry (0.0–0.2), hot (32–40), low fertilizer
//                      → coral, tip: salmon, 4 petals, size 0.8, orange center
//     - Frost rose   : cold (0–10), moderate moisture, high fertilizer
//                      → crimson, tip: burgundy, 5 petals, size 1.2, dark center
//
// ─────────────────────────────────────────────────────────────────────────────

pub struct Flower {
    pub name:        &'static str,  // display name shown in the dashboard
    pub color:       &'static str,  // petal base color (hex)
    pub tip:         &'static str,  // petal tip color  (hex, same as color = solid)
    pub petals:      u8,            // number of petals (3–12)
    pub size:        f64,           // flower scale (0.6 = small, 1.0 = normal, 1.8 = giant)
    pub center:      &'static str,  // center dot color (hex)
    pub leaf_size:   f64,           // leaf scale (0.5 = tiny, 1.0 = normal, 2.0 = large)
    pub leaf_color:  &'static str,  // leaf color (hex)
    pub stem_color:  &'static str,  // stem color (hex)
    pub leaf_pos:    f64,           // where on the stem leaves appear: 0.0 = near soil, 1.0 = near flower
    pub leaf_above:  bool,          // true = leaves render in FRONT of flower, false = behind
}

// ─── Watering mechanics ──────────────────────────────────────────────────────

pub const CAN_INTERACTIVE: bool = true;

// can_tilt_angle:
//   Returns how many degrees the can should tip when poured.
//
// water_reaches:
//   Returns true when the can is close enough that water reaches the seed.

pub fn can_tilt_angle(distance_px: f64, max_dist: f64) -> f64 {
    (0.2 - (distance_px / max_dist).clamp(0.0, 1.0)) * 50.0
}

pub fn water_reaches(distance_px: f64) -> bool {
    distance_px < 120.0
}

// ─────────────────────────────────────────────────────────────────────────────

pub fn classify(moisture: f64, fertilizer: f64, temperature: f64) -> Option<Flower> {
    // EXAMPLE — Sunflower (already filled in; leave it or change it as you like)
    if moisture >= 0.5 && moisture <= 0.8
        && fertilizer >= 1.5
        && temperature >= 22.0 && temperature <= 30.0
    {
        return Some(Flower {
            name:       "Sunflower",
            color:      "#ffd23f",
            tip:        "#ffaa00",
            petals:     5,
            size:       2.0,
            center:     "#8b5a2b",
            leaf_size:  1.5,
            leaf_color: "#4a9b5f",
            stem_color: "#3d7a3a",
            leaf_pos:   0.35,
            leaf_above: false,
        });
    }
    else if moisture < 0.5 && moisture >= 0.4 && temperature >= 22.0 && temperature <= 30.0
    {
        return Some(Flower {
            name:       "Red flower",
            color:      "#c1121f",
            tip:        "#c04000",
            petals:     8,
            size:       1.3,
            center:     "#f4a100",
            leaf_size:  1.5,
            leaf_color: "#4a9b5f",
            stem_color: "#3d7a3a",
            leaf_pos:   0.35,
            leaf_above: false,
        });
    }

    else if moisture < 0.4 &&  temperature <= 11.0 && fertilizer < 1.2
    {   
        return Some(Flower{
            name:       "cornflower",
            color:      "#6495ed",
            tip:        "#6495ed",
            petals:     16,
            size:       0.7,
            center:     "#fff44f",
            leaf_size:  1.5,
            leaf_color: "#93c572",
            stem_color: "#355e3b",
            leaf_pos:   0.35,
            leaf_above: true,
    
        });
    }
    // TODO ─────────────────────────────────────────────────────────────────
    // Come up with a couple new flowers that grow from different growing conditions.

    None // nothing grew — adjust conditions or sliders and try again
}
