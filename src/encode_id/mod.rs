use crate::bit_vec::BitVec;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct EncodeID {
    pub f: Vec<BitVec>,
    pub x: Vec<BitVec>,
    pub y: Vec<BitVec>,
}
