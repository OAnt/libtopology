use super::list::{DoublyLinked, Linked, Result};
use super::table::Table;
use super::types::Container;

#[derive(PartialEq)]
pub enum TransformationType {
    Identity,
    Involution,
    Permutation,
}

pub trait CombinatorialMap<L: Linked>: Container<L>
{
    fn link_halfedges(& mut self, he_0: L::Type, he_1: L::Type, level: usize) -> Result<()> where Self: Sized{
        L::link(self, he_0, he_1, level)
    }

    fn get_transformation_type(&self, he: L::Type, transform: & dyn Fn(& dyn CombinatorialMap<L>, L::Type) -> L::Type) -> TransformationType where Self: Sized{
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

    fn is_manifold(&self, level: usize) -> bool where Self: Sized{
        for idx in 0..self.len() {
            let he: L::Type = L::try_from(idx).ok().unwrap();
            // if we could not convert we would not have been able to add the row already
            if self.get_transformation_type(he, & |cmap: & dyn CombinatorialMap<L>, he| {cmap[he][level].next()}) != TransformationType::Involution {
                return false;
            }
        }
        return true;
    }
}

pub trait UnlinkableCombinatorialMap<L: DoublyLinked>: CombinatorialMap<L> {
    fn unlink_halfedge(& mut self, he: L::Type, level: usize) where Self: Sized{
        L::unlink(self, he, level);
    }
}

impl<L: Linked> CombinatorialMap<L> for Table<L>{}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_triangle(cmap: & mut Table<u32>) -> [u32; 3] {
        let he_0 = cmap.add_row().unwrap();
        let he_1 = cmap.add_row().unwrap();
        let he_2 = cmap.add_row().unwrap();
        assert!(cmap.link_halfedges(he_0, he_1, 0).is_ok());
        assert!(cmap.link_halfedges(he_1, he_2, 0).is_ok());
        [he_0, he_1, he_2]
    }

    fn create_tetrahedron(cmap: & mut Table<u32>) -> u32 {
        let triangle_0 = create_triangle(cmap);
        let triangle_1 = create_triangle(cmap);
        assert!(cmap.link_halfedges(triangle_0[0], triangle_1[0], 1).is_ok());
        let triangle_2 = create_triangle(cmap);
        assert!(cmap.link_halfedges(triangle_0[1], triangle_2[0], 1).is_ok());
        assert!(cmap.link_halfedges(triangle_1[2], triangle_2[1], 1).is_ok());
        let triangle_3 = create_triangle(cmap);
        assert!(cmap.link_halfedges(triangle_0[2], triangle_3[0], 1).is_ok());
        assert!(cmap.link_halfedges(triangle_1[1], triangle_3[2], 1).is_ok());
        assert!(cmap.link_halfedges(triangle_2[2], triangle_3[1], 1).is_ok());
        return triangle_0[0];
    }

    #[test]
    fn test_create_triangular_face(){
        let mut cmap: Table<u32> = Table::new(2);
        let triangle = create_triangle(& mut cmap);
        let mut n_he: u32 = 0;
        for (i, he) in cmap.iter(triangle[0], |cmap, he| {cmap[he][0].next()}).enumerate() {
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
    
    fn next_in_face(cmap: & dyn CombinatorialMap<u32>, he: u32) -> u32{
        cmap[he][0]
    }
    
    fn next_over_edge(cmap: & dyn CombinatorialMap<u32>, he: u32) -> u32{
        cmap[he][1]
    }

    #[test]
    fn test_transformations(){
        let mut cmap: Table<u32> = Table::new(2);
        let triangle_0 = create_triangle(& mut cmap);
        let triangle_1 = create_triangle(& mut cmap);
        assert!(cmap.link_halfedges(triangle_0[0], triangle_1[0], 1).is_ok());
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
        for (i, he) in cmap.iter(tetrahedron, |cmap, he| {cmap[cmap[he][1].next()][0].next()}).enumerate() {
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
