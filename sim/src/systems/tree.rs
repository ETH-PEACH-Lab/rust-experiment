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
                name: "Leaf System".to_string(),
                level: 0.3,
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
        // Feature A: Root System — grows with good moisture
        let moisture_factor = if moisture > 0.4 && moisture < 0.8 { 0.5 } else { -0.1 };
        self.feature_a.level = (self.feature_a.level + moisture_factor * dt).clamp(0.0, 1.0);
        self.feature_a.active = moisture > 0.5;

        // Feature B: Leaf System — grows with good light
        let light_factor = if light_ok { 0.3 } else { -0.2 };
        self.feature_b.level = (self.feature_b.level + light_factor * dt).clamp(0.0, 1.0);
        self.feature_b.active = light_ok;

        // Feature C: Resilience — grows when temperature is in good range
        let temp_factor = if temperature > 18.0 && temperature < 28.0 { 0.2 } else { -0.3 };
        self.feature_c.level = (self.feature_c.level + temp_factor * dt).clamp(0.0, 1.0);
        self.feature_c.active = temperature > 15.0 && temperature < 30.0;

        // Update tree health based on overall conditions
        let all_good = moisture > 0.4 && light_ok && temperature > 15.0 && temperature < 30.0;
        self.health = if all_good {
            (self.health + 0.01 * dt).min(1.0)
        } else {
            (self.health - 0.02 * dt).max(0.0)
        };
    }
}
