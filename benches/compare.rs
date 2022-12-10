use cgisf_lib::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    // let (i_long, i_realistic) = input
    //     .split_once('\n')
    //     .expect("test_inputs should contain two lines, a long and realistic sentence.");
    c.bench_function("generate a random sentence", |b| {
        b.iter(|| {
            gen_sentence(black_box(SentenceConfigBuilder::random().build()));
        })
    });
    // c.bench_function("real world\tnoalloc cleanup", |b| {
    //     b.iter(|| string_cleanup_noalloc(black_box(i_realistic.to_string())))
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
