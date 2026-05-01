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
                name: "a".to_string(),
                level: 0.0,
                active: false,
            },
            feature_b: Feature {
                name: "b".to_string(),
                level: 0.0,
                active: false,
            },
            feature_c: Feature {
                name: "c".to_string(),
                level: 0.0,
                active: false,
            },
        }
    }

    pub fn tick(&mut self, dt: f64, moisture: f64, light_ok: bool, temperature: f64) {
        // Feature a: responds to moisture
        if moisture > 0.3 {
            self.feature_a.level = (self.feature_a.level + 0.01 * dt).min(1.0);
            self.feature_a.active = true;
        } else {
            self.feature_a.level = (self.feature_a.level - 0.005 * dt).max(0.0);
            self.feature_a.active = false;
        }

        // Feature b: responds to light
        if light_ok {
            self.feature_b.level = (self.feature_b.level + 0.01 * dt).min(1.0);
            self.feature_b.active = true;
        } else {
            self.feature_b.level = (self.feature_b.level - 0.005 * dt).max(0.0);
            self.feature_b.active = false;
        }

        // Feature c: responds to temperature (15-30°C range)
        if temperature >= 15.0 && temperature <= 30.0 {
            self.feature_c.level = (self.feature_c.level + 0.01 * dt).min(1.0);
            self.feature_c.active = true;
        } else {
            self.feature_c.level = (self.feature_c.level - 0.005 * dt).max(0.0);
            self.feature_c.active = false;
        }
    }
}
