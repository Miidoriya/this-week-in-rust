///```rust
/// use ferrous_systems_training::testing::Direction; // tests ran with `cargo test --doc`
/// let way_home = Direction::North;
/// ```
pub enum Direction { North, South, East, West } 

pub fn is_north(dir: Direction) -> bool {
    match dir {
        Direction::North => true,
        _ => false,
    }
}

#[test]
fn test_is_north() {
    assert!(is_north(Direction::North));
    assert!(!is_north(Direction::South));
}

// module example of testing is_north within the same file
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_north() {
        assert!(is_north(Direction::North));
        assert!(!is_north(Direction::South));
    }
}