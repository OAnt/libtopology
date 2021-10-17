use super::types::UIndex;
use super::table::Table;

pub struct CombinatorialMap<T: UIndex + Clone + Copy> {
    table: Table<T>,
}

struct CombinatorialMapIterator<'a, T: UIndex + Clone + Copy> {
    cmap: &'a CombinatorialMap<T>,
    level: usize,
    active: T,
    end: T,
    done: bool,
}

impl<'a, T: UIndex + Clone + Copy + Eq> Iterator for CombinatorialMapIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.done {
            return None;
        }else{
            let val = self.active;
            self.active = self.cmap.table[val][self.level];
            self.done = self.active == self.end;
            return Some(val);
        }
    }
}

impl<T: UIndex + Clone + Copy + Eq> CombinatorialMap<T> {
    fn new(dimension: usize) -> CombinatorialMap<T> {
        CombinatorialMap {table: Table::new(dimension)}
    }

    fn new_halfedge(& mut self) -> T {
        let halfedge = self.table.len(); 
        let _halfedge = self.table.add_row(halfedge);
        assert!(halfedge == T::from_index(_halfedge));
        halfedge
    }

    fn link_halfedges(& mut self, he_0: T, he_1: T, level: usize){
        let tmp: T = self.table[he_0][level];
        self.table[he_0][level] = he_1;
        self.table[he_1][level] = tmp;
    }

    fn iter(&self, he_0: T, level: usize) -> CombinatorialMapIterator<'_, T> {
        CombinatorialMapIterator {cmap: self, level: level, active: he_0, end: he_0, done: false}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_triangular_face(){
        let mut cmap: CombinatorialMap<u32> = CombinatorialMap::new(2);
        let he_0 = cmap.new_halfedge();
        let he_1 = cmap.new_halfedge();
        let he_2 = cmap.new_halfedge();
        cmap.link_halfedges(he_0, he_1, 0);
        cmap.link_halfedges(he_1, he_2, 0);
        for (i, he) in cmap.iter(he_0, 0).enumerate() {
            match i {
                0 => assert!(he == he_0),
                1 => assert!(he == he_1),
                2 => assert!(he == he_2),
                _ => assert!(false),
            }
        }
    }
}
