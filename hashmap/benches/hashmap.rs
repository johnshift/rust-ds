use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hashmap::exercise::{e1_hashmap1, e1_hashmap2, e1_manual_array, e1_naive};

pub fn criterion_benchmark(c: &mut Criterion) {
  // c.bench_function("e1_naive", |b| {
  //   b.iter(|| e1_naive(black_box(vec![2, 5, 1, 2, 3, 5, 1, 2, 4])))
  // });

  // c.bench_function("e1_manual_array", |b| {
  //   b.iter(|| e1_manual_array(black_box(vec![2, 5, 1, 2, 3, 5, 1, 2, 4])))
  // });

  // c.bench_function("e1_hashmap1", |b| {
  //   b.iter(|| e1_hashmap1(black_box(vec![2, 5, 1, 2, 3, 5, 1, 2, 4])))
  // });

  // c.bench_function("e1_hashmap2", |b| {
  //   b.iter(|| e1_hashmap2(black_box(vec![2, 5, 1, 2, 3, 5, 1, 2, 4])))
  // });

  let huge_num = 10_000;

  c.bench_function("e1_naive_100k", |b| {
    b.iter(|| e1_naive(black_box((0..huge_num).chain(0..5).collect())))
  });

  c.bench_function("e1_manual_array_100k", |b| {
    b.iter(|| e1_manual_array(black_box((0..huge_num).chain(0..5).collect())))
  });

  c.bench_function("e1_hashmap1_100k", |b| {
    b.iter(|| e1_hashmap1(black_box((0..huge_num).chain(0..5).collect())))
  });

  c.bench_function("e1_hashmap2_100k", |b| {
    b.iter(|| e1_hashmap2(black_box((0..huge_num).chain(0..5).collect())))
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
