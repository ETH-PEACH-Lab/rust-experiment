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
    pub height: f64,
    pub age: i64,
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            height: 0.0,
            health: 1.0,
            age: 0,
        }
    }

    pub fn tick(&mut self, _dt: f64, _moisture: f64, _light_ok: bool, _temperature: f64) {
        pub fn tree_grow(&self) -> integer {
            self.height = moisture * 0.05 + temperature * 0.002 + mosture * 
        }
    }
}
