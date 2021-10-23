use std::ops::Index;
use std::result::Result;
use super::table::{Table, TableIndex};

pub trait CombinatorialMapIndex: TableIndex + Copy {}
impl<T> CombinatorialMapIndex for T
where
    T: TableIndex + Copy + Eq {}

#[derive(PartialEq)]
pub enum TransformationType {
    Identity,
    Involution,
    Permutation,
}

pub trait CombinatorialMap<T: CombinatorialMapIndex>: Index<T, Output=[T]> {
    fn new_halfedge(& mut self) -> Result<T, ()>;
    fn link_halfedges(& mut self, he_0: T, he_1: T, level: usize);
    fn get_transformation_type(&self, he: T, transform: & dyn Fn(& dyn CombinatorialMap<T>, T) -> T) -> TransformationType;
    fn is_manifold(&self, level: usize) -> bool;
}

impl<T: CombinatorialMapIndex> CombinatorialMap<T> for Table<T>{

    fn new_halfedge(& mut self) -> Result<T, ()> {
        self.add_row()
    }

    fn link_halfedges(& mut self, he_0: T, he_1: T, level: usize){
        let tmp: T = self[he_0][level];
        self[he_0][level] = he_1;
        self[he_1][level] = tmp;
    }

    fn get_transformation_type(&self, he: T, transform: & dyn Fn(& dyn CombinatorialMap<T>, T) -> T) -> TransformationType {
        let he_prime = transform(self, he);
        if he_prime == he {
           return TransformationType::Identity;
        }
        let he_second = transform(self, he_prime);
        if he == he_second {
            TransformationType::Involution
        }else{
            TransformationType::Permutation
        }
    }

    fn is_manifold(&self, level: usize) -> bool {
        for idx in 0..self.len() {
            let he: T = T::try_from(idx).ok().unwrap();
            // if we could not convert we would not have been able to add the row already
            if self.get_transformation_type(he, & |cmap: & dyn CombinatorialMap<T>, he| {cmap[he][level]}) != TransformationType::Involution {
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
        cmap.link_halfedges(triangle_1[2], triangle_2[1], 1);
        let triangle_3 = create_triangle(cmap);
        cmap.link_halfedges(triangle_0[2], triangle_3[0], 1);
        cmap.link_halfedges(triangle_1[1], triangle_3[2], 1);
        cmap.link_halfedges(triangle_2[2], triangle_3[1], 1);
        return triangle_0[0];
    }

    #[test]
    fn test_create_triangular_face(){
        let mut cmap: Table<u32> = Table::new(2);
        let triangle = create_triangle(& mut cmap);
        let mut n_he: u32 = 0;
        for (i, he) in cmap.iter(triangle[0], |cmap, he| {cmap[he][0]}).enumerate() {
            n_he += 1;
            match i {
                0 => assert!(he == triangle[0]),
                1 => assert!(he == triangle[1]),
                2 => assert!(he == triangle[2]),
                _ => assert!(false),
            }
        }
        assert!(n_he == 3);
    }

    #[test]
    fn test_transformations(){
        let mut cmap: Table<u32> = Table::new(2);
        let triangle_0 = create_triangle(& mut cmap);
        let triangle_1 = create_triangle(& mut cmap);
        cmap.link_halfedges(triangle_0[0], triangle_1[0], 1);
        let next_in_face = |cmap: & dyn CombinatorialMap<u32>, he: u32| {cmap[he][0]};
        let next_over_edge = |cmap: & dyn CombinatorialMap<u32>, he: u32| {cmap[he][1]};
        assert!(cmap.get_transformation_type(triangle_0[0], &next_in_face) == TransformationType::Permutation);
        assert!(cmap.get_transformation_type(triangle_0[0], &next_over_edge) == TransformationType::Involution);
        assert!(cmap.get_transformation_type(triangle_0[1], &next_in_face) == TransformationType::Permutation);
        assert!(cmap.get_transformation_type(triangle_0[1], &next_over_edge) == TransformationType::Identity);
        assert!(cmap.get_transformation_type(triangle_0[2], &next_in_face) == TransformationType::Permutation);
        assert!(cmap.get_transformation_type(triangle_0[2], &next_over_edge) == TransformationType::Identity);
        assert!(cmap.get_transformation_type(triangle_1[0], &next_in_face) == TransformationType::Permutation);
        assert!(cmap.get_transformation_type(triangle_1[0], &next_over_edge) == TransformationType::Involution);
        assert!(cmap.get_transformation_type(triangle_1[1], &next_in_face) == TransformationType::Permutation);
        assert!(cmap.get_transformation_type(triangle_1[1], &next_over_edge) == TransformationType::Identity);
        assert!(cmap.get_transformation_type(triangle_1[2], &next_in_face) == TransformationType::Permutation);
        assert!(cmap.get_transformation_type(triangle_1[2], &next_over_edge) == TransformationType::Identity);
    }

    #[test]
    fn test_manifold(){
        let mut cmap: Table<u32> = Table::new(2);
        create_tetrahedron(& mut cmap);
        assert!(cmap.is_manifold(1));
    }

    #[test]
    fn test_transformation(){
        let mut cmap: Table<u32> = Table::new(2);
        let tetrahedron = create_tetrahedron(& mut cmap);
        let mut n_he: u32 = 0;
        for (i, he) in cmap.iter(tetrahedron, |cmap, he| {cmap[cmap[he][1]][0]}).enumerate() {
            n_he += 1;
            match i {
                0 => assert!(he == 0),
                1 => assert!(he == 4),
                2 => assert!(he == 9),
                _ => assert!(false),
            }
        }
        assert!(n_he == 3);
    }
}
