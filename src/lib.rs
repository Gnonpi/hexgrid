use std::cmp::Eq;
use std::collections::HashMap;
use thiserror::Error;

type TMetric = isize;
type TCellData = String;

#[derive(Error, Debug)]
pub enum HexGridError {
    #[error("Cell not snapped to grid")]
    NotSnappedToGrid(HexCoord),
    #[error("Resolution was 0")]
    ResolutionSetToZero,
}

fn check_snapping(x: TMetric, y: TMetric) -> bool {
    // to snap on the grid, the sum of the coords must be even
    (x + y) % 2 == 0
}

fn immediate_adjacent_coords(center: &HexCoord) -> [HexCoord; 6] {
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

fn adjacent_coords(center: &HexCoord, resolution: usize) -> Vec<HexCoord> {
    let all_d = WalkDirection::all_directions();
    all_d.iter().map(|d| center.walk(d, resolution)).collect()
}

pub struct HexGrid {
    cells: HashMap<HexCoord, HexCell>,
}

impl HexGrid {
    fn new() -> Self {
        HexGrid {
            cells: HashMap::<HexCoord, HexCell>::new(),
        }
    }

    fn count_cells(&self) -> usize {
        self.cells.len()
    }

    fn get_cell_at(&self, x: TMetric, y: TMetric) -> Result<Option<HexCell>, HexGridError> {
        if !check_snapping(x, y) {
            return Err(HexGridError::NotSnappedToGrid(HexCoord { x, y }));
        }
        Ok(self.cells.get(&HexCoord { x, y }).cloned())
    }

    fn add_cell_at(
        &mut self,
        x: TMetric,
        y: TMetric,
        data: TCellData,
    ) -> Result<Option<HexCell>, HexGridError> {
        if !check_snapping(x, y) {
            return Err(HexGridError::NotSnappedToGrid(HexCoord { x, y }));
        }
        let coord = HexCoord { x, y };
        Ok(self.cells.insert(coord, HexCell { coord, data }))
    }

    fn remove_cell_at(&mut self, x: TMetric, y: TMetric) -> Result<Option<HexCell>, HexGridError> {
        if !check_snapping(x, y) {
            return Err(HexGridError::NotSnappedToGrid(HexCoord { x, y }));
        }
        Ok(self.cells.remove(&HexCoord { x, y }))
    }

    fn get_cell(&self, coord: &HexCoord) -> Option<HexCell> {
        self.cells.get(coord).cloned()
    }

    fn add_cell(&mut self, coord: &HexCoord, data: TCellData) -> Option<HexCell> {
        self.cells.insert(
            *coord,
            HexCell {
                coord: *coord,
                data,
            },
        )
    }

    fn remove_cell(&mut self, coord: &HexCoord) -> Option<HexCell> {
        self.cells.remove(coord)
    }

    fn get_adjacent(&self, center: &HexCoord) -> Vec<HexCell> {
        let adj_coords = immediate_adjacent_coords(center);
        let mut bag = vec![];
        for coord in adj_coords.iter() {
            if let Some(c) = self.get_cell(coord) {
                bag.push(c)
            }
        }
        bag
    }

    fn get_in_circle(
        &self,
        center: &HexCoord,
        resolution: usize,
    ) -> Result<Vec<HexCell>, HexGridError> {
        match resolution {
            0 => Err(HexGridError::ResolutionSetToZero),
            1 => Ok(self.get_adjacent(center)),
            _ => {
                let mut bag = vec![];
                for r in 1..resolution {
                    let adjs = adjacent_coords(center, r)
                        .iter()
                        .filter_map(|c| self.get_cell(c))
                        .collect();
                    bag = [bag, adjs].concat();
                }
                Ok(bag)
            }
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct HexCell {
    coord: HexCoord,
    data: TCellData,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub struct HexCoord {
    x: TMetric,
    y: TMetric,
}

impl HexCoord {
    fn new(x: TMetric, y: TMetric) -> Result<Self, HexGridError> {
        if !check_snapping(x, y) {
            return Err(HexGridError::NotSnappedToGrid(HexCoord { x, y }));
        }
        Ok(HexCoord { x, y })
    }

    fn walk(&self, direction: &WalkDirection, distance: usize) -> Self {
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

#[derive(Copy, Clone, Debug)]
enum WalkDirection {
    Up,
    UpRight,
    DownRight,
    Down,
    DownLeft,
    UpLeft,
}

impl WalkDirection {
    fn all_directions() -> [WalkDirection; 6] {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn init_log() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn init_grid() -> HexGrid {
        let mut grid = HexGrid::new();
        let mut cnt = 0;
        for i in -10..10 {
            for j in -10..10 {
                if check_snapping(i, j) {
                    grid.add_cell_at(i, j, cnt.to_string());
                    cnt += 1;
                }
            }
        }
        grid
    }

    #[test]
    fn it_can_add_and_remove_cells() {
        // init grid
        let mut grid = HexGrid::new();

        // add one
        let first_data = String::from("first");
        let first_add = grid.add_cell_at(1, 1, first_data.clone()).unwrap();
        assert!(first_add.is_none());
        assert_eq!(grid.count_cells(), 1);
        let first_get = grid.get_cell_at(1, 1).unwrap();
        assert!(first_get.is_some());
        assert_eq!(first_get.clone().unwrap().data, first_data);

        // re-add -- update
        let second_data = String::from("second");
        let second_cell = HexCell {
            coord: HexCoord { x: 1, y: 1 },
            data: second_data.clone(),
        };
        let second_add = grid.add_cell_at(1, 1, second_data.clone()).unwrap();
        assert!(second_add.is_some());
        assert_eq!(second_add.unwrap(), first_get.unwrap());
        assert_eq!(grid.count_cells(), 1);
        let second_get = grid.get_cell_at(1, 1).unwrap();
        assert!(second_get.is_some());
        assert_eq!(second_get.unwrap(), second_cell);

        // remove
        let first_remove = grid.remove_cell_at(1, 1).unwrap();
        assert!(first_remove.is_some());
        assert_eq!(first_remove.unwrap().data, second_data);
        let third_get = grid.get_cell_at(1, 1).unwrap();
        assert!(third_get.is_none());

        // re-remove
        let second_remove = grid.remove_cell_at(1, 1).unwrap();
        assert!(second_remove.is_none());
    }

    #[test]
    fn it_refuse_overlapping_cells() {
        let mut grid = HexGrid::new();
        let a = grid.add_cell_at(0, 0, "zero".to_string());
        assert!(a.is_ok());
        let b = grid.add_cell_at(0, 1, "too close".to_string());
        assert!(b.is_err());
    }

    #[test]
    fn it_can_find_adjacent_cells() {
        let mut grid = init_grid();
        let adjs = grid.get_adjacent(&HexCoord { x: 0, y: 0 });
        assert_eq!(adjs.len(), 6);
        let adjs = grid.get_adjacent(&HexCoord { x: 20, y: 20 });
        assert_eq!(adjs.len(), 0);
    }
}
