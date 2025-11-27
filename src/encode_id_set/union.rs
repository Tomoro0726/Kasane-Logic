use crate::encode_id_set::EncodeIDSet;

impl EncodeIDSet {
    /// 二つのSpaceTimeIDSetを結合する
    pub fn union(&self, other: &EncodeIDSet) -> EncodeIDSet {
        // IDの個数が少ないほうを small、多いほうを large にする
        let (small, large) = if self.len() <= other.len() {
            (self, other)
        } else {
            (other, self)
        };

        // large をコピーして small の内容を挿入する
        let mut result = large.clone();
        for encode_id in small.iter() {
            result.uncheck_insert(encode_id);
        }

        result
    }
}
