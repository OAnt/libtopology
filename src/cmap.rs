use std::ops::Add;
use std::result::Result;
use super::table::{Table, TableIndex};

trait CombinatorialMapIndex: TableIndex + Copy + Eq {}
impl<T> CombinatorialMapIndex for T
where
    T: TableIndex + Copy + Eq {}

#[derive(PartialEq)]
enum TransformationType {
    Identity,
    Involution,
    Permutation,
}

trait CombinatorialMap<T: CombinatorialMapIndex> {
    fn new_halfedge(& mut self) -> Result<T, ()>;
    fn link_halfedges(& mut self, he_0: T, he_1: T, level: usize);
    fn next_helfedge(&self, he: T, level: usize) -> T;
    fn iter(&self, he_0: T, level: usize) -> CombinatorialMapIterator<'_, T>;
    fn get_transformation_type(&self, he: T, level: usize) -> TransformationType;
    fn is_manifold(&self, level: usize) -> bool;
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

impl<T: CombinatorialMapIndex + Add<Output = T> > CombinatorialMap<T> for Table<T>{

    fn new_halfedge(& mut self) -> Result<T, ()> {
        self.add_row()
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

    fn is_manifold(&self, level: usize) -> bool {
        for idx in (0..self.bare_len()).step_by(self.m) {
            // if we could not convert we would not have been able to add the row already
            if self.get_transformation_type(T::try_from(idx).ok().unwrap(), level) != TransformationType::Involution {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_triangle(cmap: & mut Table<u32>) -> [u32; 3] {
        let he_0 = cmap.new_halfedge().unwrap();
        let he_1 = cmap.new_halfedge().unwrap();
        let he_2 = cmap.new_halfedge().unwrap();
        cmap.link_halfedges(he_0, he_1, 0);
        cmap.link_halfedges(he_1, he_2, 0);
        [he_0, he_1, he_2]
    }

    fn create_tetrahedron(cmap: & mut Table<u32>) -> u32 {
        let triangle_0 = create_triangle(cmap);
        let triangle_1 = create_triangle(cmap);
        cmap.link_halfedges(triangle_0[0], triangle_1[0], 1);
        let triangle_2 = create_triangle(cmap);
        cmap.link_halfedges(triangle_0[1], triangle_2[0], 1);
        cmap.link_halfedges(triangle_1[1], triangle_2[1], 1);
        let triangle_3 = create_triangle(cmap);
        cmap.link_halfedges(triangle_0[2], triangle_3[1], 1);
        cmap.link_halfedges(triangle_1[2], triangle_3[2], 1);
        cmap.link_halfedges(triangle_2[2], triangle_3[0], 1);
        return triangle_0[0];
    }

    #[test]
    fn test_create_triangular_face(){
        let mut cmap: Table<u32> = Table::new(2);
        let triangle = create_triangle(& mut cmap);
        for (i, he) in cmap.iter(triangle[0], 0).enumerate() {
            match i {
                0 => assert!(he == triangle[0]),
                1 => assert!(he == triangle[1]),
                2 => assert!(he == triangle[2]),
                _ => assert!(false),
            }
        }
    }

    #[test]
    fn test_transformations(){
        let mut cmap: Table<u32> = Table::new(2);
        let triangle_0 = create_triangle(& mut cmap);
        let triangle_1 = create_triangle(& mut cmap);
        cmap.link_halfedges(triangle_0[0], triangle_1[0], 1);
        assert!(cmap.get_transformation_type(triangle_0[0], 0) == TransformationType::Permutation);
        assert!(cmap.get_transformation_type(triangle_0[0], 1) == TransformationType::Involution);
        assert!(cmap.get_transformation_type(triangle_0[1], 0) == TransformationType::Permutation);
        assert!(cmap.get_transformation_type(triangle_0[1], 1) == TransformationType::Identity);
        assert!(cmap.get_transformation_type(triangle_0[2], 0) == TransformationType::Permutation);
        assert!(cmap.get_transformation_type(triangle_0[2], 1) == TransformationType::Identity);
        assert!(cmap.get_transformation_type(triangle_1[0], 0) == TransformationType::Permutation);
        assert!(cmap.get_transformation_type(triangle_1[0], 1) == TransformationType::Involution);
        assert!(cmap.get_transformation_type(triangle_1[1], 0) == TransformationType::Permutation);
        assert!(cmap.get_transformation_type(triangle_1[1], 1) == TransformationType::Identity);
        assert!(cmap.get_transformation_type(triangle_1[2], 0) == TransformationType::Permutation);
        assert!(cmap.get_transformation_type(triangle_1[2], 1) == TransformationType::Identity);
    }
}
