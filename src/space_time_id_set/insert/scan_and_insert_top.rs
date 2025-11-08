use std::collections::HashSet;

use crate::{
    space_time_id_set::{
        Index, SpaceTimeIdSet,
        insert::{check_relation, insert_main_dim::MainDimensionSelect},
    },
    r#type::bit_vec::BitVec,
};

impl SpaceTimeIdSet {
    pub fn scan_and_insert_top(
        &self,
        main_top: &HashSet<Index>,
        other_encoded: &[&Vec<(usize, BitVec)>; 2],
        main_dim_select: MainDimensionSelect,
    ) {
        //徐々に削除していくので配列をコピーする必要がありそう
        let other_encoded_copy = other_encoded.clone();

        for index in main_top {
            let dim = self.select_dimensions(&main_dim_select);
            let reverse = self.reverse.get(index).unwrap();
            let target_x = &reverse.x;
            let target_y = &reverse.y;

            match main_dim_select {
                MainDimensionSelect::F => {
                    let mut x_relations = vec![];
                    let mut y_relations = vec![];

                    //Xについて調べる
                    for (x_index, x_bit) in other_encoded[0] {
                        let x_relation = Self::check_relation(x_bit, &target_x);
                        match x_relation {
                            check_relation::Relation::Disjoint => {
                                //Xが無関係だったので強制的に挿入する
                                let x_bit_removed = other_encoded_copy[0].remove(*x_index);
                            }
                            v => {
                                //関係がある場合は、配列に追加する
                                x_relations.push((x_index, v));
                            }
                        }
                    }

                    //Yについて調べる
                    for (y_index, y_bit) in other_encoded[1] {
                        y_relations.push((y_index, Self::check_relation(y_bit, &target_y)));
                    }
                }
                MainDimensionSelect::X => todo!(),
                MainDimensionSelect::Y => todo!(),
            }
        }
    }
}
