use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::iter;
use std::ops;
use std::result::Result;

pub trait TableIndex: TryFrom<usize> + TryInto<usize> + Clone + Copy + fmt::Display + Eq {}
impl<T> TableIndex for T
where
    T: TryFrom<usize> + TryInto<usize> + Clone + Copy + fmt::Display + Eq {}

pub struct Table<T: TableIndex>  {
    m: usize,
    matrix: Vec<T>,
}

impl<T: TableIndex> Table<T> {
    pub fn new(m: usize) -> Table<T> {
        Table {m: m, matrix: Vec::new()}
    }

    pub fn len(&self) -> usize {
        self.matrix.len() / self.m
    }

    pub fn add_row(& mut self) -> Result<T, ()> {
        let size: usize =  self.matrix.len();
        let row = size / self.m;
        let can_add = T::try_from(row);
        match can_add {
            Ok(val) => {
                self.matrix.resize(size + self.m, val);
                return Ok(val);
            }
            Err(_e) => {
                return Err(());
            }
        }
    }

    pub fn iter<'a>(& 'a self, start: T, transform: fn(& Self, T) -> T) -> impl Iterator<Item=T> + 'a{
        iter::successors(Some(start), move |he: &T| {
            let next_he = transform(self, *he);
            if next_he == start {
                None
            }else{
                Some(next_he)
            }
        })
    }

    fn get_row_pos(&self, row: T) -> Result<usize, T> {
        let _row: Result<usize, <T as TryInto<usize>>::Error> = row.try_into();
        match _row {
            Ok(v) => Ok(v * self.m),
            Err(_e) => Err(row)
        }
    }
}

impl<T: TableIndex> ops::Index<T> for Table<T>{
    type Output = [T];

    fn index(&self, row: T) -> &[T] {
        let can_access = self.get_row_pos(row);
        match can_access {
            Ok(pos) => &self.matrix[pos..pos+self.m],
            Err(orig) => panic!("{} cannot be used as an index", orig)
        }
    }
}

impl<T: TableIndex> ops::IndexMut<T> for Table<T>{
    fn index_mut(& mut self, row: T) -> & mut [T] {
        let can_access = self.get_row_pos(row);
        match can_access {
            Ok(pos) => & mut self.matrix[pos..pos+self.m],
            Err(orig) => panic!("{} cannot be used as an index", orig)
        }
    }
}

fn fmt_column_as_row<I, T: fmt::Display>(range: I, width: usize, f: &mut fmt::Formatter) -> fmt::Result 
where
    I: IntoIterator<Item=T>
{
    for i in range {
        let res = write!(f, "{:width$} ", i, width=width);
        match res {
            Err(e) => return Err(e),
            _ => {}
        }
    }
    write!(f, "\n")
}

impl<T: TableIndex> fmt::Display for Table<T>{

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let max: f64 = self.len() as f64;
        let n_digits = max.log10() as usize + 1;
        let mut res = fmt_column_as_row(0..self.len(), n_digits, f); 
        match res {
            Err(e) => return Err(e),
            _ => {}
        }
        for i in 0..self.m {
            res = fmt_column_as_row((0..self.len()).map(|j| self.matrix[self.m * j + i]), n_digits, f);
            match res {
                Err(e) => return Err(e),
                _ => {}
            }
        }
        res
    }
}

pub trait Linkable: TableIndex {
    fn link(table: & mut Table<Self>, lhs: Self, rhs: Self, level: usize);
}

macro_rules! linkable_impl_int {
    ($($t:ty)*) => ($(
        impl Linkable for $t {
        
            fn link(table: & mut Table<$t>, lhs: $t, rhs: $t, level: usize){

                let tmp: $t = table[lhs][level];
                table[lhs][level] = rhs;
                table[rhs][level] = tmp;
            }
        }
    )*)
}

linkable_impl_int!{ u8 u16 u32 u64 usize i8 i16 i32 i64 isize }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_table(){
        let t: Table<u32> = Table::new(3);
        assert!(t.m == 3 && t.matrix.len() == 0);
    }

    #[test]
    fn test_get_row(){
        let mut t: Table<u32> = Table::new(3);
        let r = t.add_row();
        assert!(r != Err(()));
        assert!(t[0][0] == 0);
        assert!(t[0][1] == 0);
        assert!(t[0][2] == 0);
    }

    #[test]
    fn test_edit_row(){
        let mut t: Table<u32> = Table::new(3);
        let r = t.add_row();
        assert!(r != Err(()));
        t[0][0] = 2;
        t[0][1] = 3;
        t[0][2] = 4;
        assert!(t.matrix[0] == 2);
        assert!(t.matrix[1] == 3);
        assert!(t.matrix[2] == 4);
    }

    #[test]
    fn test_conversion_failure(){
        let mut t: Table<u8> = Table::new(3);
        for i in 0..=255 {
            let r = t.add_row();
            assert!(r != Err(()));
            assert!(t[i][0] == i);
            assert!(t[i][1] == i);
            assert!(t[i][2] == i);
        }
        let r = t.add_row(); 
        assert!(r == Err(()));
    }

    #[test]
    #[should_panic]
    fn test_index_failure(){
        let mut t: Table<i8> = Table::new(3);
        t[-1][0] = 1;
    }

    #[test]
    #[should_panic]
    fn test_get_row_out_of_bounds(){
        let mut t: Table<u32> = Table::new(3);
        let r = t.add_row();
        assert!(r != Err(()));
        let r = t.add_row();
        assert!(r != Err(()));
        assert!(t[0][t.m] == 1);
    }

    #[test]
    #[should_panic]
    fn test_get_row_fails(){
        let t: Table<u32>  = Table::new(3);
        let _val: u32 = t[2][0];
    }
}
