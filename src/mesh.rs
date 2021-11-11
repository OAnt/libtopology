use std::convert::{TryFrom, TryInto};
use std::result;
use super::cmap::CombinatorialMap;
use super::list::Linked;
use super::table::Table;
use super::types::{TableIndex, ConversionResult, TableElement, Container};

trait Feature<T: TableIndex>{
    fn halfedges(&self) -> Option<T>;
    fn set_halfedge(&mut self, he: T);
}

trait Vertex<T: TableIndex>: Feature<T>{}
trait Face<T: TableIndex>: Feature<T>{}

trait Mesh<L: Linked, V: Vertex<L::Type>, F: Face<L::Type>>
{
    fn new(n_dimensions: usize) -> Self;
    fn add_vertex(&mut self, vertex: V) -> ConversionResult<L::Type>;
    fn add_face(&mut self, vertices: &[L::Type]) -> ConversionResult<L::Type>;
}

struct HalfEdge<T: TableIndex>{
    face: T,
    vertex: T
}

struct SimpleMesh<L: Linked, V: Vertex<L::Type>, F: Face<L::Type>> {
    table: Table<L>,
    halfedges: Vec<HalfEdge<L::Type>>,
    vertices: Vec<V>,
    faces: Vec<F>
}

fn populate_face<L: Linked, V: Vertex<L::Type>>(
    fi: L::Type,
    vertices: &mut Vec<V>,
    indices: &[L::Type],
    halfedges: &mut Vec<HalfEdge<L::Type>>,
    table: &mut dyn Container<L>,
    darts: &mut dyn Iterator<Item=L::Type>) -> ConversionResult<L::Type>
{
    let mut can_link: Option<L::Type> = None;
    let mut first_da: Option<L::Type> = None;
    for (i, da) in darts.enumerate() {
        // linking vertices in the face
        match can_link {
            Some(prev_da) => {
                // Not supposed to fail, we just created them and this function is private
                let was_linked = Linked::link(table, prev_da, da, 0);
                assert!(was_linked.is_ok())
            },
            None => {
                first_da = Some(da);
            }
        }
        // the is private we just created as many rows as there are vertices
        assert!(i < indices.len());
        let vi = indices[i];
        halfedges.push(HalfEdge{face: fi, vertex: vi});
        can_link = Some(da);
    }
    match first_da {
        Some(da) => Ok(da),
        None => panic!("cannot create a face without vertices")
    }
}

impl<L: Linked, V: Vertex<L::Type>, F: Face<L::Type>> Mesh<L, V, F> for SimpleMesh<L, V, F>
{
    fn new(n_dimensions: usize) -> Self {
        // I am using two columns to store corresponding vertex and face indices
        SimpleMesh{table: Table::new(n_dimensions), halfedges: Vec::new(), vertices: Vec::new(), faces: Vec::new()}
    }

    fn add_vertex(&mut self, vertex: V) -> ConversionResult<L::Type> {
        let n = self.vertices.len();
        let can_add = L::Type::try_from(n);
        match can_add {
            Ok(vi) => {
                self.vertices.push(vertex);
                Ok(vi)
            },
            e => {
                e
            }
        }
    }

    fn add_face(&mut self, vertices: &[L::Type]) -> ConversionResult<L::Type> {
        // ensuring we can create everything first
        // before doing anything to the topology
        let n = self.faces.len();
        let can_add_face = L::Type::try_from(n);
        match can_add_face {
            Ok(fi) => {
                let can_add_darts = self.table.add_multiple_rows(vertices.len());
                match can_add_darts {
                    Ok((container, mut iterator)) => {
                        populate_face(fi, &mut self.vertices, vertices, &mut self.halfedges, container, &mut iterator)
                    },
                    Err(e) => {
                        Err(e)
                    }
                }
            },
            e => e
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestFeature<T: TableIndex> {
        he: Option<T>,
    }

    impl<T: TableIndex> Feature<T> for TestFeature<T> {
        fn halfedges(& self) -> Option<T> {
            self.he
        }
        fn set_halfedge(&mut self, he: T) {
            self.he = Some(he);
        }
    }
    impl<T: TableIndex> Vertex<T> for TestFeature<T> {}
    impl<T: TableIndex> Face<T> for TestFeature<T> {}

    #[test]
    fn test_add_face(){
        let mut mesh: SimpleMesh<u32, TestFeature<u32>, TestFeature<u32>> = SimpleMesh::new(1);
        let v0: u32 = mesh.add_vertex(TestFeature{he: None}).unwrap();
        let v1: u32 = mesh.add_vertex(TestFeature{he: None}).unwrap();
        let v2: u32 = mesh.add_vertex(TestFeature{he: None}).unwrap();
        let f0: u32 = mesh.add_face(&[v0, v1, v2]).unwrap();
        assert!(f0 == 0);
    }
}
