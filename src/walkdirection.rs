#[derive(Copy, Clone, Debug)]
pub enum WalkDirection {
    Up,
    UpRight,
    DownRight,
    Down,
    DownLeft,
    UpLeft,
}

impl WalkDirection {
    pub fn all_directions() -> [WalkDirection; 6] {
        [
            WalkDirection::Up,
            WalkDirection::UpRight,
            WalkDirection::DownRight,
            WalkDirection::Down,
            WalkDirection::DownLeft,
            WalkDirection::UpLeft,
        ]
    }
}
