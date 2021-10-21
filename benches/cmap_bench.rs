use criterion::{criterion_group, criterion_main, Criterion};
use libtopology::table::Table;
use libtopology::cmap::CombinatorialMap;

fn level_0_loop(cmap: & Table<u32>, _he: u32, n_he_tgt: u32){
    let stop = _he;
    let mut he = _he;
    let mut n_he = 0;
    loop{
        he = cmap[he][0];
        n_he += 1;
        if he == stop {
            break;
        }
    }
    assert!(n_he == n_he_tgt);
}

fn level_0_iterator(cmap: & Table<u32>, _he: u32, n_he_tgt: u32){
    let mut n_he = 0;
    for _he in cmap.iter(_he, 0) {
        n_he += 1;
    }
    assert!(n_he == n_he_tgt);
}

fn level_0_benchmark(c: & mut Criterion){
    let mut cmap: Table<u32> = Table::new(2);
    let mut prev_he = None;
    let n_he: u32 = 100;
    for _i in 0..n_he {
        let he = cmap.new_halfedge().unwrap();
        match prev_he {
            Some(_prev_he) => cmap.link_halfedges(_prev_he, he, 0),
            None => {},
        }
        prev_he = Some(he);
    }
    c.bench_function("Level 0 loop", |b| b.iter(|| level_0_loop(& cmap, prev_he.unwrap(), n_he)));
    c.bench_function("Level 0 Iterator", |b| b.iter(|| level_0_iterator(& cmap, prev_he.unwrap(), n_he)));
}

criterion_group!(benches, level_0_benchmark);
criterion_main!(benches);
