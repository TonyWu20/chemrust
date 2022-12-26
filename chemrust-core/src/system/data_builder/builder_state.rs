use std::fmt::Debug;

#[derive(Debug)]
pub struct Pending;
#[derive(Debug)]
pub struct Ready;
#[derive(Debug)]
pub struct Done;

pub trait BuilderState: Debug {}

macro_rules! impl_state {
    ($($x: ty), *) => {
        $(impl BuilderState for $x{})*
    };
}

impl_state!(Pending, Ready, Done);
