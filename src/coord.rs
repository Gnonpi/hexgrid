use crate::{check_snapping, HexGridError, TMetric, WalkDirection};

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub struct HexCoord {
    pub x: TMetric,
    pub y: TMetric,
}

impl HexCoord {
    pub fn new(x: TMetric, y: TMetric) -> Result<Self, HexGridError> {
        if !check_snapping(x, y) {
            return Err(HexGridError::NotSnappedToGrid(HexCoord { x, y }));
        }
        Ok(HexCoord { x, y })
    }

    pub(crate) fn walk(&self, direction: &WalkDirection, distance: usize) -> Self {
        let idistance = distance as TMetric;
        match direction {
            WalkDirection::Up => HexCoord {
                x: self.x,
                y: self.y + idistance * 2,
            },
            WalkDirection::UpRight => HexCoord {
                x: self.x + idistance,
                y: self.y + idistance,
            },
            WalkDirection::DownRight => HexCoord {
                x: self.x + idistance,
                y: self.y - idistance,
            },
            WalkDirection::Down => HexCoord {
                x: self.x,
                y: self.y - idistance * 2,
            },
            WalkDirection::DownLeft => HexCoord {
                x: self.x - idistance,
                y: self.y - idistance,
            },
            WalkDirection::UpLeft => HexCoord {
                x: self.x - idistance,
                y: self.y + idistance,
            },
        }
    }
}
