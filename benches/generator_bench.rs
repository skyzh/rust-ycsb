use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::SmallRng;
use rand::SeedableRng;
use ycsb::generator::{
    AcknowledgedCounterGenerator, ConstantGenerator, Generator, UniformLongGenerator,
    ZipfianGenerator,
};

fn bench_constant_generator(c: &mut Criterion) {
    let constant = ConstantGenerator::new(23333);
    let mut rng = SmallRng::from_entropy();
    c.bench_function("constant", |b| {
        b.iter(|| black_box(constant.next_value(&mut rng)))
    });
}

fn bench_uniform_generator(c: &mut Criterion) {
    let uniform = UniformLongGenerator::new(0, 10000);
    let mut rng = SmallRng::from_entropy();
    c.bench_function("uniform", |b| b.iter(|| uniform.next_value(&mut rng)));
}

fn bench_zipfian_generator(c: &mut Criterion) {
    let zipfian = ZipfianGenerator::from_range(0, 10000);
    let mut rng = SmallRng::from_entropy();
    c.bench_function("zipfian", |b| b.iter(|| zipfian.next_value(&mut rng)));
}

fn bench_acknowledged_counter_generator(c: &mut Criterion) {
    let counter = AcknowledgedCounterGenerator::new(1);
    let mut rng = SmallRng::from_entropy();
    c.bench_function("acknowledged counter", |b| {
        b.iter(|| {
            counter.acknowledge(counter.next_value(&mut rng));
            counter.last_value();
        })
    });
}

criterion_group!(
    benches,
    bench_constant_generator,
    bench_uniform_generator,
    bench_zipfian_generator,
    bench_acknowledged_counter_generator
);
criterion_main!(benches);
