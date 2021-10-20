use std::convert::{TryFrom, TryInto};
use std::fmt::Display;
use std::ops;
use std::result::Result;

pub trait TableIndex: TryFrom<usize> + TryInto<usize> + Clone + Copy + Display {}
impl<T> TableIndex for T
where
    T: TryFrom<usize> + TryInto<usize> + Clone + Copy + Display {}

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
