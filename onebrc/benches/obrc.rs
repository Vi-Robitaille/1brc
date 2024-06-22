use criterion::{criterion_group, criterion_main, Criterion};
use onebrc_lib::{init_mmap, make_me_the_good_good, MMAP};
use std::time::Duration;

fn criterion_benchmark_1_000_000(c: &mut Criterion) {
    init_mmap(Some("./measurements-1_000_000.txt"));
    println!("Size of memory mapped file {:.2} MB", (MMAP.get().unwrap().len() as f32) / 1_048_576_f32);
    let mut group = c.benchmark_group("One million");
    group.measurement_time(Duration::from_secs(60));
    group.bench_function("make_me_the_good_good 1_000_000", |b| {
        b.iter(|| make_me_the_good_good(false))
    });
    group.finish();
}

fn criterion_benchmark_1_000_000_000(c: &mut Criterion) {
    init_mmap(Some("./measurements.txt"));
    println!("Size of memory mapped file {:.2} GB", (MMAP.get().unwrap().len() as f32) / 1_073_741_824_f32);
    let mut group = c.benchmark_group("One billion");
    group.measurement_time(Duration::from_secs(300));
    group.sample_size(30);
    group.bench_function("make_me_the_good_good 1_000_000_000", |b| {
        b.iter(|| make_me_the_good_good(false))
    });
    group.finish();
}

criterion_group!(
    benches,
    criterion_benchmark_1_000_000,
    // criterion_benchmark_1_000_000_000
    );
criterion_main!(benches);
