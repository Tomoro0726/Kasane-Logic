use crate::encode_id_set::EncodeIDSet;

impl EncodeIDSet {
    pub fn difference(&self, other: &EncodeIDSet) -> EncodeIDSet {
        let (small, large) = if self.len() <= other.len() {
            (self, other)
        } else {
            (other, self)
        };

        // large をコピーして small の内容を削除するする
        let mut result = large.clone();
        for encode_id in small.iter() {
            result.remove(encode_id);
        }

        result
    }
}
