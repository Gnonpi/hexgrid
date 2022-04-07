use std::cmp::Eq;
use std::collections::HashMap;

struct HexGrid {
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

    fn get_cell(&self, x: TMetric, y: TMetric) -> Option<HexCell> {
        self.cells.get(&HexCoord { x, y }).cloned()
    }

    fn add_cell(&mut self, x: TMetric, y: TMetric, data: TCellData) -> Option<HexCell> {
        let coord = HexCoord { x, y };
        self.cells.insert(coord, HexCell { coord, data })
    }

    fn remove_cell(&mut self, x: TMetric, y: TMetric) -> Option<HexCell> {
        self.cells.remove(&HexCoord { x, y })
    }
}

#[derive(PartialEq, Clone, Debug)]
struct HexCell {
    coord: HexCoord,
    data: TCellData,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct HexCoord {
    x: TMetric,
    y: TMetric,
}

type TMetric = usize;
type TCellData = String;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_can_add_and_remove_cells() {
        // init grid
        let mut grid = HexGrid::new();

        // add one
        let first_data = String::from("first");
        let first_add = grid.add_cell(1, 1, first_data.clone());
        assert!(first_add.is_none());
        assert_eq!(grid.count_cells(), 1);
        let first_get = grid.get_cell(1, 1);
        assert!(first_get.is_some());
        assert_eq!(first_get.clone().unwrap().data, first_data);

        // re-add -- update
        let second_data = String::from("second");
        let second_cell = HexCell {
            coord: HexCoord { x: 1, y: 1 },
            data: second_data.clone(),
        };
        let second_add = grid.add_cell(1, 1, second_data.clone());
        assert!(second_add.is_some());
        assert_eq!(second_add.unwrap(), first_get.unwrap());
        assert_eq!(grid.count_cells(), 1);
        let second_get = grid.get_cell(1, 1);
        assert!(second_get.is_some());
        assert_eq!(second_get.unwrap(), second_cell);

        // remove
        let first_remove = grid.remove_cell(1, 1);
        assert!(first_remove.is_some());
        assert_eq!(first_remove.unwrap().data, second_data);
        let third_get = grid.get_cell(1, 1);
        assert!(third_get.is_none());

        // re-remove
        let second_remove = grid.remove_cell(1, 1);
        assert!(second_remove.is_none());
    }
}
