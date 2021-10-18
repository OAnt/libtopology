use std::ops::Add;
pub trait UIndex: {
    const ZERO: Self;
    const ONE: Self;
    fn as_index(self) -> usize;
    fn from_index(val: usize) -> Self;
}
impl UIndex for u8 {
    const ZERO: u8 = 0;
    const ONE: u8 = 1;
    fn as_index(self) -> usize {
        self as usize
    }
    fn from_index(val: usize) -> Self{
        val as u8
    }
}
impl UIndex for u16 {
    const ZERO: u16 = 0;
    const ONE: u16 = 1;
    fn as_index(self) -> usize {
        self as usize
    }
    fn from_index(val: usize) -> Self{
        val as u16
    }
}
impl UIndex for u32 {
    const ZERO: u32 = 0;
    const ONE: u32 = 1;
    fn as_index(self) -> usize {
        self as usize
    }
    fn from_index(val: usize) -> Self{
        val as u32
    }
}
impl UIndex for u64 {
    const ZERO: u64 = 0;
    const ONE: u64 = 1;
    fn as_index(self) -> usize {
        self as usize
    }
    fn from_index(val: usize) -> Self{
        val as u64
    }
}
impl UIndex for usize {
    const ZERO: usize = 0;
    const ONE: usize = 1;
    fn as_index(self) -> usize{
        self
    }
    fn from_index(val: usize) -> Self{
        val
    }
}

pub struct UIndexIterator<T: UIndex> {
    pub start: T,
    pub end: T,
}

impl<T: UIndex + Add<Output = T> + Eq + Copy> Iterator for UIndexIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.start == self.end{
            None
        }else{
            let val: T = self.start;
            self.start = val + T::ONE;
            Some(val)
        }

    }
}
