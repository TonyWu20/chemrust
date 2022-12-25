use std::ops::Deref;

use super::AttrView;

pub struct CollectionIter<'c, T> {
    data: &'c [T],
}

impl<'c, T> Iterator for CollectionIter<'c, T> {
    type Item = &'c T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((prefix_elem, suffix)) = self.data.split_first() {
            self.data = suffix;
            Some(prefix_elem)
        } else {
            None
        }
    }
}

impl<'c, T> Deref for CollectionIter<'c, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

pub trait Iterable {
    type Item<'collection>
    where
        Self: 'collection;
    type Iterator<'collection>: Iterator<Item = Self::Item<'collection>>
    where
        Self: 'collection;

    fn iter<'c>(&'c self) -> Self::Iterator<'c>;
}

impl<T> Iterable for Vec<T> {
    type Item<'c> = &'c T
    where
        T: 'c;

    type Iterator<'c>=CollectionIter<'c, T>
    where
        T: 'c;

    fn iter<'c>(&'c self) -> Self::Iterator<'c> {
        CollectionIter { data: self }
    }
}

pub trait CollectionViewer<T> {
    type Item;
    fn get_by_index(&self, index: usize) -> Option<&Self::Item>;
    fn get_mut_by_index(&mut self, index: usize) -> Option<&mut Self::Item>;
    fn get_by_item_index<U>(&self, item: &U) -> Option<&Self::Item>
    where
        U: AttrView;
    fn get_mut_by_item_index<U>(&mut self, item: &U) -> Option<&mut Self::Item>
    where
        U: AttrView;
}

impl<T> CollectionViewer<T> for Vec<T> {
    type Item = T;

    fn get_by_index(&self, index: usize) -> Option<&Self::Item> {
        self.get(index)
    }

    fn get_mut_by_index(&mut self, index: usize) -> Option<&mut Self::Item> {
        self.get_mut(index)
    }

    fn get_by_item_index<U>(&self, item: &U) -> Option<&Self::Item>
    where
        U: AttrView,
    {
        let index = item.index();
        self.get(*index)
    }

    fn get_mut_by_item_index<U>(&mut self, item: &U) -> Option<&mut Self::Item>
    where
        U: AttrView,
    {
        let index = item.index();
        self.get_mut(*index)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        data::atom::AtomId,
        system::data_view::{attributes::AttrBuild, collections::CollectionViewer},
    };

    #[test]
    fn test_gat() {
        let a: Vec<i32> = (0..12).into_iter().collect();
        let atom_id = AtomId::new(1, 8);
        assert_eq!(Some(&0), a.get_by_index(0));
        assert_eq!(Some(&8), a.get_by_item_index(&atom_id));
    }
}
