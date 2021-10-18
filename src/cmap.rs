use super::table::Table;
use super::table::TableIndex;

trait CombinatorialMapIndex: TableIndex + Copy + Eq {}
impl<T> CombinatorialMapIndex for T
where
    T: TableIndex + Copy + Eq {}

enum TransformationType {
    Identity,
    Involution,
    Permutation,
}

trait CombinatorialMap<T: CombinatorialMapIndex> {
    fn new_halfedge(& mut self) -> T;
    fn link_halfedges(& mut self, he_0: T, he_1: T, level: usize);
    fn next_helfedge(&self, he: T, level: usize) -> T;
    fn iter(&self, he_0: T, level: usize) -> CombinatorialMapIterator<'_, T>;
    fn get_transformation_type(&self, he: T, level: usize) -> TransformationType;
}

struct CombinatorialMapIterator<'a, T: CombinatorialMapIndex> {
    cmap: &'a dyn CombinatorialMap<T>,
    level: usize,
    active: T,
    end: T,
    done: bool,
}

impl<'a, T: CombinatorialMapIndex> Iterator for CombinatorialMapIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.done {
            return None;
        }else{
            let val = self.active;
            self.active = self.cmap.next_helfedge(val, self.level);
            self.done = self.active == self.end;
            return Some(val);
        }
    }
}

impl<T: CombinatorialMapIndex> CombinatorialMap<T> for Table<T>{

    fn new_halfedge(& mut self) -> T {
        let halfedge = self.len(); 
        let _halfedge = self.add_row(halfedge);
        assert!(halfedge == T::from_index(_halfedge));
        halfedge
    }

    fn link_halfedges(& mut self, he_0: T, he_1: T, level: usize){
        let tmp: T = self[he_0][level];
        self[he_0][level] = he_1;
        self[he_1][level] = tmp;
    }

    fn iter(&self, he_0: T, level: usize) -> CombinatorialMapIterator<'_, T> {
        CombinatorialMapIterator {cmap: self, level: level, active: he_0, end: he_0, done: false}
    }

    fn next_helfedge(&self, he: T, level: usize) -> T {
        self[he][level]
    }

    fn get_transformation_type(&self, he: T, level: usize) -> TransformationType {
        let he_prime = self[he][level];
        if he_prime == he {
           return TransformationType::Identity;
        }
        let he_second = self[he_prime][level];
        if he == he_second {
            TransformationType::Involution
        }else{
            TransformationType::Permutation
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_triangular_face(){
        let mut cmap: Table<u32> = Table::new(2);
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

    #[test]
    fn test_identity(){
        let mut cmap: Table<u32> = Table::new(2);
        let he_0 = cmap.new_halfedge();
    }
}
