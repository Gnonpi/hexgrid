use crate::HexCoord;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HexGridError {
    #[error("Cell not snapped to grid")]
    NotSnappedToGrid(HexCoord),
    #[error("Resolution was 0")]
    ResolutionSetToZero,
}
