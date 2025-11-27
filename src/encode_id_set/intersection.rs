use crate::encode_id_set::EncodeIDSet;

impl EncodeIDSet {
    /// 二つのSpaceTimeIDSetを結合する
    pub fn intersection(&self, other: &EncodeIDSet) -> EncodeIDSet {
        // 小さい方を small、大きい方を large にする
        let (small, large) = if self.len() <= other.len() {
            (self, other)
        } else {
            (other, self)
        };

        // large をコピーして small の内容を挿入する
        let mut result = EncodeIDSet::new();

        //個別に処理を行う
        for encode_id in small.iter() {
            for id in large.get(&encode_id) {
                result.uncheck_insert(id);
            }
        }

        result
    }
}
