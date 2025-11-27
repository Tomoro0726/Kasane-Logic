use crate::{encode_id::EncodeID, encode_id_set::EncodeIDSet};

pub struct EncodeIDSetIter<'a> {
    inner_iter: crate::encode_id_map::iterator::EncodeIDMapIter<'a, ()>,
}

impl EncodeIDSet {
    pub fn iter(&'_ self) -> EncodeIDSetIter<'_> {
        EncodeIDSetIter {
            inner_iter: self.inner().iter(),
        }
    }

    pub fn len(&self) -> usize {
        self.inner().len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner().is_empty()
    }
}

impl<'a> Iterator for EncodeIDSetIter<'a> {
    type Item = EncodeID;

    fn next(&mut self) -> Option<Self::Item> {
        let (encode_id, _) = self.inner_iter.next()?;
        Some(encode_id)
    }
}

impl<'a> IntoIterator for &'a EncodeIDSet {
    type Item = EncodeID;
    type IntoIter = EncodeIDSetIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> ExactSizeIterator for EncodeIDSetIter<'a> {
    fn len(&self) -> usize {
        self.inner_iter.len()
    }
}
