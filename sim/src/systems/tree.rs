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
                name:   "Leaves".to_string(),
                level:  0.0,
                active: false,
            },
            feature_b: Feature {
                name:   "Flowers".to_string(),
                level:  0.0,
                active: false,
            },
            feature_c: Feature {
                name:   "Root System".to_string(),
                level:  0.0,
                active: false,
            },
        }
    }

    pub fn tick(&mut self, dt: f64, moisture: f64, light_ok: bool, temperature: f64) {
        // Feature A: Leaves — grows with light and moderate temperature
        let temp_ok = temperature >= 18.0 && temperature <= 28.0;
        let leaf_growth = if light_ok && temp_ok {
            0.15 * dt  // Strong growth in good conditions
        } else if light_ok {
            0.05 * dt  // Slow growth with light but poor temp
        } else {
            -0.08 * dt // Decay without light
        };
        self.feature_a.level = (self.feature_a.level + leaf_growth).clamp(0.0, 1.0);
        self.feature_a.active = self.feature_a.level > 0.5;

        // Feature B: Flowers — grows with moisture and light
        let moisture_good = moisture >= 0.4 && moisture <= 0.8;
        let flower_growth = if light_ok && moisture_good {
            0.12 * dt  // Blooms with water and light
        } else if moisture_good {
            0.03 * dt  // Slow growth with moisture
        } else {
            -0.06 * dt // Wilt when dry
        };
        self.feature_b.level = (self.feature_b.level + flower_growth).clamp(0.0, 1.0);
        self.feature_b.active = self.feature_b.level > 0.4;

        // Feature C: Root System — grows with available moisture
        let root_growth = if moisture > 0.3 {
            0.1 * dt * moisture  // Grows stronger with more moisture
        } else {
            -0.05 * dt  // Weakens in drought
        };
        self.feature_c.level = (self.feature_c.level + root_growth).clamp(0.0, 1.0);
        self.feature_c.active = self.feature_c.level > 0.3;

        // Tree health affected by overall feature development
        let avg_feature_level = (self.feature_a.level + self.feature_b.level + self.feature_c.level) / 3.0;
        self.health = (avg_feature_level * 0.8 + 0.2).clamp(0.0, 1.0);
    }
}
