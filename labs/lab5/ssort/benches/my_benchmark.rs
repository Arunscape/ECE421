use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{thread_rng, Rng};
#[path = "../src/optimized.rs"]
mod optimized;
#[path = "../src/unoptimized.rs"]
mod unoptimized;

fn bench_unoptimized(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut l: Vec<i64> = (0..10000).map(|_| rng.gen_range(1, 10000)).collect();
    c.bench_function("unoptimized selection_sort", |b| {
        b.iter(|| unoptimized::selection_sort::<i64>(black_box(&mut l)))
    });
}

fn bench_optimized(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut l: Vec<i64> = (0..10000).map(|_| rng.gen_range(1, 10000)).collect();
    c.bench_function("optimized selection_sort", |b| {
        b.iter(|| optimized::selection_sort::<i64>(black_box(&mut l)))
    });
}

criterion_group!(benches, bench_unoptimized, bench_optimized);
criterion_main!(benches);
