use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum FertilizerType {
    Nitrogen,
    Phosphorus,
    Balanced,
}

impl FertilizerType {
    pub fn boost_multiplier(&self) -> f64 {
        unimplemented!()
    }
}

pub fn seasonal_rate(_season: &crate::garden::Season) -> f64 {
    unimplemented!()
}

pub fn decayed_boost(_initial: f64, _age_secs: f64) -> f64 {
    unimplemented!()
}
