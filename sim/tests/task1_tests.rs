use garden_sim::garden::{GardenState, MOISTURE_MIN};

#[test]
fn test_watering_increases_moisture() {
    println!("XXX");
    let mut g = GardenState::new();
    g.temperature = 20.0;
    g.beds[0].moisture = 0.2;
    g.set_watering(0, true);
    let before = g.beds[0].moisture;

    for _ in 0..100 { g.tick(0.1); }

    assert!(
        g.beds[0].moisture > before,
        "moisture: {:.3} → {:.3} with watering active — expected increase",
        before, g.beds[0].moisture
    );
}

#[test]
fn test_evaporation_dries_soil_without_watering() {
    let mut g = GardenState::new();
    g.temperature = 25.0;
    g.beds[0].moisture = 0.6;
    g.set_watering(0, false);
    let before = g.beds[0].moisture;

    for _ in 0..100 { g.tick(0.1); }

    assert!(
        g.beds[0].moisture < before,
        "moisture: {:.3} → {:.3} with no watering — expected decrease",
        before, g.beds[0].moisture
    );
}

#[test]
fn test_watering_can_recover_dry_bed() {
    let mut g = GardenState::new();
    g.temperature = 20.0;
    g.beds[0].moisture = 0.05;
    g.set_watering(0, true);

    for _ in 0..600 { g.tick(0.1); }

    assert!(
        g.beds[0].moisture >= MOISTURE_MIN,
        "moisture after 60 s of watering: {:.3} — expected >= {:.2}",
        g.beds[0].moisture, MOISTURE_MIN
    );
}

#[test]
fn test_day_night_schedule_is_consistent() {
    let g = GardenState::new();
    let pairs = [
        (g.c1_day_h, g.c1_dark_h, 1u8),
        (g.c2_day_h, g.c2_dark_h, 2),
        (g.c3_day_h, g.c3_dark_h, 3),
        (g.c4_day_h, g.c4_dark_h, 4),
        (g.c5_day_h, g.c5_dark_h, 5),
    ];
    for (day, dark, n) in pairs {
        assert!(
            (day + dark - 24.0).abs() < 0.01,
            "cycle {n} schedule is inconsistent: {day} + {dark} = {}",
            day + dark
        );
    }
}

#[test]
fn test_plant_reaches_full_growth() {
    let mut g = GardenState::new();
    g.temperature      = 22.0;
    g.beds[0].moisture = 0.5;

    for _ in 0..6000 { g.tick(0.1); }

    assert_eq!(g.cycles_completed, 5,
        "only {}/5 growth cycles completed",
        g.cycles_completed);
    assert!(
        g.beds[0].progress > 0.25,
        "progress {:.3} — not enough growth after 5 cycles",
        g.beds[0].progress
    );
}

#[test]
fn test_greenhouse_temperature_is_reasonable() {
    let mut g = GardenState::new();
    g.sun_size = 0.55;
    g.tick(0.1);
    assert!(
        (g.temperature - 22.0).abs() < 3.0,
        "sun_size=0.55 produced temperature {:.2} — expected ~22°C",
        g.temperature
    );
}

#[test]
fn test_maximum_sun_does_not_overheat() {
    let mut g = GardenState::new();
    g.sun_size = 1.0;
    g.tick(0.1);
    assert!(
        g.temperature < 80.0,
        "sun_size=1.0 produced temperature {:.2} — expected a physically plausible value",
        g.temperature
    );
}

#[test]
fn test_beds_not_overheated_at_normal_sun() {
    let mut g = GardenState::new();
    g.sun_size = 0.55;
    g.tick(0.1);
    for bed in &mut g.beds { bed.moisture = 0.5; }
    let statuses: Vec<&str> = (0..4).map(|i| g.bed_status(i)).collect();
    assert!(
        statuses.iter().all(|s| *s != "Overheated"),
        "beds show {:?} at normal sun — none should be Overheated",
        statuses
    );
}
