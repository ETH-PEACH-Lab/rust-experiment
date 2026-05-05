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
                level: 0.0,
                active: true,
            },
            feature_b: Feature {
                name: "Blossoms".to_string(),
                level: 0.0,
                active: false,
            },
            feature_c: Feature {
                name: "Birds".to_string(),
                level: 0.0,
                active: false,
            },
        }
    }

    pub fn tick(&mut self, dt: f64, _moisture: f64, light_ok: bool, temperature: f64) {
        // Leaves: stable, present when light and temperature are good
        let temp_ok = temperature >= 15.0 && temperature <= 30.0;
        if light_ok && temp_ok {
            self.feature_a.level = 1.0;
            self.feature_a.active = true;
        } else {
            self.feature_a.level = (self.feature_a.level - 0.002 * dt).clamp(0.0, 1.0);
            self.feature_a.active = self.feature_a.level > 0.1;
        }

        // Blossoms: increase linearly with light and temperature
        let temp_normalized = ((temperature - 15.0) / (30.0 - 15.0)).clamp(0.0, 1.0);
        let blossom_factor = if light_ok { temp_normalized } else { 0.0 };
        self.feature_b.level = (self.feature_b.level + blossom_factor * 0.02 * dt).clamp(0.0, 1.0);
        self.feature_b.active = self.feature_b.level > 0.0;

        // Birds: active only when blossoms > 0.5
        if self.feature_b.level > 0.5 {
            self.feature_c.active = true;
            self.feature_c.level = (self.feature_c.level + 0.003 * dt).clamp(0.0, 1.0);
        } else {
            self.feature_c.active = false;
            self.feature_c.level = (self.feature_c.level - 0.005 * dt).clamp(0.0, 1.0);
        }
    }
}
