pub trait AttrCollectionView<Ref> {
    type Ref;
    type Output;
    type NewCollection;
    /// Get content field of the tuple struct in the collection.
    fn view_content_at(&self, index: usize) -> Option<&Self::Output>;
    /// Re-arrange the vector by reference to the other vec's index order.
    fn rearrange_with(self, other: &Self::Ref) -> Self::NewCollection;
    /// Set new index for the attribute.
    fn update_all_index(&mut self);
}

#[cfg(test)]
mod test {}
