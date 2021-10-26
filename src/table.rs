use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::iter;
use std::ops;
use std::result::Result;
use super::types::{TableElement, Container};

pub struct Table<L: TableElement>  {
    n: usize,
    m: usize,
    matrix: Vec<L>,
}

impl<L: TableElement> Table<L> {
    pub fn new(m: usize) -> Table<L> {
        Table {n: 0, m: m, matrix: Vec::new()}
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn add_row(& mut self) -> Result<L::Type, <L::Type as TryFrom<usize>>::Error> {
        let size: usize =  self.matrix.len();
        let row = self.n;
        // For some reason this make the compiled code faster
        assert!(row * self.m == size);
        let can_add = L::Type::try_from(row);
        match can_add {
            Ok(val) => {
                self.matrix.resize(size + self.m, L::new(val));
                self.n += 1;
                return Ok(val);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    pub fn add_multiple_rows<'a>(& 'a mut self, n: usize) -> Result<impl Iterator<Item=L::Type> + 'a, <L::Type as TryFrom<usize>>::Error> {
        let size: usize = self.matrix.len();
        let row = self.n;
        assert!(row * self.m == size);
        let can_add = {
            if self.n + n == 0 {
                L::Type::try_from(0)
            }else{
                L::Type::try_from(self.n + n - 1)
            }
        };
        match can_add {
            Ok(_v) => {
                self.n += n;
                Ok((row..self.n).map( move |idx| {
                    // we already checked the bigger index can be converted
                    let val = L::Type::try_from(idx).ok().unwrap();
                    self.matrix.resize(self.matrix.len() + self.m, L::new(val));
                    val
                }))
            }
            Err(e) =>  {
                Err(e)
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
}

impl<L: TableElement> ops::Index<L::Type> for Table<L>{
    type Output = [L];

    fn index(&self, row: L::Type) -> &[L] {
        let can_access: Result<usize, <L::Type as TryInto<usize>>::Error> = row.try_into();
        match can_access {
            Ok(row_as_usize) => {
                let pos = row_as_usize * self.m;
                &self.matrix[pos..pos+self.m]
            },
            Err(_) => panic!("{} cannot be used as an index", row)
        }
    }
}

impl<L: TableElement> ops::IndexMut<L::Type> for Table<L>{
    fn index_mut(& mut self, row: L::Type) -> & mut [L] {
        let can_access: Result<usize, <L::Type as TryInto<usize>>::Error> = row.try_into();
        match can_access {
            Ok(row_as_usize) => {
                let pos = row_as_usize * self.m;
                & mut self.matrix[pos..pos+self.m]
            },
            Err(_) => panic!("{} cannot be used as an index", row)
        }
    }
}

impl<T: TableElement> Container<T> for Table<T> {}

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
        assert!(r.is_ok());
        assert!(t[0][0] == 0);
        assert!(t[0][1] == 0);
        assert!(t[0][2] == 0);
    }

    #[test]
    fn test_add_multiple_rows(){
        let mut t: Table<i8> = Table::new(2);
        let mut result: Vec<i8> = Vec::new();
        match t.add_multiple_rows(128) {
            Ok(iter) => {
                for (i, j) in iter.enumerate() {
                    assert!(i == j as usize);
                    result.push(j);
                }
            }
            Err(_e) => {
                assert!(false);
            }
        };
        assert!(result.len() == 128);
        for r in result {
            assert!(t[r][0] == r);
            assert!(t[r][0] == r);
        }
    }

    #[test]
    fn test_add_multiple_rows_fails(){
        let mut t: Table<i8> = Table::new(2);
        match t.add_multiple_rows(129) {
            Ok(_iter) => assert!(false),
            Err(_e) => assert!(true)
        };
    }

    #[test]
    fn test_add_multiple_rows_2(){
        let mut t: Table<i8> = Table::new(2);
        match t.add_multiple_rows(0) {
            Ok(iter) => {
                for _i in iter {
                    assert!(false);
                }
            }
            Err(_e) => assert!(false)
        };
    }

    #[test]
    fn test_edit_row(){
        let mut t: Table<u32> = Table::new(3);
        let r = t.add_row();
        assert!(r.is_ok());
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
            assert!(r.is_ok());
            assert!(t[i][0] == i);
            assert!(t[i][1] == i);
            assert!(t[i][2] == i);
        }
        let r = t.add_row(); 
        assert!(r.is_err());
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
        assert!(r.is_err());
        let r = t.add_row();
        assert!(r.is_err());
        assert!(t[0][t.m] == 1);
    }

    #[test]
    #[should_panic]
    fn test_get_row_fails(){
        let t: Table<u32>  = Table::new(3);
        let _val: u32 = t[2][0];
    }
}
