use cgisf_lib::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let config = SentenceConfigBuilder::default().build();
    c.bench_function("generate a sentence, default config", |b| {
        b.iter(|| {
            let _ = gen_sentence(black_box(config));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
