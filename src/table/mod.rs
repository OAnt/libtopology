use std::ops;

trait UIndex {}
impl UIndex for u8 {}
impl UIndex for u16 {}
impl UIndex for u32 {}
impl UIndex for u64 {}
impl UIndex for u128 {}
impl UIndex for usize {}

struct Table<T: UIndex + Clone>  {
    m: usize,
    matrix: Vec<T>,
}

impl<T: UIndex + Clone> Table<T> {
    fn add_column(& mut self, val: T) -> usize {
        let size: usize =  self.matrix.len();
        let index = size / self.m;
        self.matrix.resize(size + self.m, val);
        index
    }
}

impl<T: UIndex + Clone> ops::Index<T> for Table<T> {
    type Output<'a> = &'a[T];


}
