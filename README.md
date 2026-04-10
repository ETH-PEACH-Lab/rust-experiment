# Pixel Garden

A Rust garden simulator. You have two tasks, each 20 minutes.

---

## Getting started

**Launch the dashboard** (keep this running throughout):
```bash
cargo run -p dashboard
```
A small window appears — move it to the right edge of your screen. Use the rest of the screen for your editor.

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

## Project layout

```
sim/
  src/
    garden.rs          main simulation — Task 1
    systems/
      tree.rs          Task 2
      fertilizer.rs
      pests.rs
  tests/
    task1_tests.rs
    task2_tests.rs

dashboard/             visualisation — no need to edit this
```
