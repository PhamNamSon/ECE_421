use question_6::question_2;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut l: Vec<i64> = (0..10000).map(|_| rng.gen_range(1..10000)).collect();
    c.bench_function("your function: ", |b| {b.iter(|| {question_2(black_box(&mut l));})});
    
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);