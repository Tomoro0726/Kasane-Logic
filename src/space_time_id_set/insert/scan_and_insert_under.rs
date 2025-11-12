use std::collections::HashSet;

use crate::{
    space_time_id_set::{
        Index, SpaceTimeIdSet,
        insert::{
            check_relation::{self, Relation},
            insert_main_dim::MainDimensionSelect,
        },
    },
    r#type::bit_vec::BitVec,
};

impl SpaceTimeIdSet {
    pub fn scan_and_insert_under(
        &mut self,
        main_bit: &BitVec,
        main_under: &HashSet<Index>,
        other_encoded: &[&Vec<(usize, BitVec)>; 2],
        main_dim_select: MainDimensionSelect,
        main_under_count: &usize,
    ) {
        // コピーは破壊的操作用
        let mut other_encoded_copy = other_encoded.map(|v| (*v).clone());

        // 軸を動的に決定
        let main_idx = main_dim_select.as_index();
        let other_axes: [usize; 2] = match main_idx {
            0 => [1, 2], // F -> X,Y
            1 => [0, 2], // X -> F,Y
            2 => [0, 1], // Y -> F,X
            _ => unreachable!(),
        };

        for index in main_under {
            let reverse = self.reverse.get(index).unwrap();
            let target_bits = [&reverse.f.clone(), &reverse.x.clone(), &reverse.y.clone()];
            let mut target_main = target_bits[main_idx].clone();

            // 他軸の参照を動的に取得
            let target_a = &target_bits[other_axes[0]];
            let target_b = &target_bits[other_axes[1]];

            // 2軸間で共通ロジックを使う
            let mut a_relations = Vec::new();
            let mut b_relations = Vec::new();

            // ---- A軸を処理 ----
            for (i, (_, bit_a)) in other_encoded[0].iter().enumerate() {
                let relation = Self::check_relation(bit_a, target_a);

                if relation == Relation::Disjoint {
                    let removed = other_encoded_copy[0].remove(i);
                    for (_, bit_b) in &other_encoded_copy[1] {
                        self.uncheck_insert(main_bit, &removed.1, bit_b);
                    }
                    continue;
                }

                a_relations.push((i, relation));
            }

            // ---- B軸を処理 ----
            for (i, (_, bit_b)) in other_encoded[1].iter().enumerate() {
                let relation = Self::check_relation(bit_b, target_b);

                if relation == Relation::Disjoint {
                    let removed = other_encoded_copy[1].remove(i);
                    for (_, bit_a) in &other_encoded_copy[0] {
                        self.uncheck_insert(main_bit, &removed.1, bit_a);
                    }
                    continue;
                }

                b_relations.push((i, relation));
            }

            // ---- メイン軸を含めた結合処理 ----
            for (ai, a_rel) in &a_relations {
                for (bi, b_rel) in &b_relations {
                    //全てが下位の場合はそのIDを削除
                    if (*a_rel == Relation::Under) && (*b_rel == Relation::Under) {
                        self.uncheck_delete(index);
                    }

                    // メイン軸は target_main
                    if (*a_rel == Relation::Top) && (*b_rel == Relation::Under) {
                        let split_a = Self::split_dimension(
                            target_a,
                            &mut other_encoded_copy[0][*ai].1.clone(),
                        );
                        for bit_a in split_a {
                            self.uncheck_insert(main_bit, &bit_a, &other_encoded_copy[1][*bi].1);
                        }
                    }

                    if (*a_rel == Relation::Under) && (*b_rel == Relation::Top) {
                        let split_b = Self::split_dimension(
                            target_b,
                            &mut other_encoded_copy[1][*bi].1.clone(),
                        );
                        for bit_b in split_b {
                            self.uncheck_insert(main_bit, &other_encoded_copy[0][*ai].1, &bit_b);
                        }
                    }

                    // mainに突き抜けていたIDを分割
                    for bit_main in Self::split_dimension(&main_bit, &mut target_main) {
                        let f = &bit_main;
                        let x = &target_a;
                        let y = &target_b;
                        self.uncheck_insert(f, x, y);
                    }
                }
            }
        }
    }
}
