use crate::{encode_id::EncodeID, encode_id_set::EncodeIDSet};

impl EncodeIDSet {
    ///EncodeIDSetに新規のEncodeIDを挿入する。
    /// 既存の範囲と重複がある場合は挿入時に調整が行われ、重複が排除される。
    pub fn insert(&mut self, encode_id: EncodeID) {
        self.inner_mut().insert(encode_id, ());
    }

    /// 内部チェックを行わずに挿入する
    pub(crate) fn uncheck_insert(&mut self, encode_id: EncodeID) {
        self.inner_mut().uncheck_insert(encode_id, ());
    }
}
