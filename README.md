# Pixel Garden

A Rust garden simulator. You have three tasks, each 20 minutes.

---

## Getting started

**Launch the dashboard** (already launched, keep this running throughout):
```bash
cargo run -p dashboard
```

**Run tests** (your main feedback loop):
```bash
cargo test -p garden-sim --test task1_tests
cargo test -p garden-sim --test task2_tests
```

Tests tell you what is passing and what is not. The dashboard visualises the simulation in real time.

---

## Quick Rust reference

```rust
// Compile check only (fast)
cargo check

// Run all tests
cargo test -p garden-sim

// Run a single test by name
cargo test -p garden-sim -- test_name_here

// See test output even when passing
cargo test -p garden-sim -- --nocapture
```

Types you will encounter: `f64` (floating point), `bool`, `String`, `Vec<T>` (list), `Option<T>` (nullable), `&mut self` (mutable method). The compiler error messages are detailed — read them carefully.

---

## Task 1 — Fix the bugs (20 min)

The simulation in `sim/src/garden.rs` has three bugs. The garden is not behaving correctly — watch the dashboard and run the tests to find them.

All Task 1 tests should pass when you are done:
```bash
cargo test -p garden-sim --test task1_tests
```

**Where to look:** `sim/src/garden.rs`

---

## Task 2 — Grow the tree (20 min)

The garden has a bare tree. Add three features to bring it to life.

Open `sim/src/systems/tree.rs`. You will find a `Tree` struct with three empty `Feature` slots and a `tick()` method that currently does nothing. Fill them in however you like — the only requirement is that each feature responds to the garden's conditions over time.

All Task 2 tests should pass when you are done:
```bash
cargo test -p garden-sim --test task2_tests
```

**Where to work:** `sim/src/systems/tree.rs`

---

## Task 3 — Plant Lab (20 min)

Design your own flowers. Open `sim/src/systems/garden_lab.rs` — the file is full of comments that explain everything.

You will define at least two plant variants. Each one specifies:
- The **conditions** it needs to grow (moisture, fertilizer, temperature ranges)
- How it **looks**: petal color, tip color, petal count, flower size, center color, leaf color, leaf size, leaf position on the stem, and stem color

A worked example (a Sunflower) is already filled in. Pay attention to the code structures to get an idea of how to implement your vision for the flowers of your dreams (or nightmares)!

To test: open the **Task 3 — Plant Lab** tab in the dashboard, move the sliders to match your conditions, and try to drag the watering can over to the dry part of the soil. If you managed to implement the watering can logic to work correctly, then it will begin to grow the flower. If the dry spot has been watered but nothing grows, your slider values are outside your defined ranges, so adjust either the sliders or your conditions until it blooms.

A good working implementation should lead to correct watering can interactive usage as well as three or more flowers that grow in different conditions.

**Where to work:** `sim/src/systems/garden_lab.rs`

---

## Project layout

```
sim/
  src/
    garden.rs          main simulation — Task 1
    systems/
      tree.rs          Task 2
      garden_lab.rs    Task 3
      fertilizer.rs
      pests.rs
  tests/
    task1_tests.rs
    task2_tests.rs

dashboard/             visualisation — no need to edit this
```
