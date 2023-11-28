use criterion::{criterion_group, criterion_main, Criterion};

fn fibonacci() {
    let mut my_vec = (1..500000000).rev().collect::<Vec<i32>>();
    my_vec.sort();
}

fn criterion_benchmark(c: &mut Criterion) {
    let _count = 1;
    c.bench_function("fib 20", |b| b.iter(|| fibonacci()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
