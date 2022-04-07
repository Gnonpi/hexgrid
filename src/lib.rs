use std::cmp::Eq;
use std::collections::HashMap;
use thiserror::Error;

type TMetric = isize;
type TCellData = String;

#[derive(Error, Debug)]
pub enum HexGridError {
    #[error("Cell not snapped to grid")]
    NotSnappedToGrid(HexCoord),
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

    fn get_cell(&self, x: TMetric, y: TMetric) -> Result<Option<HexCell>, HexGridError> {
        if !HexGrid::check_snapping(x, y) {
            return Err(HexGridError::NotSnappedToGrid(HexCoord { x, y }));
        }
        Ok(self.cells.get(&HexCoord { x, y }).cloned())
    }

    fn add_cell(
        &mut self,
        x: TMetric,
        y: TMetric,
        data: TCellData,
    ) -> Result<Option<HexCell>, HexGridError> {
        if !HexGrid::check_snapping(x, y) {
            return Err(HexGridError::NotSnappedToGrid(HexCoord { x, y }));
        }
        let coord = HexCoord { x, y };
        Ok(self.cells.insert(coord, HexCell { coord, data }))
    }

    fn remove_cell(&mut self, x: TMetric, y: TMetric) -> Result<Option<HexCell>, HexGridError> {
        if !HexGrid::check_snapping(x, y) {
            return Err(HexGridError::NotSnappedToGrid(HexCoord { x, y }));
        }
        Ok(self.cells.remove(&HexCoord { x, y }))
    }

    fn check_snapping(x: TMetric, y: TMetric) -> bool {
        // to snap on the grid, the sum of the coords must be even
        (x + y) % 2 == 0
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

#[cfg(test)]
mod tests {
    use super::*;

    fn init_grid() {}

    #[test]
    fn it_can_add_and_remove_cells() {
        // init grid
        let mut grid = HexGrid::new();

        // add one
        let first_data = String::from("first");
        let first_add = grid.add_cell(1, 1, first_data.clone()).unwrap();
        assert!(first_add.is_none());
        assert_eq!(grid.count_cells(), 1);
        let first_get = grid.get_cell(1, 1).unwrap();
        assert!(first_get.is_some());
        assert_eq!(first_get.clone().unwrap().data, first_data);

        // re-add -- update
        let second_data = String::from("second");
        let second_cell = HexCell {
            coord: HexCoord { x: 1, y: 1 },
            data: second_data.clone(),
        };
        let second_add = grid.add_cell(1, 1, second_data.clone()).unwrap();
        assert!(second_add.is_some());
        assert_eq!(second_add.unwrap(), first_get.unwrap());
        assert_eq!(grid.count_cells(), 1);
        let second_get = grid.get_cell(1, 1).unwrap();
        assert!(second_get.is_some());
        assert_eq!(second_get.unwrap(), second_cell);

        // remove
        let first_remove = grid.remove_cell(1, 1).unwrap();
        assert!(first_remove.is_some());
        assert_eq!(first_remove.unwrap().data, second_data);
        let third_get = grid.get_cell(1, 1).unwrap();
        assert!(third_get.is_none());

        // re-remove
        let second_remove = grid.remove_cell(1, 1).unwrap();
        assert!(second_remove.is_none());
    }

    #[test]
    fn it_refuse_overlapping_cells() {
        let mut grid = HexGrid::new();
        let a = grid.add_cell(0, 0, "zero".to_string());
        assert!(a.is_ok());
        let b = grid.add_cell(0, 1, "too close".to_string());
        assert!(b.is_err());
    }

    #[test]
    fn it_can_find_adjacent_cells() {}
}
