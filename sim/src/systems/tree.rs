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
            // Initialize three features with distinct names and purposes
            feature_a: Feature { name: "Leaves".to_string(), level: 0.0, active: false },
            feature_b: Feature { name: "Blossoms".to_string(), level: 0.0, active: false },
            feature_c: Feature { name: "Branches".to_string(), level: 0.0, active: false },
        }
    }

    pub fn tick(&mut self, dt: f64, moisture: f64, light_ok: bool, temperature: f64) {
        // Feature A - Leaves: Responds to moisture
        // Leaves grow when soil has adequate moisture (> 0.4)
        let moisture_good = moisture > 0.4;
        self.feature_a.active = moisture_good;
        let leaf_growth = if moisture_good { 0.0002 * dt } else { -0.0001 * dt };
        self.feature_a.level = (self.feature_a.level + leaf_growth).clamp(0.0, 1.0);

        // Feature B - Blossoms: Responds to light
        // Blossoms grow when there's sufficient light
        self.feature_b.active = light_ok;
        let blossom_growth = if light_ok { 0.0002 * dt } else { -0.0001 * dt };
        self.feature_b.level = (self.feature_b.level + blossom_growth).clamp(0.0, 1.0);

        // Feature C - Branches: Responds to temperature
        // Branches grow in optimal temperature range (15–25°C)
        let temp_optimal = temperature >= 15.0 && temperature <= 25.0;
        self.feature_c.active = temp_optimal;
        let branch_growth = if temp_optimal { 0.0002 * dt } else { -0.0001 * dt };
        self.feature_c.level = (self.feature_c.level + branch_growth).clamp(0.0, 1.0);
    }
}
