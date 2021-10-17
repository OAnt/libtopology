pub trait UIndex {
    fn as_index(self) -> usize;
}
impl UIndex for u8 {
    fn as_index(self) -> usize {
        self as usize
    }
}
impl UIndex for u16 {
    fn as_index(self) -> usize {
        self as usize
    }
}
impl UIndex for u32 {
    fn as_index(self) -> usize {
        self as usize
    }
}
impl UIndex for u64 {
    fn as_index(self) -> usize {
        self as usize
    }
}
impl UIndex for usize {
    fn as_index(self) -> usize{
        self
    }
}
