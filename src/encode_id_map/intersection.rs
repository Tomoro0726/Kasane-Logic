use crate::encode_id_map::EncodeIDMap;

impl<V: Clone> EncodeIDMap<V> {
    /// 二つのEncodeIDMapの積集合を計算する
    pub fn intersection(&self, other: &EncodeIDMap<V>) -> EncodeIDMap<V> {
        // 小さい方を small、大きい方を large にする
        let (small, large) = if self.len() <= other.len() {
            (self, other)
        } else {
            (other, self)
        };

        // large をコピーして small の内容を挿入する
        let mut result = EncodeIDMap::new();

        //個別に処理を行う
        for (_, (encode_id, _)) in &small.reverse {
            for (id, value) in large.get(encode_id) {
                result.uncheck_insert(id, value);
            }
        }

        result
    }
}
