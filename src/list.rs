use std::fmt;
use std::result;
use super::types::{TableIndex, TableElement, Container};

pub type Result<T> = result::Result<T, LinkError>;

#[derive(Debug, Clone)]
pub struct LinkError;

pub trait Linked: TableElement{
    fn link(table: & mut dyn Container<Self>, lhs: Self::Type, rhs: Self::Type, level: usize) -> Result<()>;
    fn next(&self) -> Self::Type;
}

macro_rules! linked_impl {
    ($($t:ty)*) => ($(
        impl Linked for $t {
            #[inline]
            fn link(table: & mut dyn Container<$t>, lhs: $t, rhs: $t, level: usize) -> Result<()>{
                if(table[rhs][level] == rhs){
                    let tmp: $t = table[lhs][level];
                    table[lhs][level] = rhs;
                    table[rhs][level] = tmp;
                    Ok(())
                }else{
                    Err(LinkError)
                }
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
    fn unlink(table: & mut dyn Container<Self>, rhs: Self::Type, level: usize);
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
    fn link(table: & mut dyn Container<DoublyLinkedNode<T>>, lhs: T, rhs: T, level: usize) -> Result<()>{
        if table[rhs][level].fwd == rhs && table[rhs][level].bwd == rhs {
            let next = table[lhs][level].fwd;
            table[next][level].bwd = rhs;
            table[rhs][level].fwd = table[lhs][level].fwd;
            table[rhs][level].bwd = lhs;
            table[lhs][level].fwd = rhs;
            Ok(())
        } else {
            Err(LinkError)
        }
    }
    
    fn next(&self) -> T {
        self.fwd
    }
}

impl<T: TableIndex> DoublyLinked for DoublyLinkedNode<T> {
    fn unlink(table: & mut dyn Container<DoublyLinkedNode<T> >, idx: T, level: usize){
        let elem = table[idx][level];
        table[elem.fwd][level].bwd = elem.bwd;
        table[elem.bwd][level].fwd = elem.fwd;
        table[idx][level].fwd = idx;
        table[idx][level].bwd = idx;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::table::Table;
    
    #[test]
    fn test_integer_list () {
        let mut table: Table<u32> = Table::new(1);
        let cnt = table.add_multiple_rows(3).ok().unwrap().count();
        assert!(cnt == 3);
        assert!(u32::link(& mut table, 0, 1, 0).is_ok());
        assert!(table[0][0] == 1);
        assert!(table[1][0] == 0);
        assert!(u32::link(& mut table, 0, 2, 0).is_ok());
        assert!(table[0][0] == 2);
        assert!(table[1][0] == 0);
        assert!(table[2][0] == 1);
    }

    #[test]
    fn test_multiple_links(){
        let mut table: Table<u32> = Table::new(1);
        let cnt = table.add_multiple_rows(3).ok().unwrap().count();
        assert!(cnt == 3);
        assert!(u32::link(& mut table, 0, 1, 0).is_ok());
        assert!(u32::link(& mut table, 0, 1, 0).is_err());
        assert!(u32::link(& mut table, 1, 0, 0).is_err());
        assert!(u32::link(& mut table, 0, 2, 0).is_ok());
        assert!(u32::link(& mut table, 1, 0, 0).is_err());
        assert!(u32::link(& mut table, 0, 1, 0).is_err());
    }

    #[test]
    fn test_doubly_linked(){
        let mut table: Table<DoublyLinkedNode<u8>> = Table::new(1);
        let cnt = table.add_multiple_rows(3).ok().unwrap().count();
        assert!(cnt == 3);
        assert!(DoublyLinkedNode::link(& mut table, 0, 1, 0).is_ok());
        assert!(table[0][0].fwd == 1);
        assert!(table[0][0].bwd == 1);
        assert!(table[1][0].fwd == 0);
        assert!(table[1][0].bwd == 0);
        assert!(DoublyLinkedNode::link(& mut table, 0, 2, 0).is_ok());
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
        assert!(DoublyLinkedNode::link(& mut table, 2, 1, 0).is_ok());
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
        assert!(DoublyLinkedNode::link(& mut table, 1, 0, 0).is_ok());
        assert!(table[0][0].fwd == 2);
        assert!(table[0][0].bwd == 1);
        assert!(table[1][0].fwd == 0);
        assert!(table[1][0].bwd == 2);
        assert!(table[2][0].fwd == 1);
        assert!(table[2][0].bwd == 0);
    }

    #[test]
    fn test_multiple_links_2(){
        let mut table: Table<DoublyLinkedNode<u32>> = Table::new(1);
        let cnt = table.add_multiple_rows(3).ok().unwrap().count();
        assert!(cnt == 3);
        assert!(DoublyLinkedNode::link(& mut table, 0, 1, 0).is_ok());
        assert!(DoublyLinkedNode::link(& mut table, 0, 1, 0).is_err());
        assert!(DoublyLinkedNode::link(& mut table, 1, 0, 0).is_err());
        assert!(DoublyLinkedNode::link(& mut table, 0, 2, 0).is_ok());
        assert!(DoublyLinkedNode::link(& mut table, 1, 0, 0).is_err());
        assert!(DoublyLinkedNode::link(& mut table, 0, 1, 0).is_err());
    }

}
