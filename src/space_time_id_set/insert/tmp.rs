use std::collections::{BTreeMap, HashSet};

use crate::{
    space_time_id_set::{Index, LayerInfo, SpaceTimeIdSet},
    r#type::bit_vec::BitVec,
};

impl SpaceTimeIdSet {
    pub fn tmp(
        main_dim: &mut BTreeMap<BitVec, LayerInfo>,
        other_dim: &[&mut BTreeMap<BitVec, LayerInfo>; 2],
        main_under_count: &usize,
        main: &BitVec,
        main_encoded: &mut Vec<(usize, BitVec)>,
        main_index: &usize,
        other_encoded: &[&mut Vec<(usize, BitVec)>; 2],
    ) {
        //まず上位の範囲を検索(同位を含む)
        let mut main_top: HashSet<Index> = HashSet::new();

        for top in main.top_prefix() {
            match main_dim.get(&top) {
                Some(v) => main_top.extend(v.index.clone()),
                None => {}
            }
        }

        //上位も下位も0の場合
        if main_top.len() == 0 && *main_under_count == 0 {
            //encodedの配列から代表次元を削除し、取り出す
            let removed = main_encoded.remove(*main_index);

            //他の次元と組み合わせて挿入する
        }

        //下位の範囲を検索する
        let mut main_under: HashSet<Index> = HashSet::new();

        //必要な場合は検索
    }
}
