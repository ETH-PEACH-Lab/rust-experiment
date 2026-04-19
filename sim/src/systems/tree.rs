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
                name: "Roots".to_string(),
                level: 0.0,
                active: false,
            },
            feature_b: Feature {
                name: "Leaves".to_string(),
                level: 0.0,
                active: false,
            },
            feature_c: Feature {
                name: "Blossoms".to_string(),
                level: 0.0,
                active: false,
            },
        }
    }

    pub fn tick(&mut self, dt: f64, moisture: f64, light_ok: bool, temperature: f64) {
        // Feature A: Roots grow with moisture
        self.feature_a.active = moisture > 0.4;
        if self.feature_a.active {
            self.feature_a.level = (self.feature_a.level + 0.002 * dt).min(1.0);
        } else {
            self.feature_a.level = (self.feature_a.level - 0.001 * dt).max(0.0);
        }

        // Feature B: Leaves grow with light
        self.feature_b.active = light_ok;
        if self.feature_b.active {
            self.feature_b.level = (self.feature_b.level + 0.0015 * dt).min(1.0);
        } else {
            self.feature_b.level = (self.feature_b.level - 0.0008 * dt).max(0.0);
        }

        // Feature C: Blossoms respond to ideal temperature (20-25°C)
        let temp_ideal = temperature > 18.0 && temperature < 28.0;
        self.feature_c.active = temp_ideal;
        if self.feature_c.active {
            self.feature_c.level = (self.feature_c.level + 0.001 * dt).min(1.0);
        } else {
            self.feature_c.level = (self.feature_c.level - 0.0005 * dt).max(0.0);
        }

        // Tree health influenced by feature activity
        let active_features = [self.feature_a.active, self.feature_b.active, self.feature_c.active]
            .iter()
            .filter(|&&a| a)
            .count() as f64;
        if active_features >= 2.0 {
            self.health = (self.health + 0.0001 * dt).min(1.0);
        }
    }
}
