use crate::{encode_id::EncodeID, encode_id_set::EncodeIDSet};

impl EncodeIDSet {
    pub fn get(&self, encode_id: &EncodeID) -> Vec<EncodeID> {
        self.inner().get_ids(encode_id)
    }
}
