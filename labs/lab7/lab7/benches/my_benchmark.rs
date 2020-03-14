use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

#[path = "../src/part2.rs"]
mod part2;

fn bench_non_parallel(c: &mut Criterion) {
    let mut group = c.benchmark_group("not_parallel");

    for size in [1000, 10000, 100000, 1000000].iter() {
        //        group.throughput(Throughput::Bytes(*tree_size));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut v: Vec<part2::Person> = Vec::new();
                for i in 1..size {
                    v.push(part2::Person { age: i });
                }
                part2::average_age(&v);
            })
        });
    }
    group.finish();
}

fn bench_parallel(c: &mut Criterion) {
    let mut group = c.benchmark_group("parallel");

    for size in [1000, 10000, 100000, 1000000].iter() {
        //        group.throughput(Throughput::Bytes(*tree_size));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut v: Vec<part2::Person> = Vec::new();
                for i in 1..size {
                    v.push(part2::Person { age: i });
                }
                part2::average_age_parallel(&v);
            })
        });
    }
    group.finish();
}

criterion_group!(benches, bench_non_parallel, bench_parallel);
criterion_main!(benches);
