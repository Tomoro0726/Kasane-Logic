use crate::{encode_id::EncodeID, encode_id_set::EncodeIDSet};

impl EncodeIDSet {
    ///EncodeIDSetから指定されたEncodeIDを削除する。
    /// 既存の範囲と重複がある場合は削除時に調整が行われ、削除されたIDが返される。
    pub fn remove(&mut self, encode_id: EncodeID) -> Vec<EncodeID> {
        self.inner_mut()
            .remove(encode_id)
            .into_iter()
            .map(|(id, _)| id)
            .collect()
    }
}
