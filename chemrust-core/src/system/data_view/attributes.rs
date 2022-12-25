pub trait AttrView {
    type Output;
    /// Get content of the tuple struct.
    fn content(&self) -> &Self::Output;
    /// Get mut reference of the tuple struct.
    fn content_mut(&mut self) -> &mut Self::Output;
    /// Get the index-key in the Vec<T> of this T.
    fn index(&self) -> &usize;
    /// Set new index for the attribute.
    fn set_index(&mut self, new_id: usize);
}

pub trait AttrBuild {
    type Input;
    type Output;
    fn new(input: Self::Input, index: usize) -> Self::Output;
}
