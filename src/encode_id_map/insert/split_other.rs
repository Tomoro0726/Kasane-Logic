use std::collections::{HashMap, HashSet};

use crate::{
    bit_vec::BitVec,
    encode_id::EncodeID,
    encode_id_map::{EncodeIDMap, Index, utils::select_dimensions::DimensionSelect},
};

impl<V: Clone> EncodeIDMap<V> {
    ///上位,上位,下位の場合に相手を切断する
    pub(crate) fn split_other(
        &self,
        target_index: &Index,
        target_reverse: &EncodeID,
        target_bit: &BitVec,
        target_dim: &DimensionSelect,
        need_delete: &mut HashSet<Index>,
        need_insert: &mut HashMap<EncodeID, V>,
    ) {
        let top = match target_dim {
            DimensionSelect::F => target_reverse.f.clone(),
            DimensionSelect::X => target_reverse.x.clone(),
            DimensionSelect::Y => target_reverse.y.clone(),
        };

        let splited = top.subtract_range(&target_bit);

        let reverse_f = target_reverse.f.clone();
        let reverse_x = target_reverse.x.clone();
        let reverse_y = target_reverse.y.clone();

        // Get the value for this index
        let value = self
            .reverse
            .get(target_index)
            .expect("Internal error: reverse index not found in split_other")
            .1
            .clone();

        for single in splited {
            match target_dim {
                DimensionSelect::F => {
                    need_insert.insert(
                        EncodeID {
                            f: single,
                            x: reverse_x.clone(),
                            y: reverse_y.clone(),
                        },
                        value.clone(),
                    );
                }
                DimensionSelect::X => {
                    need_insert.insert(
                        EncodeID {
                            f: reverse_f.clone(),
                            x: single,
                            y: reverse_y.clone(),
                        },
                        value.clone(),
                    );
                }
                DimensionSelect::Y => {
                    need_insert.insert(
                        EncodeID {
                            f: reverse_f.clone(),
                            x: reverse_x.clone(),
                            y: single,
                        },
                        value.clone(),
                    );
                }
            };
        }

        need_delete.insert(*target_index);
    }
}
