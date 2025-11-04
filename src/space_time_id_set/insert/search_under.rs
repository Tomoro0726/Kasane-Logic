use std::collections::BTreeMap;

use crate::{
    space_time_id_set::{LayerInfo, SpaceTimeIdSet},
    r#type::bit_vec::{self, BitVec},
};
impl SpaceTimeIdSet {
    pub fn search_under(btree: &BTreeMap<BitVec, LayerInfo>, encoded: &BitVec) -> usize {
        match btree.get(&encoded) {
            Some(v) => v.count,
            None => 0,
        }
    }
}
