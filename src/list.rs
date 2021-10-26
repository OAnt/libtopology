use std::fmt;
use super::table::Table;
use super::types::{TableIndex, TableElement};

pub trait Linked: TableElement{
    fn link(table: & mut Table<Self>, lhs: Self::Type, rhs: Self::Type, level: usize);
    fn next(&self) -> Self::Type;
}

macro_rules! linked_impl {
    ($($t:ty)*) => ($(
        impl Linked for $t {
            #[inline]
            fn link(table: & mut Table<$t>, lhs: $t, rhs: $t, level: usize){
                let tmp: $t = table[lhs][level];
                table[lhs][level] = rhs;
                table[rhs][level] = tmp;
            }

            #[inline]
            fn next(&self) -> $t {
                *self
            }
        }
    )*)
}

linked_impl! { usize u8 u16 u32 u64 i8 i16 i32 i64 isize }

pub trait DoublyLinked: Linked {
    fn unlink(table: & mut Table<Self>, rhs: Self::Type, level: usize);
}

#[derive(Copy, Clone)]
pub struct DoublyLinkedNode<T: TableIndex> {
    fwd: T,
    bwd: T,
}

impl<T: TableIndex> fmt::Display for DoublyLinkedNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.fwd)
    }
}

impl<T: TableIndex> TableElement for DoublyLinkedNode<T>{
    type Type = T;

    fn new(t: T) -> DoublyLinkedNode<T> {
        DoublyLinkedNode{fwd: t, bwd: t}
    }
}

impl<T: TableIndex> Linked for DoublyLinkedNode<T> {
    fn link(table: & mut Table<DoublyLinkedNode<T> >, lhs: T, rhs: T, level: usize){
        let next = table[lhs][level].fwd;
        table[next][level].bwd = rhs;
        table[rhs][level].fwd = table[lhs][level].fwd;
        table[rhs][level].bwd = lhs;
        table[lhs][level].fwd = rhs;
    }
    
    fn next(&self) -> T {
        self.fwd
    }
}

impl<T: TableIndex> DoublyLinked for DoublyLinkedNode<T> {
    fn unlink(table: & mut Table<DoublyLinkedNode<T> >, idx: T, level: usize){
        let elem = table[idx][level];
        table[elem.fwd][level].bwd = elem.bwd;
        table[elem.bwd][level].fwd = elem.fwd;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_integer_list () {
        let mut table: Table<u32> = Table::new(1);
        let cnt = table.add_multiple_rows(3).ok().unwrap().count();
        assert!(cnt == 3);
        u32::link(& mut table, 0, 1, 0);
        assert!(table[0][0] == 1);
        assert!(table[1][0] == 0);
        u32::link(& mut table, 0, 2, 0);
        assert!(table[0][0] == 2);
        assert!(table[1][0] == 0);
        assert!(table[2][0] == 1);
    }

    #[test]
    fn test_doubly_linked(){
        let mut table: Table<DoublyLinkedNode<u8>> = Table::new(1);
        let cnt = table.add_multiple_rows(3).ok().unwrap().count();
        assert!(cnt == 3);
        DoublyLinkedNode::link(& mut table, 0, 1, 0);
        assert!(table[0][0].fwd == 1);
        assert!(table[0][0].bwd == 1);
        assert!(table[1][0].fwd == 0);
        assert!(table[1][0].bwd == 0);
        DoublyLinkedNode::link(& mut table, 0, 2, 0);
        assert!(table[0][0].fwd == 2);
        assert!(table[0][0].bwd == 1);
        assert!(table[1][0].fwd == 0);
        assert!(table[1][0].bwd == 2);
        assert!(table[2][0].fwd == 1);
        assert!(table[2][0].bwd == 0);
        DoublyLinkedNode::unlink(& mut table, 1, 0);
        assert!(table[0][0].fwd == 2);
        assert!(table[0][0].bwd == 2);
        assert!(table[2][0].fwd == 0);
        assert!(table[2][0].bwd == 0);
        DoublyLinkedNode::link(& mut table, 2, 1, 0);
        assert!(table[0][0].fwd == 2);
        assert!(table[0][0].bwd == 1);
        assert!(table[1][0].fwd == 0);
        assert!(table[1][0].bwd == 2);
        assert!(table[2][0].fwd == 1);
        assert!(table[2][0].bwd == 0);
        DoublyLinkedNode::unlink(& mut table, 0, 0);
        assert!(table[1][0].fwd == 2);
        assert!(table[1][0].bwd == 2);
        assert!(table[2][0].fwd == 1);
        assert!(table[2][0].bwd == 1);
        DoublyLinkedNode::link(& mut table, 1, 0, 0);
        assert!(table[0][0].fwd == 2);
        assert!(table[0][0].bwd == 1);
        assert!(table[1][0].fwd == 0);
        assert!(table[1][0].bwd == 2);
        assert!(table[2][0].fwd == 1);
        assert!(table[2][0].bwd == 0);
    }

}
