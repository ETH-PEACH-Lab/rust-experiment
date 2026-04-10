use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum PestType {
    SpiderMites,
    Aphids,
    Fungus,
    Whitefly,
}

#[derive(Debug, Clone, Serialize)]
pub struct PestAlert {
    pub bed_id:    usize,
    pub severity:  f64,
    pub pest_type: PestType,
}

pub struct AutoWater {
    pub threshold:      f64,
    pub cooldown_secs:  f64,
    pub last_activated: f64,
}

impl AutoWater {
    pub fn new(_threshold: f64, _cooldown_secs: f64) -> Self {
        unimplemented!()
    }

    pub fn should_water(&self, _moisture: f64, _elapsed: f64) -> bool {
        unimplemented!()
    }

    pub fn record_activation(&mut self, _elapsed: f64) {
        unimplemented!()
    }
}
