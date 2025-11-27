use crate::encode_id_map::EncodeIDMap;

impl<V: Clone> EncodeIDMap<V> {
    /// 二つのEncodeIDMapを結合する
    pub fn union(&self, other: &EncodeIDMap<V>) -> EncodeIDMap<V> {
        // IDの個数が少ないほうを small、多いほうを large にする
        let (small, large) = if self.len() <= other.len() {
            (self, other)
        } else {
            (other, self)
        };

        // large をコピーして small の内容を挿入する
        let mut result = large.clone();
        for (_, (encode_id, value)) in small.reverse.clone() {
            result.uncheck_insert(encode_id, value);
        }

        result
    }
}
