use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum FertilizerType {
    Nitrogen,
    Phosphorus,
    Balanced,
}

impl FertilizerType {
    pub fn boost_multiplier(&self) -> f64 {
        match self {
            FertilizerType::Nitrogen   => 1.5,
            FertilizerType::Phosphorus => 1.3,
            FertilizerType::Balanced   => 1.4,
        }
    }
}

pub fn seasonal_rate(season: &crate::garden::Season) -> f64 {
    match season {
        crate::garden::Season::Spring => 1.2,
        crate::garden::Season::Summer => 1.0,
        crate::garden::Season::Autumn => 0.8,
        crate::garden::Season::Winter => 0.5,
    }
}

/// Exponential decay of a fertilizer boost back toward the baseline of 1.0.
/// Half-life is roughly 5 minutes (300 s).
pub fn decayed_boost(initial: f64, age_secs: f64) -> f64 {
    const HALF_LIFE: f64 = 300.0;
    let decay = (-age_secs.max(0.0) * std::f64::consts::LN_2 / HALF_LIFE).exp();
    1.0 + (initial - 1.0) * decay
}
