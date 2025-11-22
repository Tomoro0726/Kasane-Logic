use std::collections::{HashSet, btree_map::Range};

use itertools::iproduct;

use crate::{
    bit_vec::{BitVec, relation::BitVecRelation},
    space_time_id_set::{
        Index, Interval, ReverseInfo, SpaceTimeIdSet,
        insert::{select_dimensions::DimensionSelect, under_under_top::RangesCollect},
    },
};

impl SpaceTimeIdSet {
    /// 代表次元×他の次元を挿入処理する
    pub(crate) fn insert_main_dim(
        &mut self,
        main_bit: &BitVec,
        main_index: &Index,
        main_descendant_count: &usize,
        main_encoded: &mut Vec<(Index, BitVec)>,
        a_encoded: &Vec<(Index, BitVec)>,
        b_encoded: &Vec<(Index, BitVec)>,
        main_dim_select: DimensionSelect,
    ) {
        let main_ancestors: Vec<Index> = Self::collect_ancestors(&self, main_bit, &main_dim_select);

        if main_ancestors.is_empty() && *main_descendant_count == 0 {
            for ((_, a_bit), (_, b_bit)) in iproduct!(a_encoded, b_encoded) {
                self.uncheck_insert(main_bit, a_bit, b_bit, &main_dim_select);
            }
            let _removed = main_encoded.remove(*main_index);
            return;
        }

        let main_top: Vec<Index> = self.collect_under(main_bit, &main_dim_select);

        let mut top_reverse = vec![];
        for top_index in &main_top {
            top_reverse.push(
                self.reverse
                    .get(&top_index)
                    .expect("Internal error: reverse index not found for top"),
            );
        }

        let mut under_reverse = vec![];
        for under_index in &main_ancestors {
            under_reverse.push(
                self.reverse
                    .get(&*under_index)
                    .expect("Internal error: reverse index not found for under"),
            );
        }

        let a_dim_select: DimensionSelect;
        let b_dim_select: DimensionSelect;

        match main_dim_select {
            DimensionSelect::F => {
                a_dim_select = DimensionSelect::X;
                b_dim_select = DimensionSelect::Y;
            }
            DimensionSelect::X => {
                a_dim_select = DimensionSelect::F;
                b_dim_select = DimensionSelect::Y;
            }
            DimensionSelect::Y => {
                a_dim_select = DimensionSelect::F;
                b_dim_select = DimensionSelect::X;
            }
        }

        let mut a_relations: Vec<Option<(Vec<BitVecRelation>, Vec<BitVecRelation>)>> = Vec::new();
        let mut b_relations: Vec<Option<(Vec<BitVecRelation>, Vec<BitVecRelation>)>> = Vec::new();

        for (_, a_dim) in a_encoded {
            a_relations.push(Self::collect_other_dimension(
                a_dim,
                &a_dim_select,
                &top_reverse,
                &under_reverse,
            ));
        }

        for (_, b_dim) in b_encoded {
            b_relations.push(Self::collect_other_dimension(
                b_dim,
                &b_dim_select,
                &top_reverse,
                &under_reverse,
            ));
        }

        let mut need_delete: HashSet<Index> = HashSet::new();
        let mut need_insert: HashSet<ReverseInfo> = HashSet::new();

        'outer: for ((a_encode_index, a), (b_encode_index, b)) in iproduct!(
            a_relations.iter().enumerate(),
            b_relations.iter().enumerate()
        ) {
            let a_relation = match a {
                Some(v) => v,
                None => {
                    self.uncheck_insert(
                        main_bit,
                        &a_encoded[a_encode_index].1,
                        &b_encoded[b_encode_index].1,
                        &main_dim_select,
                    );
                    continue;
                }
            };

            let b_relation = match b {
                Some(v) => v,
                None => {
                    self.uncheck_insert(
                        main_bit,
                        &a_encoded[a_encode_index].1,
                        &b_encoded[b_encode_index].1,
                        &main_dim_select,
                    );
                    continue;
                }
            };

            let mut need_divison = RangesCollect {
                main: vec![],
                a: vec![],
                b: vec![],
            };

            let mut need_delete_inside: HashSet<Index> = HashSet::new();
            let mut need_insert_inside: HashSet<ReverseInfo> = HashSet::new();

            for (i, (a_rel, b_rel)) in a_relation.0.iter().zip(b_relation.0.iter()).enumerate() {
                match (a_rel, b_rel) {
                    (
                        BitVecRelation::Greater | BitVecRelation::Equal,
                        BitVecRelation::Greater | BitVecRelation::Equal,
                    ) => {
                        need_delete_inside.insert(main_top[i]);
                    }
                    (BitVecRelation::Greater | BitVecRelation::Equal, BitVecRelation::Less) => {
                        self.top_top_under(
                            main_top[i],
                            b_encoded[b_encode_index].1.clone(),
                            &b_dim_select,
                            &mut need_delete_inside,
                            &mut need_insert_inside,
                        );
                    }
                    (BitVecRelation::Less, BitVecRelation::Greater | BitVecRelation::Equal) => {
                        self.top_top_under(
                            main_top[i],
                            a_encoded[a_encode_index].1.clone(),
                            &a_dim_select,
                            &mut need_delete_inside,
                            &mut need_insert_inside,
                        );
                    }
                    (BitVecRelation::Less, BitVecRelation::Less) => {
                        self.under_under_top(&mut need_divison, main_top[i], &main_dim_select);
                    }
                    _ => {}
                }
            }

            for (i, (a_rel, b_rel)) in a_relation.1.iter().zip(b_relation.1.iter()).enumerate() {
                match (a_rel, b_rel) {
                    (
                        BitVecRelation::Greater | BitVecRelation::Equal,
                        BitVecRelation::Greater | BitVecRelation::Equal,
                    ) => {
                        self.top_top_under(
                            main_ancestors[i],
                            main_bit.clone(),
                            &main_dim_select,
                            &mut need_delete_inside,
                            &mut need_insert_inside,
                        );
                    }
                    (BitVecRelation::Greater | BitVecRelation::Equal, BitVecRelation::Less) => {
                        self.under_under_top(&mut need_divison, main_ancestors[i], &a_dim_select);
                    }
                    (BitVecRelation::Less, BitVecRelation::Greater | BitVecRelation::Equal) => {
                        self.under_under_top(&mut need_divison, main_ancestors[i], &b_dim_select);
                    }
                    (BitVecRelation::Less, BitVecRelation::Less) => {
                        continue 'outer;
                    }
                    _ => {}
                }
            }

            let main_splited = main_bit.subtract_ranges(&need_divison.main);

            let a_splited = a_encoded[a_encode_index]
                .1
                .subtract_ranges(&need_divison.main);
            let b_splited = b_encoded[b_encode_index].1.subtract_ranges(&need_divison.b);

            for (main, a, b) in iproduct!(main_splited, a_splited, b_splited) {
                self.uncheck_insert(&main, &a, &b, &main_dim_select);
            }

            need_delete.extend(need_delete_inside);
            need_insert.extend(need_insert_inside);
        }
        for need_delete_index in need_delete {
            self.uncheck_delete(&need_delete_index);
        }
        for reverse in need_insert {
            self.uncheck_insert(&reverse.f, &reverse.x, &reverse.y, &DimensionSelect::F);
        }

        main_encoded.remove(*main_index);
    }
}
