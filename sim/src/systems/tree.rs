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
                level: 0.3, 
                active: false 
            },
            feature_b: Feature { 
                name: "Flowers".to_string(), 
                level: 0.0, 
                active: false 
            },
            feature_c: Feature { 
                name: "Fruit".to_string(), 
                level: 0.0, 
                active: false 
            },
        }
    }

    pub fn tick(&mut self, dt: f64, moisture: f64, light_ok: bool, temperature: f64) {
        // Feature A: Leaves grow with good moisture
        if moisture > 0.4 {
            self.feature_a.level = (self.feature_a.level + 0.001 * dt).min(1.0);
            self.feature_a.active = true;
        } else {
            self.feature_a.level = (self.feature_a.level - 0.0005 * dt).max(0.0);
        }

        // Feature B: Flowers bloom with good light
        if light_ok && temperature > 18.0 {
            self.feature_b.level = (self.feature_b.level + 0.002 * dt).min(1.0);
            self.feature_b.active = true;
        } else {
            self.feature_b.level = (self.feature_b.level - 0.001 * dt).max(0.0);
        }

        // Feature C: Fruit develops in warm conditions
        if temperature >= 22.0 && moisture > 0.3 && light_ok {
            self.feature_c.level = (self.feature_c.level + 0.0015 * dt).min(1.0);
            self.feature_c.active = true;
        } else {
            self.feature_c.level = (self.feature_c.level - 0.0005 * dt).max(0.0);
        }
    }
}
