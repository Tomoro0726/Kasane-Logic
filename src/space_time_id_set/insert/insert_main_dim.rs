use crate::space_time_id_set::insert::check_relation::Relation;
use crate::{space_time_id_set::SpaceTimeIdSet, r#type::bit_vec::BitVec};

#[derive(Clone)]
pub enum MainDimensionSelect {
    F,
    X,
    Y,
}

pub struct DimRelation {
    f: Relation,
    x: Relation,
    y: Relation,
}

impl SpaceTimeIdSet {
    /// 代表次元×他の次元を挿入処理する
    pub fn insert_main_dim(
        &mut self,
        main_bit: &BitVec,
        main_index: &usize,
        main_under_count: &usize,
        main_encoded: &mut Vec<(usize, BitVec)>,
        other_encoded: &[&Vec<(usize, BitVec)>; 2],
        main_dim_select: MainDimensionSelect,
    ) {
        //代表次元における上位範囲を収拾する
        let main_top = Self::collect_top(&self, main_bit, &main_dim_select);

        //代表次元において、上位も下位も存在しなかった場合
        if main_top.is_empty() && *main_under_count == 0 {
            //代表次元をBitVecから削除
            let _removed = main_encoded.remove(*main_index);

            //挿入
            self.uncheck_insert_combinations(&main_dim_select, main_bit, other_encoded);
            return;
        }

        //代表次元における下位範囲を収拾する
        let main_under = self.collect_under(main_bit, &main_dim_select);

        //逆引きをして範囲を照合

        //main_topから検索
        for index in main_top {
            let revese = match self.reverse.get(&index) {
                Some(v) => v,
                None => {
                    continue;
                }
            };

            match main_dim_select {
                MainDimensionSelect::F => {}
                MainDimensionSelect::X => todo!(),
                MainDimensionSelect::Y => todo!(),
            }
        }

        //------------------------------------------

        //main_underを検索

        //全てが下位の場合→そのIDをdelete

        //main_underのみが下位で、残りの2つが上位の場合（多数決で勝ち→相手を削る）

        //main_topと1つが下位で、残りの1つが上位の場合（多数決で負け→自分を削る）
    }
}
