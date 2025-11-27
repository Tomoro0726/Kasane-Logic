use crate::{encode_id::EncodeID, encode_id_set::EncodeIDSet, space_time_id::SpaceTimeID};

pub struct SpaceTimeIDSetIter<'a> {
    reverse_iter: std::collections::hash_map::Iter<'a, usize, EncodeID>,
}

impl EncodeIDSet {
    pub fn iter(&'_ self) -> SpaceTimeIDSetIter<'_> {
        SpaceTimeIDSetIter {
            reverse_iter: self.reverse.iter(),
        }
    }
}

impl<'a> Iterator for SpaceTimeIDSetIter<'a> {
    type Item = EncodeID;

    fn next(&mut self) -> Option<Self::Item> {
        let (_index, encode_id) = self.reverse_iter.next()?; // &usize, &EncodeID
        Some(encode_id.clone())
    }
}

impl<'a> IntoIterator for &'a EncodeIDSet {
    type Item = EncodeID;
    type IntoIter = SpaceTimeIDSetIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> ExactSizeIterator for SpaceTimeIDSetIter<'a> {
    fn len(&self) -> usize {
        self.reverse_iter.len()
    }
}
