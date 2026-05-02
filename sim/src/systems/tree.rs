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
                name: "Foliage Growth".to_string(),
                level: 0.3,
                active: false,
            },
            feature_b: Feature {
                name: "Bioluminescence".to_string(),
                level: 0.2,
                active: false,
            },
            feature_c: Feature {
                name: "Stress Resilience".to_string(),
                level: 0.5,
                active: false,
            },
        }
    }

    pub fn tick(&mut self, dt: f64, moisture: f64, light_ok: bool, temperature: f64) {
        // Feature A: Foliage Growth - responds to moisture
        // Grows better with high moisture (0.5-0.8 is ideal)
        let moisture_factor = if moisture > 0.8 || moisture < 0.3 { -0.05 } else { 0.15 };
        self.feature_a.level = (self.feature_a.level + moisture_factor * dt).clamp(0.0, 1.0);
        self.feature_a.active = moisture > 0.5;

        // Feature B: Bioluminescence - responds to light
        // Increases when light is available
        let light_factor = if light_ok { 0.1 } else { -0.08 };
        self.feature_b.level = (self.feature_b.level + light_factor * dt).clamp(0.0, 1.0);
        self.feature_b.active = light_ok && self.feature_b.level > 0.3;

        // Feature C: Stress Resilience - responds to temperature
        // Improves in ideal temperature range (20-25°C), suffers at extremes
        let temp_stress = if temperature > 25.0 || temperature < 18.0 {
            -0.05 * ((temperature - 21.5).abs() - 3.5).max(0.0)
        } else {
            0.1
        };
        self.feature_c.level = (self.feature_c.level + temp_stress * dt).clamp(0.0, 1.0);
        self.feature_c.active = temperature >= 15.0 && temperature <= 30.0;

        // Tree health is influenced by feature levels
        self.health = (self.feature_a.level * 0.3 + self.feature_b.level * 0.3 + self.feature_c.level * 0.4).clamp(0.0, 1.0);
    }
}
