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
                name: "Root System".to_string(),
                level: 0.3,
                active: false,
            },
            feature_b: Feature {
                name: "Canopy".to_string(),
                level: 0.2,
                active: false,
            },
            feature_c: Feature {
                name: "Resilience".to_string(),
                level: 0.5,
                active: false,
            },
        }
    }

    pub fn tick(&mut self, dt: f64, moisture: f64, light_ok: bool, temperature: f64) {
        // Feature A: Root System — Gaussian stress based on ideal soil moisture (0.5)
        let moisture_sigma: f64 = 0.12;
        let moisture_stress = 1.0 - (-((moisture - 0.5).powi(2) / (2.0 * moisture_sigma.powi(2))).exp());

        if moisture_stress > 0.05 {
            self.feature_a.level = (self.feature_a.level + 0.001 * dt).min(1.0);
            self.feature_a.active = true;
        } else {
            self.feature_a.level = (self.feature_a.level - 0.0005 * dt).max(0.0);
            self.feature_a.active = false;
        }

        // Feature B: Canopy — light stress (1.0 when dark, ~0.1 when light available)
        let light_stress = if light_ok { 0.1 } else { 1.0 };

        if light_stress > 0.2 {
            self.feature_b.level = (self.feature_b.level + 0.0015 * dt).min(1.0);
            self.feature_b.active = true;
        } else {
            self.feature_b.level = (self.feature_b.level - 0.0008 * dt).max(0.0);
            self.feature_b.active = false;
        }

        // Feature C: Resilience — Gaussian stress based on ideal temperature (22.5°C)
        let temp_sigma: f64 = 10.0;
        let temp_stress = 1.0 - (-((temperature - 22.5).powi(2) / (2.0 * temp_sigma.powi(2))).exp());

        if temp_stress > 0.05 {
            self.feature_c.level = (self.feature_c.level + 0.0006 * dt).min(1.0);
            self.feature_c.active = true;
        } else {
            self.feature_c.level = (self.feature_c.level - 0.0002 * dt).max(0.0);
        }

        // Update tree health based on overall feature health
        let avg_level = (self.feature_a.level + self.feature_b.level + self.feature_c.level) / 3.0;
        self.health = (avg_level * 0.8 + 0.2).min(1.0);
    }
}
