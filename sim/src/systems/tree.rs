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
                name: "Roots".to_string(),
                level: 0.0,
                active: false,
            },
            feature_b: Feature {
                name: "Leaves".to_string(),
                level: 0.0,
                active: false,
            },
            feature_c: Feature {
                name: "Flowers".to_string(),
                level: 0.0,
                active: false,
            },
        }
    }

    pub fn tick(&mut self, dt: f64, moisture: f64, light_ok: bool, temperature: f64) {
        // Feature A: Roots respond to moisture
        // Active when moisture is good (0.3 - 0.8), grows/shrinks accordingly
        let roots_target = if moisture >= 0.3 && moisture <= 0.8 {
            1.0 - (moisture - 0.55).abs() * 2.0  // Peak at 0.55
        } else {
            0.0
        };
        self.feature_a.active = roots_target > 0.3;
        self.feature_a.level = (self.feature_a.level + (roots_target - self.feature_a.level) * 0.01 * dt).clamp(0.0, 1.0);

        // Feature B: Leaves respond to light
        // Active when light is good, gradually grows to 1.0
        let leaves_target = if light_ok { 1.0 } else { 0.0 };
        self.feature_b.active = light_ok;
        self.feature_b.level = (self.feature_b.level + (leaves_target - self.feature_b.level) * 0.008 * dt).clamp(0.0, 1.0);

        // Feature C: Flowers respond to temperature and light together
        // Bloom when temperature is in range (18-28°C) and light is good
        let temp_ok = temperature >= 18.0 && temperature <= 28.0;
        let flowers_target = if temp_ok && light_ok { 1.0 } else { 0.0 };
        self.feature_c.active = temp_ok && light_ok;
        self.feature_c.level = (self.feature_c.level + (flowers_target - self.feature_c.level) * 0.006 * dt).clamp(0.0, 1.0);

        // Health decreases if conditions are poor
        let conditions_ok = (moisture >= 0.25 && moisture <= 0.85) && light_ok && temp_ok;
        if !conditions_ok {
            self.health = (self.health - 0.001 * dt).clamp(0.0, 1.0);
        } else {
            self.health = (self.health + 0.0005 * dt).clamp(0.0, 1.0);
        }
    }
}
