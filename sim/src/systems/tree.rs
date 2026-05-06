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
            feature_a: Feature { name: "Root System".to_string(), level: 0.5, active: false },
            feature_b: Feature { name: "Canopy".to_string(), level: 0.5, active: false },
            feature_c: Feature { name: "Resilience".to_string(), level: 0.5, active: false },
        }
    }

    pub fn tick(&mut self, dt: f64, moisture: f64, light_ok: bool, temperature: f64) {
        // feature_a: Root System responds to moisture
        let moisture_factor = (moisture - 0.3).max(0.0) / 0.7;
        let root_target = moisture_factor.min(1.0);
        self.feature_a.level = (self.feature_a.level + 0.01 * dt * (root_target - self.feature_a.level)).clamp(0.0, 1.0);
        self.feature_a.active = moisture > 0.5;

        // feature_b: Canopy responds to light
        let canopy_target = if light_ok { 0.8 } else { 0.2 };
        self.feature_b.level = (self.feature_b.level + 0.01 * dt * (canopy_target - self.feature_b.level)).clamp(0.0, 1.0);
        self.feature_b.active = light_ok;

        // feature_c: Resilience responds to temperature stress
        let temp_stress = if temperature > 30.0 {
            (temperature - 30.0).min(5.0) / 5.0
        } else if temperature < 15.0 {
            (15.0 - temperature).min(5.0) / 5.0
        } else {
            0.0
        };
        let resilience_target = (1.0 - temp_stress).clamp(0.0, 1.0);
        self.feature_c.level = (self.feature_c.level + 0.01 * dt * (resilience_target - self.feature_c.level)).clamp(0.0, 1.0);
        self.feature_c.active = temp_stress < 0.2;

        // Update tree health based on features
        self.health = (self.feature_a.level + self.feature_b.level + self.feature_c.level) / 3.0;
    }
}
