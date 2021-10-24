use std::fmt;
use super::table::Table;
use super::types::{TableIndex, TableElement};

pub trait Linked: TableElement{
    fn link(table: & mut Table<Self>, lhs: Self::Type, rhs: Self::Type, level: usize);
    fn next(&self) -> Self::Type;
}

#[derive(Copy, Clone)]
pub struct Cell<T: TableIndex> {
    fwd: T,
}

impl<T: TableIndex> fmt::Display for Cell<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.fwd)
    }
}

impl<T: TableIndex> TableElement for Cell<T>{
    type Type = T;

    fn new(t: T) -> Cell<T> {
        Cell{fwd: t}
    }
}

impl<T: TableIndex> Linked for Cell<T> {
    fn link(table: & mut Table<Cell<T> >, lhs: T, rhs: T, level: usize){
        let tmp: T = table[lhs][level].fwd;
        table[lhs][level].fwd = rhs;
        table[rhs][level].fwd = tmp;
    }
    
    fn next(&self) -> T {
        self.fwd
    }
}

macro_rules! linked_impl {
    ($($t:ty)*) => ($(
        impl Linked for $t {
            fn link(table: & mut Table<$t>, lhs: $t, rhs: $t, level: usize){
                let tmp: $t = table[lhs][level];
                table[lhs][level] = rhs;
                table[rhs][level] = tmp;
            }

            fn next(&self) -> $t {
                *self
            }
        }
    )*)
}

linked_impl! { usize u8 u16 u32 u64 i8 i16 i32 i64 isize }
