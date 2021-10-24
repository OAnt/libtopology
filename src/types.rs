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

