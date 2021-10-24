use std::convert::{TryFrom, TryInto};
use std::fmt;

pub trait TableIndex: TryFrom<usize> + TryInto<usize> + Clone + Copy + fmt::Display + Eq {}
impl<T> TableIndex for T
where
    T: TryFrom<usize> + TryInto<usize> + Clone + Copy + fmt::Display + Eq {}

pub trait TableElement: Clone + fmt::Display + Copy {
    type Type: TableIndex;
    fn new(t: Self::Type) -> Self;
    fn try_from(u: usize) -> Result<Self::Type,  <Self::Type as TryFrom<usize>>::Error> {
        Self::Type::try_from(u)
    }
}

macro_rules! table_element_impl{
    ($($t:ty)*) => ($(
        impl TableElement for $t {
            type Type = $t;
            fn new(t: $t) -> $t{
                t
            }
        }
    )*)
}

table_element_impl! { usize u8 u16 u32 u64 i8 i16 i32 i64 isize }
