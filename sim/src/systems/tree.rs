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
            feature_a: Feature { name: "Roots".to_string(), level: 0.5, active: false },
            feature_b: Feature { name: "Canopy".to_string(), level: 0.3, active: false },
            feature_c: Feature { name: "Blossoms".to_string(), level: 0.0, active: false },
        }
    }

    pub fn tick(&mut self, dt: f64, moisture: f64, light_ok: bool, temperature: f64) {
        // Feature A: Roots — deepen with moisture availability
        let root_growth = if moisture > 0.5 { 0.001 * dt } else { -0.0005 * dt };
        self.feature_a.level = (self.feature_a.level + root_growth).clamp(0.0, 1.0);
        self.feature_a.active = moisture > 0.4;

        // Feature B: Canopy — expand when light is good
        let canopy_growth = if light_ok { 0.0008 * dt } else { -0.0003 * dt };
        self.feature_b.level = (self.feature_b.level + canopy_growth).clamp(0.0, 1.0);
        self.feature_b.active = light_ok;

        // Feature C: Blossoms — bloom when warm and healthy
        let bloom_threshold = temperature > 20.0 && temperature < 30.0;
        let blossom_growth = if bloom_threshold && self.feature_a.level > 0.3 && self.feature_b.level > 0.2 {
            0.0012 * dt
        } else {
            -0.0006 * dt
        };
        self.feature_c.level = (self.feature_c.level + blossom_growth).clamp(0.0, 1.0);
        self.feature_c.active = self.feature_c.level > 0.5;

        // Update tree health based on overall conditions
        if moisture < 0.2 || temperature < 10.0 || temperature > 40.0 {
            self.health = (self.health - 0.0002 * dt).clamp(0.0, 1.0);
        } else if light_ok && moisture > 0.4 {
            self.health = (self.health + 0.0001 * dt).clamp(0.0, 1.0);
        }
    }
}
