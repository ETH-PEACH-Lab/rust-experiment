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
                active: false,
            },
            feature_b: Feature {
                name: "Fireflies".to_string(),
                level: 0.0,
                active: false,
            },
            feature_c: Feature::default(),
        }
    }

    pub fn tick(&mut self, dt: f64, _moisture: f64, light_ok: bool, temperature: f64) {
        // Feature A: Leaves grow when there's sufficient light over time
        if light_ok && self.feature_a.level < 1.0 {
            self.feature_a.level = (self.feature_a.level + dt * 0.15).min(1.0);
            self.feature_a.active = true;
        } else if !light_ok {
            self.feature_a.level = (self.feature_a.level - dt * 0.05).max(0.0);
        }

        // Feature B: Fireflies appear when there's no good light AND temperature is warm (>15°C)
        // This simulates fireflies coming out during dark/poor light periods when it's warm enough
        let is_dark_period = !light_ok;
        if is_dark_period && temperature > 15.0 {
            self.feature_b.level = (self.feature_b.level + dt * 0.2).min(1.0);
            self.feature_b.active = true;
        } else {
            self.feature_b.level = (self.feature_b.level - dt * 0.15).max(0.0);
            self.feature_b.active = self.feature_b.level > 0.0;
        }
    }
}
