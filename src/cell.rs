use crate::{HexCoord, TCellData};

#[derive(PartialEq, Clone, Debug)]
pub struct HexCell {
    pub coord: HexCoord,
    pub data: TCellData,
}
