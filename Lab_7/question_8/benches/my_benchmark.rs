use rayon::prelude::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[derive(Clone)]
struct Person {
    age: u32,
}

fn average_age_over_30_using_iter(v: Vec<Person>) -> f32 {
    let num_over_30 = v.iter().filter(|&x| x.age > 30).count() as f32;
    let sum_over_30: u32 = v.iter().map(|x| x.age).filter(|&x| x > 30).sum();
    let avg_over_30 = sum_over_30 as f32/ num_over_30;
    avg_over_30
}

fn average_age_over_30_using_par_iter(v: Vec<Person>) -> f32 {
    let num_over_30 = v.par_iter().filter(|&x| x.age > 30).count() as f32;
    let sum_over_30: u32 = v.par_iter().map(|x| x.age).filter(|&x| x > 30).sum();
    let avg_over_30 = sum_over_30 as f32/ num_over_30;
    avg_over_30
}

fn benchmark_for_iter(c: &mut Criterion) {
    let mut v: Vec<Person> = Vec::new();
    for i in 1..10000 {
        v.push(Person { age: i });
    }
    c.bench_function("using iter", |b| b.iter(|| average_age_over_30_using_iter(black_box(v.clone()))));
}

fn benchmark_for_par_iter(c: &mut Criterion) {
    let mut v: Vec<Person> = Vec::new();
    for i in 1..10000 {
        v.push(Person { age: i });
    }
    c.bench_function("using par_iter", |b| b.iter(|| average_age_over_30_using_par_iter(black_box(v.clone()))));
}

criterion_group!(benches, benchmark_for_iter, benchmark_for_par_iter);
criterion_main!(benches);
