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

fn benchmark(c: &mut Criterion) {
    let vector_size: Vec<usize> = vec![1000, 10000, 100000, 1000000];
    for size in vector_size {
        let mut v: Vec<Person> = Vec::new();
        for i in 1..size {
            v.push(Person { age: i as u32 });
        }
        // bench 2 functions
        c.bench_function(&format!("using iter for size {}", size), |b| b.iter(|| average_age_over_30_using_iter(black_box(v.clone()))));
        c.bench_function(&format!("using par_iter for size {}", size), |b| b.iter(|| average_age_over_30_using_par_iter(black_box(v.clone()))));
    }
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
