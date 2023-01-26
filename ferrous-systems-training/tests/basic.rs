use ferrous_systems_training::testing::{Direction, is_north};

#[test]
fn test_is_north() {
    assert!(is_north(Direction::North));
    assert!(!is_north(Direction::South));
}