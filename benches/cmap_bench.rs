use criterion::{criterion_group, criterion_main, Criterion};
use libtopology::cmap::CombinatorialMap;
use libtopology::table::Table;

fn next_helfedge_in_face(cmap: & Table<u32>, he: u32) -> u32{
    cmap[he][0]
}

fn create_polygon(cmap: & mut Table<u32>, size: u32) -> u32{

    let mut prev_he = None;
    for _i in 0..size {
        let he = cmap.new_halfedge().unwrap();
        match prev_he {
            Some(_prev_he) => cmap.link_halfedges(_prev_he, he, 0),
            None => {},
        }
        prev_he = Some(he);
    }
    prev_he.unwrap()
}

fn level_0_loop(cmap: & Table<u32>, _he: u32, n_he_tgt: u32){
    let stop = _he;
    let mut he = _he;
    let mut n_he = 0;
    loop{
        he = next_helfedge_in_face(cmap, he);
        n_he += 1;
        if he == stop {
            break;
        }
    }
    assert!(n_he == n_he_tgt);
}

fn level_0_iterator(cmap: & Table<u32>, _he: u32, n_he_tgt: u32){
    let mut n_he = 0;
    for _he in cmap.iter(_he, next_helfedge_in_face) {
        n_he += 1;
    }
    assert!(n_he == n_he_tgt);
}

fn level_0_benchmark(c: & mut Criterion){
    let mut cmap: Table<u32> = Table::new(2);
    let size = 100;
    let prev_he = create_polygon(& mut cmap, size);
    c.bench_function("Level-0-loop", |b| b.iter(|| level_0_loop(& cmap, prev_he, size)));
    c.bench_function("Level-0-Iterator", |b| b.iter(|| level_0_iterator(& cmap, prev_he, size)));
}

fn create_polygon_no_out(size: u32){
    let mut cmap: Table<u32> = Table::new(2);
    create_polygon(& mut cmap, size);
}

fn create_polygon_benchmark(c: & mut Criterion){
    c.bench_function("Create-polygon", |b| b.iter(|| create_polygon_no_out(100)));
}

criterion_group!(benches, level_0_benchmark, create_polygon_benchmark);
criterion_main!(benches);
