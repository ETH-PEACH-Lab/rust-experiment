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
                level: 0.3,
                active: false,
            },
            feature_b: Feature {
                name: "Roots".to_string(),
                level: 0.5,
                active: false,
            },
            feature_c: Feature {
                name: "Blooms".to_string(),
                level: 0.0,
                active: false,
            },
        }
    }

    pub fn tick(&mut self, dt: f64, moisture: f64, light_ok: bool, temperature: f64) {
        // Feature A: Foliage - responds to light and moisture
        if light_ok && moisture > 0.4 {
            self.feature_a.level = (self.feature_a.level + 0.001 * dt).min(1.0);
            self.feature_a.active = true;
        } else {
            self.feature_a.level = (self.feature_a.level - 0.0005 * dt).max(0.0);
            self.feature_a.active = self.feature_a.level > 0.1;
        }

        // Feature B: Roots - responds to moisture
        if moisture > 0.5 {
            self.feature_b.level = (self.feature_b.level + 0.0008 * dt).min(1.0);
            self.feature_b.active = true;
        } else if moisture < 0.3 {
            self.feature_b.level = (self.feature_b.level - 0.0003 * dt).max(0.0);
            self.feature_b.active = false;
        } else {
            self.feature_b.active = self.feature_b.level > 0.3;
        }

        // Feature C: Blooms - responds to temperature and light
        let temp_ok = temperature > 18.0 && temperature < 28.0;
        if light_ok && temp_ok && moisture > 0.35 {
            self.feature_c.level = (self.feature_c.level + 0.0012 * dt).min(1.0);
            self.feature_c.active = true;
        } else {
            self.feature_c.level = (self.feature_c.level - 0.0002 * dt).max(0.0);
            self.feature_c.active = self.feature_c.level > 0.2;
        }

        // Update health based on feature conditions
        let avg_feature_level = (self.feature_a.level + self.feature_b.level + self.feature_c.level) / 3.0;
        self.health = (0.5 + avg_feature_level * 0.5).clamp(0.0, 1.0);
    }
}
