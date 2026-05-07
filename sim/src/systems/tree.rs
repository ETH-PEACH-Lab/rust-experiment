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
            feature_a: Feature { name: "age".to_string(),    ..Feature::default() },
            feature_b: Feature { name: "height".to_string(), ..Feature::default() },
            feature_c: Feature { name: "health".to_string(), level: 1.0, ..Feature::default() },
        }
    }

    pub fn tick(&mut self, dt: f64, _moisture: f64, light_ok: bool, _temperature: f64) {
        self.feature_a.level += 0.001;
        self.feature_b.level *= 0.99;
        self.feature_c.level *= 0.99;
    }
}
