use garden_sim::systems::tree::Tree;

#[test]
fn test_feature_a_has_name() {
    let tree = Tree::new();
    assert!(!tree.feature_a.name.is_empty(), "feature_a has no name");
}

#[test]
fn test_feature_b_has_name() {
    let tree = Tree::new();
    assert!(!tree.feature_b.name.is_empty(), "feature_b has no name");
}

#[test]
fn test_feature_c_has_name() {
    let tree = Tree::new();
    assert!(!tree.feature_c.name.is_empty(), "feature_c has no name");
}

#[test]
fn test_feature_a_responds_to_conditions() {
    let mut tree = Tree::new();
    let initial = tree.feature_a.level;
    for _ in 0..600 { tree.tick(0.1, 0.6, true, 22.0); }
    assert!(
        tree.feature_a.level != initial || tree.feature_a.active,
        "feature '{}' level unchanged after 60 s (was {:.4})",
        tree.feature_a.name, initial
    );
}

#[test]
fn test_feature_b_responds_to_conditions() {
    let mut tree = Tree::new();
    let initial = tree.feature_b.level;
    for _ in 0..600 { tree.tick(0.1, 0.6, true, 22.0); }
    assert!(
        tree.feature_b.level != initial || tree.feature_b.active,
        "feature '{}' level unchanged after 60 s (was {:.4})",
        tree.feature_b.name, initial
    );
}

#[test]
fn test_feature_c_responds_to_conditions() {
    let mut tree = Tree::new();
    let initial = tree.feature_c.level;
    for _ in 0..600 { tree.tick(0.1, 0.6, true, 22.0); }
    assert!(
        tree.feature_c.level != initial || tree.feature_c.active,
        "feature '{}' level unchanged after 60 s (was {:.4})",
        tree.feature_c.name, initial
    );
}

#[test]
fn test_feature_levels_stay_in_range() {
    let mut tree = Tree::new();
    for _ in 0..3000 { tree.tick(0.1, 1.0, true, 35.0); }
    for (label, f) in [("a", &tree.feature_a), ("b", &tree.feature_b), ("c", &tree.feature_c)] {
        assert!(
            (0.0..=1.0).contains(&f.level),
            "feature_{} ('{}') level = {:.4} — must stay 0.0–1.0",
            label, f.name, f.level
        );
    }
}
