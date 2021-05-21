use criterion::{black_box, criterion_group, criterion_main, Criterion};
use unitest::testing::must;

fn must_benchmark(c: &mut Criterion) {
  c.bench_function("unitest::testing::must!", |b| {
    b.iter(|| must!(truthy: black_box(true)))
  });
}

criterion_group!(benches, must_benchmark);
criterion_main!(benches);
