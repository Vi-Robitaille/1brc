use criterion::{criterion_group, criterion_main, Criterion};
use onebrc_lib::{init_mmap, statemachine};

fn criterion_benchmark(c: &mut Criterion) {
    init_mmap();
    c.bench_function("statemachine::make_me_the_good_good", |b| b.iter(|| statemachine::make_me_the_good_good()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

