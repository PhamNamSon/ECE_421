use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;

pub fn question_7(data: &mut [i64]) -> &mut [i64] {
    let length = data.len();
    for i in 0..length {
        if let Some(min_index) = (i..length).min_by_key(|&j| &data[j]) {
            data.swap(i, min_index);
        }
    }
    data
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut l: Vec<i64> = (0..10000).map(|_| rng.gen_range(1..10000)).collect();
    c.bench_function("question 7: ", |b| {b.iter(|| {question_7(black_box(&mut l));})});
    
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);