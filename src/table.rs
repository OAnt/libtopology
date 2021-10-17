use std::ops;

use super::types::UIndex;

pub struct Table<T: UIndex + Clone>  {
    m: usize,
    matrix: Vec<T>,
}

impl<T: UIndex + Clone> Table<T> {
    pub fn new(m: usize) -> Table<T> {
        Table {m: m, matrix: Vec::new()}
    }

    pub fn len(&self) -> T {
        let n_row: usize = self.matrix.len() / self.m;
        let row: T = T::from_index(n_row);
        row
    }

    pub fn add_row(& mut self, val: T) -> usize {
        let size: usize =  self.matrix.len();
        let row = size / self.m;
        self.matrix.resize(size + self.m, val);
        row
    }

    fn get_row_pos(&self, row: T) -> usize {
        row.as_index() * self.m
    }
}

impl<T: UIndex + Clone> ops::Index<T> for Table<T>{
    type Output = [T];

    fn index(&self, row: T) -> &[T] {
        let pos: usize = self.get_row_pos(row);        
        &self.matrix[pos..pos+self.m]
    }
}

impl<T: UIndex + Clone> ops::IndexMut<T> for Table<T>{
    fn index_mut(& mut self, row: T) -> & mut [T] {
        let pos: usize = self.get_row_pos(row);        
        & mut self.matrix[pos..pos+self.m]
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
        t.add_row(1);
        assert!(t[0][0] == 1);
        assert!(t[0][1] == 1);
        assert!(t[0][2] == 1);
    }

    #[test]
    fn test_edit_row(){
        let mut t: Table<u32> = Table::new(3);
        t.add_row(1);
        t[0][0] = 2;
        t[0][1] = 3;
        t[0][2] = 4;
        assert!(t.matrix[0] == 2);
        assert!(t.matrix[1] == 3);
        assert!(t.matrix[2] == 4);
    }

    #[test]
    #[should_panic]
    fn test_get_row_out_of_bounds(){
        let mut t: Table<u32> = Table::new(3);
        t.add_row(1);
        t.add_row(2);
        assert!(t[0][t.m] == 1);
    }

    #[test]
    #[should_panic]
    fn test_get_row_fails(){
        let t: Table<u32>  = Table::new(3);
        let _val: u32 = t[2][0];
    }
}
