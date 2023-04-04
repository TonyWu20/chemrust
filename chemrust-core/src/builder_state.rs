//! The states required to implement a builder pattern.
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

#[derive(Default, Debug)]
pub struct Yes;
#[derive(Default, Debug)]
pub struct No;

pub trait ToAssign: Debug {}
pub trait Assigned: ToAssign {}
pub trait NotAssigned: ToAssign {}

impl ToAssign for Yes {}
impl ToAssign for No {}

impl Assigned for Yes {}
impl NotAssigned for No {}
