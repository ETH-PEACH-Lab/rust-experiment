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
                name: "Blossom".to_string(),
                level: 0.0,
                active: false,
            },
            feature_b: Feature {
                name: "Fruit".to_string(),
                level: 0.0,
                active: false,
            },
            feature_c: Feature {
                name: "Vigor".to_string(),
                level: 0.0,
                active: false,
            },
        }
    }

    pub fn tick(&mut self, dt: f64, moisture: f64, light_ok: bool, temperature: f64) {
        // Feature A: Blossom - responds to moisture
        let moisture_good = moisture > 0.4 && moisture < 0.8;
        if moisture_good {
            self.feature_a.level = (self.feature_a.level + dt * 0.15).min(1.0);
            self.feature_a.active = true;
        } else {
            self.feature_a.level = (self.feature_a.level - dt * 0.1).max(0.0);
            if self.feature_a.level < 0.1 {
                self.feature_a.active = false;
            }
        }

        // Feature B: Fruit - responds to light
        if light_ok {
            self.feature_b.level = (self.feature_b.level + dt * 0.12).min(1.0);
            self.feature_b.active = true;
        } else {
            self.feature_b.level = (self.feature_b.level - dt * 0.08).max(0.0);
            if self.feature_b.level < 0.1 {
                self.feature_b.active = false;
            }
        }

        // Feature C: Vigor - responds to temperature
        let temp_optimal = temperature > 18.0 && temperature < 28.0;
        if temp_optimal {
            self.feature_c.level = (self.feature_c.level + dt * 0.1).min(1.0);
            self.feature_c.active = true;
        } else {
            self.feature_c.level = (self.feature_c.level - dt * 0.12).max(0.0);
            if self.feature_c.level < 0.1 {
                self.feature_c.active = false;
            }
        }
    }
}
