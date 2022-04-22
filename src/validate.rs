use crate::TMetric;

pub fn check_snapping(x: TMetric, y: TMetric) -> bool {
    // to snap on the grid, the sum of the coords must be even
    (x + y) % 2 == 0
}
