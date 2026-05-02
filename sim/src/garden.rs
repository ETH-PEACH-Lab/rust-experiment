use serde::Serialize;
use crate::systems;

pub const NUM_BEDS: usize = 4;

pub const MOISTURE_MIN: f64 = 0.25;
pub const MOISTURE_MAX: f64 = 0.85;

pub const TEMP_MIN: f64 = 15.0;
pub const TEMP_MAX: f64 = 30.0;

const WATER_FLOW: f64 = 0.04;
const EVAP_BASE:  f64 = 0.008;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum PlantStage {
    Seed,
    Sprout,
    Growing,
    Flowering,
}

impl PlantStage {
    pub fn from_progress(p: f64) -> Self {
        if p < 0.2       { PlantStage::Seed }
        else if p < 0.5  { PlantStage::Sprout }
        else if p < 0.85 { PlantStage::Growing }
        else             { PlantStage::Flowering }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Bed {
    pub id:               usize,
    pub moisture:         f64,
    pub progress:         f64,
    pub health:           f64,
    pub stage:            PlantStage,
    pub watering_active:  bool,
    pub fertilizer_boost: f64,
    pub fertilizer_age:   f64,
}

impl Bed {
    fn new(id: usize) -> Self {
        Bed {
            id,
            moisture:         0.5,
            progress:         0.0,
            health:           1.0,
            stage:            PlantStage::Seed,
            watering_active:  false,
            fertilizer_boost: 1.0,
            fertilizer_age:   0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct GrowthCycle {
    pub day_h:  f64,
    pub dark_h: f64,
    pub lux:    f64,
}

impl GrowthCycle {
    pub fn regular(&self) -> bool {
        (self.day_h + self.dark_h - 24.0).abs() < 0.01
    }
    pub fn growth_contribution(&self) -> f64 {
        if !self.regular() { return 0.0; }
        (self.lux * self.day_h) / 24.0
    }
}

// Kept for serialization compatibility
#[derive(Debug, Clone, Serialize)]
pub struct LightReading {
    pub intensity: f64,
    pub hours:     f64,
}

impl LightReading {
    pub fn is_sufficient(&self) -> bool {
        self.intensity > 0.3 && self.hours > 6.0
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct GardenState {
    pub beds:            Vec<Bed>,
    pub light:           LightReading,
    pub temperature:     f64,
    pub sun_size:        f64,
    pub season:          Season,
    pub tree:            systems::tree::Tree,
    pub elapsed_seconds: f64,
    pub can_x:           f64,
    pub can_y:           f64,
    pub can_angle:       f64,
    pub c1_day_h:  f64,  pub c1_dark_h: f64,  pub c1_lux: f64,
    pub c2_day_h:  f64,  pub c2_dark_h: f64,  pub c2_lux: f64,
    pub c3_day_h:  f64,  pub c3_dark_h: f64,  pub c3_lux: f64,
    pub c4_day_h:  f64,  pub c4_dark_h: f64,  pub c4_lux: f64,
    pub c5_day_h:  f64,  pub c5_dark_h: f64,  pub c5_lux: f64,
    pub cycles_completed: u32,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

impl GardenState {
    pub fn new() -> Self {
        GardenState {
            beds:            (0..NUM_BEDS).map(Bed::new).collect(),
            light:           LightReading { intensity: 0.6, hours: 9.0 },
            temperature:     22.0,
            sun_size:        0.55,
            season:          Season::Spring,
            tree:            systems::tree::Tree::new(),
            elapsed_seconds: 0.0,
            can_x:           100.0,
            can_y:           40.0,
            can_angle:       45.0,
            c1_day_h:  8.0,  c1_dark_h: 16.0,  c1_lux: 0.5,
            c2_day_h: 10.0,  c2_dark_h: 14.0,  c2_lux: 0.2,
            c3_day_h:  17.0,  c3_dark_h: 7.0,  c3_lux: 0.5,
            c4_day_h: 12.0,  c4_dark_h:  12.0,  c4_lux: 0.3,
            c5_day_h:  9.0,  c5_dark_h: 15.0,  c5_lux: 0.7,
            cycles_completed: 0,
        }
    }

    pub fn set_sun_size(&mut self, size: f64) {
        self.sun_size = size.clamp(0.0, 1.0);
    }

    pub fn tick(&mut self, dt: f64) {
        self.elapsed_seconds += dt;

        let solar_flux = self.sun_size * 40.0;
        self.temperature = solar_flux;

        let temp_above_base = (self.temperature - 20.0).max(0.0);
        let evap_rate = EVAP_BASE + temp_above_base * 0.0003;

        let cycles: [GrowthCycle; 5] = [
            GrowthCycle { day_h: self.c1_day_h, dark_h: self.c1_dark_h, lux: self.c1_lux },
            GrowthCycle { day_h: self.c2_day_h, dark_h: self.c2_dark_h, lux: self.c2_lux },
            GrowthCycle { day_h: self.c3_day_h, dark_h: self.c3_dark_h, lux: self.c3_lux },
            GrowthCycle { day_h: self.c4_day_h, dark_h: self.c4_dark_h, lux: self.c4_lux },
            GrowthCycle { day_h: self.c5_day_h, dark_h: self.c5_dark_h, lux: self.c5_lux },
        ];
        let valid_cycles = cycles.iter().filter(|c| c.regular()).count() as u32;
        self.cycles_completed = valid_cycles;

        let total_growth: f64 = cycles.iter().map(|c| c.growth_contribution()).sum();
        let light_ok = total_growth > 0.0;

        for bed in &mut self.beds {
            let water_reaches = bed.watering_active && self.can_angle > 20.0 && self.can_x < 180.0;
            let water_in = if water_reaches { WATER_FLOW * dt } else { 0.0 };
            bed.moisture = (bed.moisture - evap_rate * dt + water_in).clamp(0.0, 1.0);

            let too_hot  = self.temperature > TEMP_MAX;
            let too_cold = self.temperature < TEMP_MIN;
            let temp_ok  = !too_hot && !too_cold;

            let moisture_ok = bed.moisture >= MOISTURE_MIN && bed.moisture <= MOISTURE_MAX;
            let env_factor: f64 = match (moisture_ok, light_ok, temp_ok) {
                (true, true, true)  => 1.0,
                (true, true, false) => 0.4,
                (true, false, true) => 0.3,
                (false, true, true) => 0.2,
                _                  => 0.05,
            };

            let base_growth = 0.002 * dt * env_factor * bed.fertilizer_boost * (1.0 + total_growth);
            bed.progress = (bed.progress + base_growth).min(1.0);
            bed.stage = PlantStage::from_progress(bed.progress);

            let stress = if too_hot  { 0.001 * dt * (self.temperature - TEMP_MAX) }
                         else if too_cold { 0.001 * dt * (TEMP_MIN - self.temperature) }
                         else { 0.0 };
            let drought_stress = if bed.moisture < 0.1 { 0.002 * dt } else { 0.0 };
            bed.health = (bed.health - stress - drought_stress).clamp(0.0, 1.0);

            if bed.fertilizer_boost > 1.0 {
                bed.fertilizer_age += dt;
            }
        }

        let avg_moisture = self.beds.iter().map(|b| b.moisture).sum::<f64>() / self.beds.len() as f64;
        self.tree.tick(dt, avg_moisture, light_ok, self.temperature);
    }

    pub fn set_temperature_f(&mut self, fahrenheit: f64) {
        let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
        self.set_sun_size(celsius / 40.0);
    }

    pub fn set_watering(&mut self, bed_id: usize, active: bool) {
        if let Some(bed) = self.beds.get_mut(bed_id) {
            bed.watering_active = active;
        }
    }

    pub fn bed_status(&self, bed_id: usize) -> &'static str {
        let bed = &self.beds[bed_id];
        if self.temperature > TEMP_MAX { return "Overheated"; }
        if self.temperature < TEMP_MIN { return "Too Cold"; }
        if bed.moisture < MOISTURE_MIN { return "Thirsty"; }
        if bed.moisture > MOISTURE_MAX { return "Waterlogged"; }
        if !self.light.is_sufficient() { return "Needs Light"; }
        if bed.health < 0.3            { return "Struggling"; }
        "Thriving"
    }

    pub fn set_season(&mut self, season: Season) {
        self.season = season;
    }

    pub fn apply_fertilizer(&mut self, _bed_id: usize, _ftype: systems::fertilizer::FertilizerType) {
        unimplemented!()
    }

    pub fn scan_pests(&self) -> Vec<systems::pests::PestAlert> {
        unimplemented!()
    }

    pub fn configure_auto_water(&mut self, _bed_id: usize, _threshold: f64, _cooldown_secs: f64) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_garden_defaults() {
        let g = GardenState::new();
        assert_eq!(g.beds.len(), NUM_BEDS);
        assert_eq!(g.temperature, 22.0);
        assert!(g.beds.iter().all(|b| b.moisture == 0.5));
    }

    #[test]
    fn test_plant_stage_from_progress() {
        assert_eq!(PlantStage::from_progress(0.0), PlantStage::Seed);
        assert_eq!(PlantStage::from_progress(0.3), PlantStage::Sprout);
        assert_eq!(PlantStage::from_progress(0.6), PlantStage::Growing);
        assert_eq!(PlantStage::from_progress(0.9), PlantStage::Flowering);
    }
}
