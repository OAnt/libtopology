use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::iter;
use std::ops;
use std::result::Result;
use super::types::{TableElement, TableIndex};

pub struct Table<L: TableElement>  {
    m: usize,
    matrix: Vec<L>,
}

impl<L: TableElement> Table<L> {
    pub fn new(m: usize) -> Table<L> {
        Table {m: m, matrix: Vec::new()}
    }

    pub fn len(&self) -> usize {
        self.matrix.len() / self.m
    }

    pub fn add_row(& mut self) -> Result<L::Type, ()> {
        let size: usize =  self.matrix.len();
        let row = size / self.m;
        let can_add = L::Type::try_from(row);
        match can_add {
            Ok(val) => {
                self.matrix.resize(size + self.m, L::new(val));
                return Ok(val);
            }
            Err(_e) => {
                return Err(());
            }
        }
    }

    pub fn iter<'a>(& 'a self, start: L::Type, transform: fn(& Self, L::Type) -> L::Type) -> impl Iterator<Item=L::Type> + 'a{
        iter::successors(Some(start), move |he: &L::Type| {
            let next_he = transform(self, *he);
            if next_he == start {
                None
            }else{
                Some(next_he)
            }
        })
    }

    fn get_row_pos(&self, row: L::Type) -> Result<usize, L::Type> {
        let _row: Result<usize, <L::Type as TryInto<usize>>::Error> = row.try_into();
        match _row {
            Ok(v) => Ok(v * self.m),
            Err(_e) => Err(row)
        }
    }
}

impl<L: TableElement> ops::Index<L::Type> for Table<L>{
    type Output = [L];

    fn index(&self, row: L::Type) -> &[L] {
        let can_access = self.get_row_pos(row);
        match can_access {
            Ok(pos) => &self.matrix[pos..pos+self.m],
            Err(orig) => panic!("{} cannot be used as an index", orig)
        }
    }
}

impl<L: TableElement> ops::IndexMut<L::Type> for Table<L>{
    fn index_mut(& mut self, row: L::Type) -> & mut [L] {
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

impl<L: TableElement> fmt::Display for Table<L>{

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_table(){
        let t: Table<Cell<u32>> = Table::new(3);
        assert!(t.m == 3 && t.matrix.len() == 0);
    }

    #[test]
    fn test_get_row(){
        let mut t: Table<Cell<u32>> = Table::new(3);
        let r = t.add_row();
        assert!(r != Err(()));
        assert!(t[0][0].fwd == 0);
        assert!(t[0][1].fwd == 0);
        assert!(t[0][2].fwd == 0);
    }

    #[test]
    fn test_edit_row(){
        let mut t: Table<Cell<u32>> = Table::new(3);
        let r = t.add_row();
        assert!(r != Err(()));
        t[0][0].fwd = 2;
        t[0][1].fwd = 3;
        t[0][2].fwd = 4;
        assert!(t.matrix[0].fwd == 2);
        assert!(t.matrix[1].fwd == 3);
        assert!(t.matrix[2].fwd == 4);
    }

    #[test]
    fn test_conversion_failure(){
        let mut t: Table<Cell<u8>> = Table::new(3);
        for i in 0..=255 {
            let r = t.add_row();
            assert!(r != Err(()));
            assert!(t[i][0].fwd == i);
            assert!(t[i][1].fwd == i);
            assert!(t[i][2].fwd == i);
        }
        let r = t.add_row(); 
        assert!(r == Err(()));
    }

    #[test]
    #[should_panic]
    fn test_index_failure(){
        let mut t: Table<Cell<i8>> = Table::new(3);
        t[-1][0].fwd = 1;
    }

    #[test]
    #[should_panic]
    fn test_get_row_out_of_bounds(){
        let mut t: Table<Cell<u32>> = Table::new(3);
        let r = t.add_row();
        assert!(r != Err(()));
        let r = t.add_row();
        assert!(r != Err(()));
        assert!(t[0][t.m].fwd == 1);
    }

    #[test]
    #[should_panic]
    fn test_get_row_fails(){
        let t: Table<Cell<u32>>  = Table::new(3);
        let _val: u32 = t[2][0].fwd;
    }
}
