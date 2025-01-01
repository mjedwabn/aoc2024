use criterion::{criterion_group, criterion_main, Criterion};
use day01::day17::Computer;

fn run(i: u64) {
  let mut computer = Computer::new(0, 0, 0, vec![2,4,1,5,7,5,1,6,0,3,4,6,5,5,3,0]);
  for a in 0..i {
    computer.reset(a, 0, 0);
    computer.run();
  }
}

fn criterion_benchmark(c: &mut Criterion) {
  let mut group = c.benchmark_group("bruteforce");
  group.bench_with_input("100k", &100_000, |b, i| b.iter(|| {
    run(*i);
  }));
  group.bench_with_input("1M", &1_000_000, |b, i| b.iter(|| {
    run(*i);
  }));
  group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);