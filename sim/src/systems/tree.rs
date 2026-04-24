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
                name: "Flowers".to_string(),
                level: 0.0,
                active: false,
            },
            feature_c: Feature {
                name: "Fruit".to_string(),
                level: 0.0,
                active: false,
            },
        }
    }

    pub fn tick(&mut self, dt: f64, moisture: f64, light_ok: bool, temperature: f64) {
        // Feature A: Leaves (responds to moisture)
        // Higher moisture encourages leaf growth
        let leaf_growth = if moisture > 0.4 { 0.3 * dt } else { -0.1 * dt };
        self.feature_a.level = (self.feature_a.level + leaf_growth).clamp(0.0, 1.0);
        self.feature_a.active = self.feature_a.level > 0.2;

        // Feature B: Flowers (responds to light)
        // Good light enables flowering
        let flower_growth = if light_ok { 0.2 * dt } else { -0.15 * dt };
        self.feature_b.level = (self.feature_b.level + flower_growth).clamp(0.0, 1.0);
        self.feature_b.active = light_ok && self.feature_b.level > 0.1;

        // Feature C: Fruit (responds to temperature)
        // Optimal temperature (20–25°C) encourages fruit development
        let temp_optimal = (temperature - 22.5).abs() < 5.0;
        let fruit_growth = if temp_optimal { 0.25 * dt } else { -0.05 * dt };
        self.feature_c.level = (self.feature_c.level + fruit_growth).clamp(0.0, 1.0);
        self.feature_c.active = temp_optimal && self.feature_c.level > 0.15;

        // Tree health declines if conditions are poor
        let conditions_ok = moisture > 0.3 && light_ok && (15.0..=30.0).contains(&temperature);
        if !conditions_ok {
            self.health = (self.health - 0.001 * dt).clamp(0.0, 1.0);
        }
    }
}
