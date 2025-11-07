use std::collections::{BTreeMap, HashSet};

use crate::{
    space_time_id_set::{
        Index, LayerInfo, SpaceTimeIdSet, insert::insert_main_dim::MainDimensionSelect,
    },
    r#type::bit_vec::BitVec,
};

impl SpaceTimeIdSet {
    ///与えられた次元において、上位の範囲を収集する
    pub fn collect_top(
        &self,
        main_bit: &BitVec,
        main_dim_select: &MainDimensionSelect,
    ) -> HashSet<Index> {
        let main_top;

        let dims = self.select_dimensions(&main_dim_select);

        // 上位を収集
        main_top = Self::collect_top_indices(dims.main, main_bit);

        main_top
    }

    /// 指定された BitVec の上位層インデックスを集める
    fn collect_top_indices(map: &BTreeMap<BitVec, LayerInfo>, key: &BitVec) -> HashSet<Index> {
        let mut result = HashSet::new();
        for top in key.top_prefix() {
            if let Some(v) = map.get(&top) {
                result.extend(v.index.iter().copied());
            }
        }
        result
    }
}
