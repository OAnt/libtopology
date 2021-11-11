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
    fn link_darts(& mut self, da_0: L::Type, da_1: L::Type, level: usize) -> Result<()> where Self: Sized{
        L::link(self, da_0, da_1, level)
    }

    fn get_transformation_type(&self, da: L::Type, transform: & dyn Fn(& dyn CombinatorialMap<L>, L::Type) -> L::Type) -> TransformationType where Self: Sized{
        let da_prime = transform(self, da);
        if da_prime == da {
           return TransformationType::Identity;
        }
        let da_second = transform(self, da_prime);
        if da == da_second {
            TransformationType::Involution
        }else{
            TransformationType::Permutation
        }
    }

    fn is_manifold(&self, level: usize) -> bool where Self: Sized{
        for idx in 0..self.len() {
            let he: L::Type = L::try_from(idx).ok().unwrap();
            // if we could not convert we would not have been able to add the row already
            if self.get_transformation_type(he, & |cmap: & dyn CombinatorialMap<L>, da| {cmap[da][level].next()}) != TransformationType::Involution {
                return false;
            }
        }
        return true;
    }
}

pub trait UnlinkableCombinatorialMap<L: DoublyLinked>: CombinatorialMap<L> {
    fn unlink_dart(& mut self, da: L::Type, level: usize) where Self: Sized{
        L::unlink(self, da, level);
    }
}

impl<L: Linked> CombinatorialMap<L> for Table<L>{}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_triangle(cmap: & mut Table<u32>) -> [u32; 3] {
        let da_0 = cmap.add_row().unwrap();
        let da_1 = cmap.add_row().unwrap();
        let da_2 = cmap.add_row().unwrap();
        assert!(cmap.link_darts(da_0, da_1, 0).is_ok());
        assert!(cmap.link_darts(da_1, da_2, 0).is_ok());
        [da_0, da_1, da_2]
    }

    fn create_tetrahedron(cmap: & mut Table<u32>) -> u32 {
        let triangle_0 = create_triangle(cmap);
        let triangle_1 = create_triangle(cmap);
        assert!(cmap.link_darts(triangle_0[0], triangle_1[0], 1).is_ok());
        let triangle_2 = create_triangle(cmap);
        assert!(cmap.link_darts(triangle_0[1], triangle_2[0], 1).is_ok());
        assert!(cmap.link_darts(triangle_1[2], triangle_2[1], 1).is_ok());
        let triangle_3 = create_triangle(cmap);
        assert!(cmap.link_darts(triangle_0[2], triangle_3[0], 1).is_ok());
        assert!(cmap.link_darts(triangle_1[1], triangle_3[2], 1).is_ok());
        assert!(cmap.link_darts(triangle_2[2], triangle_3[1], 1).is_ok());
        return triangle_0[0];
    }

    #[test]
    fn test_create_triangular_face(){
        let mut cmap: Table<u32> = Table::new(2);
        let triangle = create_triangle(& mut cmap);
        let mut n_da: u32 = 0;
        for (i, da) in cmap.iter(triangle[0], |cmap, da| {cmap[da][0].next()}).enumerate() {
            n_da += 1;
            match i {
                0 => assert!(da == triangle[0]),
                1 => assert!(da == triangle[1]),
                2 => assert!(da == triangle[2]),
                _ => assert!(false),
            }
        }
        assert!(n_da == 3);
    }
    
    fn next_in_face(cmap: & dyn CombinatorialMap<u32>, da: u32) -> u32{
        cmap[da][0]
    }
    
    fn next_over_edge(cmap: & dyn CombinatorialMap<u32>, da: u32) -> u32{
        cmap[da][1]
    }

    #[test]
    fn test_transformations(){
        let mut cmap: Table<u32> = Table::new(2);
        let triangle_0 = create_triangle(& mut cmap);
        let triangle_1 = create_triangle(& mut cmap);
        assert!(cmap.link_darts(triangle_0[0], triangle_1[0], 1).is_ok());
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
        let mut n_da: u32 = 0;
        for (i, da) in cmap.iter(tetrahedron, |cmap, da| {cmap[cmap[da][1].next()][0].next()}).enumerate() {
            n_da += 1;
            match i {
                0 => assert!(da == 0),
                1 => assert!(da == 4),
                2 => assert!(da == 9),
                _ => assert!(false),
            }
        }
        assert!(n_da == 3);
    }
}
