use crate::{HexCoord, WalkDirection};

pub(crate) fn immediate_adjacent_coords(center: &HexCoord) -> [HexCoord; 6] {
    [
        // up
        HexCoord::new(center.x, center.y + 2).unwrap(),
        // up right
        HexCoord::new(center.x + 1, center.y + 1).unwrap(),
        // down right
        HexCoord::new(center.x + 1, center.y - 1).unwrap(),
        // down
        HexCoord::new(center.x, center.y - 2).unwrap(),
        // down left
        HexCoord::new(center.x - 1, center.y - 1).unwrap(),
        // up left
        HexCoord::new(center.x - 1, center.y + 1).unwrap(),
    ]
}

pub(crate) fn adjacent_coords(center: &HexCoord, resolution: usize) -> Vec<HexCoord> {
    let all_d = WalkDirection::all_directions();
    all_d.iter().map(|d| center.walk(d, resolution)).collect()
}
