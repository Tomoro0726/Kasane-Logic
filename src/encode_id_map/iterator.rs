use crate::{encode_id::EncodeID, encode_id_map::EncodeIDMap};

pub struct EncodeIDMapIter<'a, V> {
    reverse_iter: std::collections::hash_map::Iter<'a, usize, (EncodeID, V)>,
}

impl<V> EncodeIDMap<V> {
    pub fn iter(&'_ self) -> EncodeIDMapIter<'_, V> {
        EncodeIDMapIter {
            reverse_iter: self.reverse.iter(),
        }
    }

    pub fn len(&self) -> usize {
        self.reverse.len()
    }

    pub fn is_empty(&self) -> bool {
        self.reverse.is_empty()
    }
}

impl<'a, V: Clone> Iterator for EncodeIDMapIter<'a, V> {
    type Item = (EncodeID, V);

    fn next(&mut self) -> Option<Self::Item> {
        let (_index, (encode_id, value)) = self.reverse_iter.next()?;
        Some((encode_id.clone(), value.clone()))
    }
}

impl<'a, V: Clone> IntoIterator for &'a EncodeIDMap<V> {
    type Item = (EncodeID, V);
    type IntoIter = EncodeIDMapIter<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, V> ExactSizeIterator for EncodeIDMapIter<'a, V>
where
    V: Clone,
{
    fn len(&self) -> usize {
        self.reverse_iter.len()
    }
}
