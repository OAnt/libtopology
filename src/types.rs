pub trait UIndex {
    fn as_index(self) -> usize;
    fn from_index(val: usize) -> Self;
}
impl UIndex for u8 {
    fn as_index(self) -> usize {
        self as usize
    }
    fn from_index(val: usize) -> Self{
        val as u8
    }
}
impl UIndex for u16 {
    fn as_index(self) -> usize {
        self as usize
    }
    fn from_index(val: usize) -> Self{
        val as u16
    }
}
impl UIndex for u32 {
    fn as_index(self) -> usize {
        self as usize
    }
    fn from_index(val: usize) -> Self{
        val as u32
    }
}
impl UIndex for u64 {
    fn as_index(self) -> usize {
        self as usize
    }
    fn from_index(val: usize) -> Self{
        val as u64
    }
}
impl UIndex for usize {
    fn as_index(self) -> usize{
        self
    }
    fn from_index(val: usize) -> Self{
        val
    }
}
