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
            feature_a: Feature { name: "Root System".to_string(), level: 0.0, active: false },
            feature_b: Feature { name: "Canopy".to_string(), level: 0.0, active: false },
            feature_c: Feature { name: "Flower Buds".to_string(), level: 0.0, active: false },
        }
    }

    pub fn tick(&mut self, dt: f64, moisture: f64, light_ok: bool, temperature: f64) {
        let good_conditions = moisture > 0.5 && light_ok && temperature > 15.0 && temperature < 30.0;

        // Feature A (Root System): responds to moisture
        if moisture > 0.5 {
            self.feature_a.level = (self.feature_a.level + 0.001 * dt).min(1.0);
            self.feature_a.active = true;
        } else {
            self.feature_a.level = (self.feature_a.level - 0.0005 * dt).max(0.0);
            self.feature_a.active = false;
        }

        // Feature B (Canopy): responds to light
        if light_ok {
            self.feature_b.level = (self.feature_b.level + 0.001 * dt).min(1.0);
            self.feature_b.active = true;
        } else {
            self.feature_b.level = (self.feature_b.level - 0.0005 * dt).max(0.0);
            self.feature_b.active = false;
        }

        // Feature C (Flower Buds): responds to all good conditions
        if good_conditions {
            self.feature_c.level = (self.feature_c.level + 0.0008 * dt).min(1.0);
            self.feature_c.active = true;
        } else {
            self.feature_c.level = (self.feature_c.level - 0.0003 * dt).max(0.0);
            self.feature_c.active = false;
        }
    }
}
