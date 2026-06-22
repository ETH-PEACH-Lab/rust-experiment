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
            health:    1.0,
            feature_a: Feature {
                name: "Foliage".to_string(),
                level: 0.5,
                active: false,
            },
            feature_b: Feature {
                name: "Flowers".to_string(),
                level: 0.2,
                active: false,
            },
            feature_c: Feature {
                name: "Fruit".to_string(),
                level: 0.0,
                active: false,
            },
        }
    }

    pub fn tick(&mut self, dt: f64, moisture: f64, light_ok: bool, temperature: f64) {
        // Feature A (Foliage) responds to moisture
        let moisture_factor = if moisture > 0.5 { 0.05 } else { -0.02 };
        self.feature_a.level = (self.feature_a.level + moisture_factor * dt).clamp(0.0, 1.0);
        self.feature_a.active = moisture > 0.5;

        // Feature B (Flowers) responds to light
        let light_factor = if light_ok { 0.03 } else { -0.01 };
        self.feature_b.level = (self.feature_b.level + light_factor * dt).clamp(0.0, 1.0);
        self.feature_b.active = light_ok && self.feature_b.level > 0.3;

        // Feature C (Fruit) responds to temperature in optimal range
        let temp_optimal = temperature > 18.0 && temperature < 26.0;
        let temp_factor = if temp_optimal { 0.02 } else { -0.01 };
        self.feature_c.level = (self.feature_c.level + temp_factor * dt).clamp(0.0, 1.0);
        self.feature_c.active = temp_optimal && self.feature_c.level > 0.1;

        // Update overall health based on feature levels
        self.health = (self.feature_a.level + self.feature_b.level + self.feature_c.level) / 3.0;
    }
}
