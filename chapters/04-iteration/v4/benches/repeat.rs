// ANCHOR: all
use ch04_iteration_v4::repeat;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn benchmark_repeat(c: &mut Criterion) {
    c.bench_function("repeat", |b| b.iter(|| repeat(black_box('a'))));
}

criterion_group!(benches, benchmark_repeat);
criterion_main!(benches);
// ANCHOR_END: all
