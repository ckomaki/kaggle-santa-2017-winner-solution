use std::convert::{From, Into};
use std::ops::{Neg, Add, Sub, Mul, AddAssign, SubAssign};
use std::io;
use std::cmp;
use std::fmt::{Display, Debug};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::marker::Copy;

pub trait I: Copy + Neg<Output = Self> + Eq + Ord + Default + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Debug + AddAssign + SubAssign + Clone + Display {}

impl I for i32 {}
impl I for i64 {}

pub trait MinCostFlow<CAPACITY: I, COST: I> {
    fn flow(&mut self, source: usize, sink: usize, flow: CAPACITY, inf: COST) -> COST;
    fn add_edge(&mut self, source: usize, sink: usize, capacity: CAPACITY, cost: COST) -> &mut MinCostFlow<CAPACITY, COST>;
    fn write_assignment(&self);
}
