// write criterion benchmarks for count_students, count_students_2, and count_students_leetcode

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use hungry_students::{count_students, count_students_leetcode};
use rand::Rng;

fn count_students_benchmark(c: &mut Criterion) {
    let sizes = [100, 200, 300, 400, 500, 600, 700, 800, 900, 1000];

    let mut group = c.benchmark_group("count_students");

    sizes.iter().for_each(|&size| {
        let mut rng = <rand::rngs::StdRng as rand::SeedableRng>::seed_from_u64(0);
        let students: Vec<i32> = (0..size).map(|_| rng.gen_range(0..2)).collect();
        let sandwiches: Vec<i32> = (0..size).map(|_| rng.gen_range(0..2)).collect();
        
        group.bench_with_input(
            criterion::BenchmarkId::new("mine", size),
            &size,
            |b, _| {
                b.iter(
                    || count_students(&students, &sandwiches),
                );
            },
        );
        group.bench_with_input(
            criterion::BenchmarkId::new("leetcode", size),
            &size,
            |b, _| {
                b.iter_batched(
                    || {
                        (students.clone(), sandwiches.clone())
                    },
                    |(students, sandwiches)| count_students_leetcode(students, sandwiches),
                    BatchSize::SmallInput,
                );
            },
        );
    });
    
    group.finish();
}


criterion_group!(
    benches,
    count_students_benchmark,
);
criterion_main!(benches);
