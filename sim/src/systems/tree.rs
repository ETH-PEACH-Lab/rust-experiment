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
                name: "Leaves".to_string(),
                level: 0.5,
                active: true,
            },
            feature_b: Feature {
                name: "Root System".to_string(),
                level: 0.5,
                active: true,
            },
            feature_c: Feature {
                name: "Flowers".to_string(),
                level: 0.0,
                active: false,
            },
        }
    }

    pub fn tick(&mut self, dt: f64, moisture: f64, light_ok: bool, temperature: f64) {
        // Feature A: Leaves respond to light and temperature
        if light_ok {
            self.feature_a.active = true;
            self.feature_a.level = (self.feature_a.level + 0.005 * dt).min(1.0);
        } else {
            self.feature_a.active = false;
            self.feature_a.level = (self.feature_a.level - 0.003 * dt).max(0.0);
        }

        // Feature B: Root System responds to moisture availability
        let moisture_optimal = (moisture - 0.3).abs() < 0.3;
        if moisture_optimal {
            self.feature_b.active = true;
            self.feature_b.level = (self.feature_b.level + 0.004 * dt).min(1.0);
        } else {
            self.feature_b.active = false;
            self.feature_b.level = (self.feature_b.level - 0.002 * dt).max(0.0);
        }

        // Feature C: Flowers bloom when light is good and temp is warm
        let temp_good = temperature > 18.0 && temperature < 28.0;
        if light_ok && temp_good && self.feature_a.level > 0.4 {
            self.feature_c.active = true;
            self.feature_c.level = (self.feature_c.level + 0.006 * dt).min(1.0);
        } else {
            self.feature_c.active = false;
            self.feature_c.level = (self.feature_c.level - 0.003 * dt).max(0.0);
        }

        // Update tree health based on features
        let avg_feature_level = (self.feature_a.level + self.feature_b.level + self.feature_c.level) / 3.0;
        self.health = 0.5 + avg_feature_level * 0.5;
    }
}
