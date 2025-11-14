use crate::{bit_vec::BitVec, space_time_id_set::SpaceTimeIdSet};

impl SpaceTimeIdSet {
    pub fn divison(target: &BitVec, divison: Vec<BitVec>) {
        let mut result = vec![];

        for ele in &mut divison.clone() {
            let splited = Self::split_dimension(target, ele);
            result.extend(splited);
        }
    }
}
