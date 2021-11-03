use criterion::{criterion_group, criterion_main, Criterion};
use libtopology::cmap::CombinatorialMap;
use libtopology::table::Table;

fn next_dart_in_face(cmap: & Table<u32>, he: u32) -> u32{
    cmap[he][0]
}

fn create_polygon(cmap: & mut Table<u32>, size: u32) -> u32{

    let mut prev_da = None;
    for _i in 0..size {
        let da = cmap.add_row().unwrap();
        match prev_da {
            Some(_prev_da) => assert!(cmap.link_darts(_prev_da, da, 0).is_ok()),
            None => {},
        }
        prev_da = Some(da);
    }
    prev_da.unwrap()
}

fn level_0_loop(cmap: & Table<u32>, _da: u32, n_da_tgt: u32){
    let stop = _da;
    let mut da = _da;
    let mut n_da = 0;
    loop{
        da = next_dart_in_face(cmap, da);
        n_da += 1;
        if da == stop {
            break;
        }
    }
    assert!(n_da == n_da_tgt);
}

fn level_0_iterator(cmap: & Table<u32>, _da: u32, n_da_tgt: u32){
    let mut n_da = 0;
    for _da in cmap.iter(_da, next_dart_in_face) {
        n_da += 1;
    }
    assert!(n_da == n_da_tgt);
}

fn level_0_benchmark(c: & mut Criterion){
    let mut cmap: Table<u32> = Table::new(2);
    let size = 100;
    let prev_da = create_polygon(& mut cmap, size);
    c.bench_function("Level-0-loop", |b| b.iter(|| level_0_loop(& cmap, prev_da, size)));
    c.bench_function("Level-0-Iterator", |b| b.iter(|| level_0_iterator(& cmap, prev_da, size)));
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
