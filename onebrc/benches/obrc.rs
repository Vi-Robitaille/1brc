use criterion::{criterion_group, criterion_main, Criterion};
use onebrc_lib::{init_mmap, statemachine};

fn criterion_benchmark_1_000_000(c: &mut Criterion) {
    init_mmap(Some("./measurements_1_000_000.txt"));
    c.bench_function("statemachine::make_me_the_good_good 1_000_000", |b| b.iter(|| statemachine::make_me_the_good_good(false)));
}
    
fn criterion_benchmark_1_000_000_000(c: &mut Criterion) {
    init_mmap(None);
    c.bench_function("statemachine::make_me_the_good_good 1_000_000_000", |b| b.iter(|| statemachine::make_me_the_good_good(false)));
}

criterion_group!(benches, criterion_benchmark_1_000_000, criterion_benchmark_1_000_000_000);
criterion_main!(benches);

