use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::SmallRng;
use rand::SeedableRng;
use ycsb::generator::{
    AcknowledgedCounterGenerator, ConstantGenerator, Generator, UniformLongGenerator,
    ZipfianGenerator,
};

fn bench_constant_generator(c: &mut Criterion) {
    let mut rng = SmallRng::from_entropy();
    let constant = ConstantGenerator::new(23333 as u64);
    c.bench_function("constant", |b| {
        b.iter(|| black_box(constant.next_value(&mut rng)))
    });

    let constant = black_box(Box::new(constant)) as Box<dyn Generator<u64>>;
    c.bench_function("constant boxed", |b| {
        b.iter(|| black_box(constant.next_value(&mut rng)))
    });
}

fn bench_uniform_generator(c: &mut Criterion) {
    let mut rng = SmallRng::from_entropy();
    let uniform = UniformLongGenerator::new(0, 10000);
    c.bench_function("uniform", |b| b.iter(|| uniform.next_value(&mut rng)));

    let uniform = black_box(Box::new(uniform)) as Box<dyn Generator<u64>>;
    c.bench_function("uniform boxed", |b| b.iter(|| uniform.next_value(&mut rng)));
}

fn bench_zipfian_generator(c: &mut Criterion) {
    let mut rng = SmallRng::from_entropy();
    let zipfian = ZipfianGenerator::from_range(0, 10000);
    c.bench_function("zipfian", |b| b.iter(|| zipfian.next_value(&mut rng)));

    let zipfian = black_box(Box::new(zipfian)) as Box<dyn Generator<u64>>;
    c.bench_function("zipfian boxed", |b| b.iter(|| zipfian.next_value(&mut rng)));
}

fn bench_acknowledged_counter_generator(c: &mut Criterion) {
    let mut rng = SmallRng::from_entropy();
    let counter = AcknowledgedCounterGenerator::new(1);
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
