use crate::adjacent::{adjacent_coords, immediate_adjacent_coords};
use crate::{check_snapping, HexCell, HexCoord, HexGridError, TCellData, TMetric};
use std::collections::HashMap;

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
