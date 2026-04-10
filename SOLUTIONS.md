# Pixel Garden — Solutions

Keep this file private. Do not share with participants.

---

## Task 1 — Bug Fixes

### Bug 1: Watering adds to evaporation instead of opposing it

**File:** `sim/src/garden.rs`, inside `tick()`.

**Symptom:** Moisture drops faster with watering ON than OFF.

**Root cause:** `water_in` is added inside the evaporation term, making watering increase the drain:
```rust
// BUGGY — water_in is subtracted
bed.moisture = (bed.moisture - (evap_rate * dt + water_in)).clamp(0.0, 1.0);
```

**Fix:** Separate the addition and subtraction:
```rust
// FIXED
let water_in = if bed.watering_active { WATER_FLOW * dt } else { 0.0 };
bed.moisture = (bed.moisture - evap_rate * dt + water_in).clamp(0.0, 1.0);
```

**Passing tests:** `test_watering_increases_moisture`, `test_evaporation_dries_soil_without_watering`, `test_watering_can_recover_dry_bed`

---

### Bug 2: LightReading defaults don't satisfy is_sufficient(), so nights never end

**File:** `sim/src/garden.rs`, `GardenState::new()` — the three values in `LightReading { ... }`.

**Symptom (dashboard):** Stage 2 goes to night and stays there through all 5 cycles. The moon never leaves.

**Root cause:** The day/night cycle logic in `tick()` is correct — night ends only when `is_sufficient()` returns true. But the default values produce `photo * recovery = 8.0`, which is below the threshold of `15.0`, so `is_sufficient()` always returns false and `is_day` stays false permanently.

The formula:
```rust
let photo    = self.intensity * self.hours;
let recovery = self.rest_hours / (self.intensity + 0.5);
photo * recovery > 15.0  // must be true for dawn to occur
```

**Fix:** Change the three values in `new()` so the formula is satisfied. One working set:
```rust
light: LightReading { intensity: 0.7, hours: 9.0, rest_hours: 7.0 },
// photo = 0.7 * 9.0 = 6.3
// recovery = 7.0 / (0.7 + 0.5) = 5.83
// product = 36.7 > 15.0  ✓
```
Many combinations work — the participant must experiment since the denominator `(intensity + 0.5)` makes it non-linear.

**Passing tests:** `test_default_light_is_sufficient`, `test_day_night_cycle_returns_to_day`

---

### Bug 3: Temperature derived from sun_size with a hidden Kelvin offset in tick()

**File:** `sim/src/garden.rs`, inside `tick()`.

**Symptom:** All beds show "Overheated" even on a mild day. The simulation sun grows
oversized and the thermometer immediately shows ~295°. Plants never grow because
`temp_ok` is always false. The bug is not in `set_temperature_f()` — it is in the
`solar_flux` formula called every tick.

**Root cause:** `tick()` derives temperature from `sun_size` with `+ 273.15` disguised
as a "Stefan-Boltzmann baseline":
```rust
// BUGGY — 273.15 pushes Celsius into Kelvin territory
let solar_flux = self.sun_size * 40.0;
self.temperature = solar_flux + 273.15; // "baseline thermal offset"
```
At `sun_size = 0.55`: `0.55 × 40 + 273.15 = 295.15` instead of `22.0`.

**Fix:** Remove the offset — `solar_flux` is already in Celsius:
```rust
// FIXED
let solar_flux = self.sun_size * 40.0;
self.temperature = solar_flux;
```

**Passing tests:** `test_normal_sun_gives_correct_temperature`, `test_full_sun_temperature_is_celsius`, `test_sun_size_gives_healthy_temp_status`

---

## Task 2 — Feature Implementations

### Feature 1: Fertilizer system

**`sim/src/systems/fertilizer.rs`:**
```rust
impl FertilizerType {
    pub fn boost_multiplier(&self) -> f64 {
        match self {
            FertilizerType::Nitrogen   => 1.8,
            FertilizerType::Phosphorus => 1.5,
            FertilizerType::Balanced   => 2.0,
        }
    }
}

pub fn seasonal_rate(season: &crate::garden::Season) -> f64 {
    match season {
        Season::Spring => 1.5,
        Season::Summer => 1.0,
        Season::Autumn => 0.5,
        Season::Winter => 0.1,
    }
}

pub fn decayed_boost(initial: f64, age_secs: f64) -> f64 {
    let half_lives = age_secs / 300.0;
    let decayed = 1.0 + (initial - 1.0) * 0.5_f64.powf(half_lives);
    decayed.max(1.0)
}
```

**`sim/src/garden.rs`** — `apply_fertilizer()`:
```rust
pub fn apply_fertilizer(&mut self, bed_id: usize, ftype: systems::fertilizer::FertilizerType) {
    if let Some(bed) = self.beds.get_mut(bed_id) {
        bed.fertilizer_boost = ftype.boost_multiplier();
        bed.fertilizer_age = 0.0;
    }
}
```

**`sim/src/garden.rs`** — in `tick()`, replace the fertilizer decay placeholder:
```rust
if bed.fertilizer_boost > 1.0 {
    bed.fertilizer_age += dt;
    bed.fertilizer_boost = systems::fertilizer::decayed_boost(
        bed.fertilizer_boost,
        bed.fertilizer_age,
    );
}
```

Also add seasonal rate to `base_growth` in `tick()`:
```rust
let seasonal = systems::fertilizer::seasonal_rate(&self.season);
let base_growth = 0.002 * dt * env_factor * bed.fertilizer_boost * seasonal;
```

---

### Feature 2: Seasonal growth modifier

Already covered above in Feature 1 (`seasonal_rate()` + integration in `tick()`).

---

### Feature 3: Pest detection

**`sim/src/garden.rs`** — `scan_pests()`:
```rust
pub fn scan_pests(&self) -> Vec<systems::pests::PestAlert> {
    use systems::pests::{PestAlert, PestType};
    self.beds.iter().filter_map(|bed| {
        if bed.health >= 0.4 { return None; }
        let pest_type = if bed.moisture < 0.15 {
            PestType::SpiderMites
        } else if self.temperature > 35.0 {
            PestType::Aphids
        } else if self.temperature < 10.0 {
            PestType::Fungus
        } else {
            PestType::Whitefly
        };
        Some(PestAlert {
            bed_id:    bed.id,
            severity:  1.0 - bed.health,
            pest_type,
        })
    }).collect()
}
```

---

### Feature 4: Auto-watering scheduler

**`sim/src/systems/pests.rs`:**
```rust
pub struct AutoWater {
    pub threshold:     f64,
    pub cooldown_secs: f64,
    last_activated:    f64,
}

impl AutoWater {
    pub fn new(threshold: f64, cooldown_secs: f64) -> Self {
        AutoWater { threshold, cooldown_secs, last_activated: -9999.0 }
    }

    pub fn should_water(&self, moisture: f64, elapsed: f64) -> bool {
        moisture < self.threshold
            && (elapsed - self.last_activated) >= self.cooldown_secs
    }

    pub fn record_activation(&mut self, elapsed: f64) {
        self.last_activated = elapsed;
    }
}
```

---

## Verify all tests pass

```bash
cargo test -p garden-sim
```

Expected output: all tests pass, no failures.
