use crate::{
    bit_vec::relation::BitVecRelation,
    encode_id::EncodeID,
    encode_id_map::{EncodeIDMap, Index, utils::select_dimensions::DimensionSelect},
};

impl<V: Clone> EncodeIDMap<V> {
    pub fn get(&self, encode_id: &EncodeID) -> Vec<(EncodeID, V)> {
        let mut result = vec![];

        //下位IDの個数が最小の次元を選択する
        let f_descendants_count = match self.f.get(&encode_id.f) {
            Some(info) => info.count,
            None => 0,
        };

        let x_descendants_count = match self.x.get(&encode_id.x) {
            Some(info) => info.count,
            None => 0,
        };

        let y_descendants_count = match self.y.get(&encode_id.y) {
            Some(info) => info.count,
            None => 0,
        };

        let min_count = f_descendants_count.min(x_descendants_count.min(y_descendants_count));

        //代表の次元を選出する
        let main_dim;
        let a_dim;
        let b_dim;
        let main;
        let a;
        let b;

        if min_count == f_descendants_count {
            main_dim = DimensionSelect::F;
            a_dim = DimensionSelect::X;
            b_dim = DimensionSelect::Y;
            main = &encode_id.f;
            a = &encode_id.x;
            b = &encode_id.y;
        } else if min_count == x_descendants_count {
            main_dim = DimensionSelect::X;
            a_dim = DimensionSelect::F;
            b_dim = DimensionSelect::Y;
            main = &encode_id.x;
            a = &encode_id.f;
            b = &encode_id.y;
        } else {
            main_dim = DimensionSelect::Y;
            a_dim = DimensionSelect::F;
            b_dim = DimensionSelect::X;
            main = &encode_id.y;
            a = &encode_id.f;
            b = &encode_id.x;
        }

        //Main次元の祖先を探索する
        let main_ancestors: Vec<Index> = Self::collect_ancestors(&self, main, &main_dim);

        //Main次元において、祖先にも子孫にも重なる範囲が存在しなければ挿入
        if main_ancestors.is_empty() && min_count == 0 {
            return result;
        }

        //Main次元における祖先のIndexを取得する
        let mut main_ancestors_reverse = vec![];
        let mut main_ancestors_values = vec![];

        //Main次元における祖先を逆引き情報で取得する
        for ancestor_index in &main_ancestors {
            let (encode_id, value) = self
                .reverse
                .get(&*ancestor_index)
                .expect("Internal error: reverse index not found for under");
            main_ancestors_reverse.push(encode_id);
            main_ancestors_values.push(value.clone());
        }

        //Main次元における子孫のIndexを取得する
        let main_descendants: Vec<Index> = self.collect_descendants(main, &main_dim);

        //Main次元における子孫を逆引き情報で取得する
        let mut main_descendants_reverse = vec![];
        let mut main_descendants_values = vec![];
        for descendant_index in &main_descendants {
            let (encode_id, value) = self
                .reverse
                .get(&descendant_index)
                .expect("Internal error: reverse index not found for top");
            main_descendants_reverse.push(encode_id);
            main_descendants_values.push(value.clone());
        }

        let a_relation = match Self::collect_other_dimension(
            a,
            &main_ancestors_reverse,
            &main_descendants_reverse,
            &a_dim,
        ) {
            Some(v) => v,
            None => {
                return result;
            }
        };

        let b_relation = match Self::collect_other_dimension(
            b,
            &main_ancestors_reverse,
            &main_descendants_reverse,
            &b_dim,
        ) {
            Some(v) => v,
            None => {
                return result;
            }
        };

        //Main次元における祖先の範囲を調べる
        for (i, (a_rel, b_rel)) in a_relation.0.iter().zip(b_relation.0.iter()).enumerate() {
            match (a_rel, b_rel) {
                (
                    BitVecRelation::Descendant | BitVecRelation::Equal,
                    BitVecRelation::Descendant | BitVecRelation::Equal,
                ) => {
                    let map_dims = Self::map_dims(main, a, b, &main_dim);
                    result.push((
                        EncodeID {
                            f: map_dims.f.clone(),
                            x: main_ancestors_reverse[i].x.clone(),
                            y: main_ancestors_reverse[i].y.clone(),
                        },
                        main_ancestors_values[i].clone(),
                    ));
                }
                (BitVecRelation::Descendant | BitVecRelation::Equal, BitVecRelation::Ancestor) => {
                    let map_dims = Self::map_dims(main, a, b, &main_dim);
                    result.push((
                        EncodeID {
                            f: map_dims.f.clone(),
                            x: main_ancestors_reverse[i].x.clone(),
                            y: map_dims.y.clone(),
                        },
                        main_ancestors_values[i].clone(),
                    ));
                }
                (BitVecRelation::Ancestor, BitVecRelation::Descendant | BitVecRelation::Equal) => {
                    let map_dims = Self::map_dims(main, a, b, &main_dim);
                    result.push((
                        EncodeID {
                            f: map_dims.f.clone(),
                            x: map_dims.x.clone(),
                            y: main_ancestors_reverse[i].y.clone(),
                        },
                        main_ancestors_values[i].clone(),
                    ));
                }
                (BitVecRelation::Ancestor, BitVecRelation::Ancestor) => {
                    //全ての次元において祖先のIDが存在するため、入力IDそのものを返す
                    return vec![(encode_id.clone(), main_ancestors_values[i].clone())];
                }
                _ => {}
            }
        }

        //Main次元における子孫の範囲について調べる
        for (i, (a_rel, b_rel)) in a_relation.1.iter().zip(b_relation.1.iter()).enumerate() {
            match (a_rel, b_rel) {
                (
                    BitVecRelation::Descendant | BitVecRelation::Equal,
                    BitVecRelation::Descendant | BitVecRelation::Equal,
                ) => {
                    //全ての次元において子孫のIDが存在するため、結果に追加する
                    let (encode_id, value) = self
                        .reverse
                        .get(&main_descendants[i])
                        .expect("Internal error: reverse index not found for under");
                    result.push((encode_id.clone(), value.clone()));
                }
                (BitVecRelation::Descendant | BitVecRelation::Equal, BitVecRelation::Ancestor) => {
                    let map_dims = Self::map_dims(main, a, b, &main_dim);
                    result.push((
                        EncodeID {
                            f: map_dims.f.clone(),
                            x: main_descendants_reverse[i].x.clone(),
                            y: map_dims.y.clone(),
                        },
                        main_descendants_values[i].clone(),
                    ));
                }
                (BitVecRelation::Ancestor, BitVecRelation::Descendant | BitVecRelation::Equal) => {
                    let map_dims = Self::map_dims(main, a, b, &main_dim);
                    result.push((
                        EncodeID {
                            f: map_dims.f.clone(),
                            x: map_dims.x.clone(),
                            y: main_descendants_reverse[i].y.clone(),
                        },
                        main_descendants_values[i].clone(),
                    ));
                }
                (BitVecRelation::Ancestor, BitVecRelation::Ancestor) => {
                    let map_dims = Self::map_dims(main, a, b, &main_dim);
                    result.push((
                        EncodeID {
                            f: main_ancestors_reverse[i].f.clone(),
                            x: map_dims.x.clone(),
                            y: map_dims.y.clone(),
                        },
                        main_descendants_values[i].clone(),
                    ));
                }
                _ => {}
            }
        }

        result
    }

    /// 指定されたEncodeIDに対応する値のみを取得する
    pub fn get_values(&self, encode_id: &EncodeID) -> Vec<V> {
        self.get(encode_id).into_iter().map(|(_, v)| v).collect()
    }

    /// 指定されたEncodeIDに対応するIDのみを取得する
    pub fn get_ids(&self, encode_id: &EncodeID) -> Vec<EncodeID> {
        self.get(encode_id).into_iter().map(|(id, _)| id).collect()
    }
}
