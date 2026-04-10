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
            feature_a: Feature::default(),
            feature_b: Feature::default(),
            feature_c: Feature::default(),
        }
    }

    pub fn tick(&mut self, _dt: f64, _moisture: f64, _light_ok: bool, _temperature: f64) {
    }
}
