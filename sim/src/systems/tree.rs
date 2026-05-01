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
                name: "Bloom".to_string(),
                level: 0.5,
                active: false,
            },
            feature_b: Feature {
                name: "Root System".to_string(),
                level: 0.5,
                active: false,
            },
            feature_c: Feature {
                name: "Resilience".to_string(),
                level: 0.8,
                active: false,
            },
        }
    }

    pub fn tick(&mut self, dt: f64, moisture: f64, light_ok: bool, temperature: f64) {
        // Feature A: Bloom - responds to light and temperature
        let bloom_favorable = light_ok && temperature > 20.0 && temperature < 35.0;
        if bloom_favorable {
            self.feature_a.level = (self.feature_a.level + 0.003 * dt).min(1.0);
        } else {
            self.feature_a.level = (self.feature_a.level - 0.002 * dt).max(0.0);
        }
        self.feature_a.active = self.feature_a.level > 0.3;

        // Feature B: Root System - responds to moisture
        let moisture_healthy = moisture >= 0.3 && moisture <= 0.8;
        if moisture_healthy {
            self.feature_b.level = (self.feature_b.level + 0.003 * dt).min(1.0);
        } else {
            self.feature_b.level = (self.feature_b.level - 0.002 * dt).max(0.0);
        }
        self.feature_b.active = self.feature_b.level > 0.4;

        // Feature C: Resilience - responds to all conditions combined
        let all_conditions_good = light_ok && moisture_healthy && temperature > 15.0 && temperature < 30.0;
        if all_conditions_good {
            self.feature_c.level = (self.feature_c.level + 0.002 * dt).min(1.0);
            self.feature_c.active = true;
        } else {
            self.feature_c.level = (self.feature_c.level - 0.001 * dt).max(0.0);
            self.feature_c.active = false;
        }
    }
}
