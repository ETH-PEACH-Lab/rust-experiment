use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
pub struct Feature {
    pub name:   String,
    pub level:  f64,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct Tree {
    pub health:    f64,
    pub feature_a: Feature,
    pub feature_b: Feature,
    pub feature_c: Feature,
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            health: 1.0,
            feature_a: Feature {
                name: "Leaves".to_string(),
                level: 0.3,
                active: false,
            },
            feature_b: Feature {
                name: "Flowers".to_string(),
                level: 0.0,
                active: false,
            },
            feature_c: Feature {
                name: "Strength".to_string(),
                level: 0.5,
                active: false,
            },
        }
    }

    pub fn tick(&mut self, dt: f64, moisture: f64, light_ok: bool, temperature: f64) {
        // Feature A: Leaves respond to moisture
        let leaves_growth = moisture * 0.3 * dt;
        self.feature_a.level = (self.feature_a.level + leaves_growth).clamp(0.0, 1.0);
        self.feature_a.active = moisture > 0.5;

        // Feature B: Flowers respond to light and temperature
        let temp_factor = if temperature > 15.0 && temperature < 30.0 {
            1.0 - ((temperature - 22.5).abs() / 7.5).min(1.0)
        } else {
            0.0
        };
        let flowers_growth = if light_ok { temp_factor * 0.2 * dt } else { -0.05 * dt };
        self.feature_b.level = (self.feature_b.level + flowers_growth).clamp(0.0, 1.0);
        self.feature_b.active = light_ok && temp_factor > 0.5;

        // Feature C: Strength responds to overall conditions
        let strength_growth = (moisture * 0.15 + if light_ok { 0.1 } else { 0.0 }) * dt;
        self.feature_c.level = (self.feature_c.level + strength_growth).clamp(0.0, 1.0);
        self.feature_c.active = moisture > 0.4 && light_ok;
    }
}
