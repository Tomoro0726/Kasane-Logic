use crate::encode_id_map::EncodeIDMap;

impl<V: Clone> EncodeIDMap<V> {
    pub fn difference(&self, other: &EncodeIDMap<V>) -> EncodeIDMap<V> {
        let (small, large) = if self.len() <= other.len() {
            (self, other)
        } else {
            (other, self)
        };

        // large をコピーして small の内容を削除するする
        let mut result = large.clone();
        for (_, (encode_id, _)) in small.reverse.clone() {
            result.remove(encode_id);
        }

        result
    }
}
