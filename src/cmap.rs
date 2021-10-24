use std::ops::Index;
use std::result::Result;
use super::list::Linked;
use super::table::Table;
use super::types::TableElement;

//#[derive(Debug, Copy, Clone, PartialEq, Eq)]
//struct Pair<T: TableIndex> {
    //fwd: T,
    //bwd: T,
//}

//impl<T: TableIndex> fmt::Display for Pair<T> {
    //fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //write!(f, "{} {}", self.fwd, self.bwd)
    //}
//}

//impl<T: TableIndex> convert::TryFrom<usize> for Pair<T> {
    //type Error = <T as convert::TryFrom<usize>>::Error;
    //fn try_from(val: usize) -> Result<Self, Self::Error> {
        //let cval = T::try_from(val);
        //match cval {
            //Ok(t) => Ok(Pair{fwd: t, bwd: t}),
            //Err(e) => Err(e)
        //}
    //}
//}

//impl<T: TableIndex> convert::TryInto<usize> for Pair<T> {
    //type Error = <T as convert::TryInto<usize>>::Error;
    //fn try_into(self) -> Result<usize, Self::Error> {
        //self.fwd.try_into()
    //}
//}

//impl<T: TableIndex> Linkable for Pair<T> {
    //type Input = T;
    //fn link(table: & mut Table<Pair<T>>, lhs: T, rhs: T, level: usize){
        //let next: Pair<T> = table[lhs][level];
    //}
//}

#[derive(PartialEq)]
pub enum TransformationType {
    Identity,
    Involution,
    Permutation,
}

pub trait CombinatorialMap<L: Linked>: Index<<L as TableElement>::Type, Output=[L]>
{
    fn link_halfedges(& mut self, he_0: L::Type, he_1: L::Type, level: usize);
    fn get_transformation_type(&self, he: L::Type, transform: & dyn Fn(& dyn CombinatorialMap<L>, L::Type) -> L::Type) -> TransformationType;
    fn is_manifold(&self, level: usize) -> bool;
}

impl<L: Linked> CombinatorialMap<L> for Table<L>
{

    fn link_halfedges(& mut self, he_0: L::Type, he_1: L::Type, level: usize){
        L::link(self, he_0, he_1, level);
    }

    fn get_transformation_type(&self, he: L::Type, transform: & dyn Fn(& dyn CombinatorialMap<L>, L::Type) -> L::Type) -> TransformationType {
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
            let he: L::Type = L::try_from(idx).ok().unwrap();
            // if we could not convert we would not have been able to add the row already
            if self.get_transformation_type(he, & |cmap: & dyn CombinatorialMap<L>, he| {cmap[he][level].next()}) != TransformationType::Involution {
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
        let he_0 = cmap.add_row().unwrap();
        let he_1 = cmap.add_row().unwrap();
        let he_2 = cmap.add_row().unwrap();
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
        cmap.link_halfedges(triangle_0[0], triangle_1[0], 1);
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

    #[test]
    fn test_two_way(){
        let mut fwd: Table<u32> = Table::new(2);
        let mut bwd: Table<u32> = Table::new(2);
        let mut prev_he_fwd = None;
        let mut prev_he_bwd = None;
        for _i in 0..10 {
            let he_fwd = fwd.add_row().unwrap();
            let he_bwd = bwd.add_row().unwrap();
            match prev_he_fwd {
                Some(_prev_he) => fwd.link_halfedges(_prev_he, he_fwd, 0),
                None => {},
            }
            match prev_he_bwd {
                Some(_prev_he) => bwd.link_halfedges(he_bwd, _prev_he, 0),
                None => {},
            }
            prev_he_fwd = Some(he_fwd);
            prev_he_bwd = Some(he_bwd);
        }
        println!("{}", fwd);
        println!("{}", bwd);
    }
}
