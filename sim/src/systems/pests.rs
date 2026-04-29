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

#[derive(Debug, Clone, Serialize)]
pub struct AutoWater {
    pub threshold:      f64,
    pub cooldown_secs:  f64,
    pub last_activated: f64,
}

impl AutoWater {
    pub fn new(threshold: f64, cooldown_secs: f64) -> Self {
        AutoWater {
            threshold:      threshold.clamp(0.0, 1.0),
            cooldown_secs:  cooldown_secs.max(0.0),
            last_activated: f64::NEG_INFINITY,
        }
    }

    pub fn should_water(&self, moisture: f64, elapsed: f64) -> bool {
        moisture < self.threshold && (elapsed - self.last_activated) >= self.cooldown_secs
    }

    pub fn record_activation(&mut self, elapsed: f64) {
        self.last_activated = elapsed;
    }
}
