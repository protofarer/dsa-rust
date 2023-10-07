use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::Rng;
use rust::sort_bubble::{sort_bubble, sort_bubble_inplace};
use rust::sort_merge::sort_merge;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sort_merge", |b| {
        b.iter(|| sort_merge(black_box(&[1, 2]), black_box(true)))
    });
}

pub fn bench_bubblesorts(c: &mut Criterion) {
    let mut group = c.benchmark_group("Bubble Sort");

    // generate test input variations:

    // Vary sizes
    let sizes = [10, 100, 1000];
    for &size in &sizes {
        let mut rng = rand::thread_rng();

        // Random
        let data_random: Vec<u64> = (0..size).map(|_| rng.gen()).collect();

        // Sorted
        let data_sorted: Vec<u64> = {
            let mut sorted = data_random.clone();
            sorted.sort();
            sorted
        };

        // Reverse sorted
        let data_sorted_reversed: Vec<u64> = {
            let mut reversed = data_sorted.clone();
            reversed.reverse();
            reversed
        };

        //? CSDR Vary types: strings, signed integers, floats
        group.bench_with_input(
            BenchmarkId::new("In-Place-Random", size),
            &data_random,
            |b, v| {
                let mut data = v.clone();
                b.iter(|| sort_bubble_inplace(&mut data, true))
            },
        );
        group.bench_with_input(
            BenchmarkId::new("Out-Of-Place-Random", size),
            &data_random,
            |b, v| b.iter(|| sort_bubble(v, true)),
        );

        // Optionally, benchmark with sorted vector
        group.bench_with_input(
            BenchmarkId::new("In-Place-Sorted", size),
            &data_sorted,
            |b, v| {
                let mut data = v.clone();
                b.iter(|| sort_bubble_inplace(&mut data, true))
            },
        );
        group.bench_with_input(
            BenchmarkId::new("Out-Of-Place-Sorted", size),
            &data_sorted,
            |b, v| b.iter(|| sort_bubble(v, true)),
        );

        // Optionally, benchmark with reversed vector
        group.bench_with_input(
            BenchmarkId::new("In-Place-Reversed", size),
            &data_sorted_reversed,
            |b, v| {
                let mut data = v.clone();
                b.iter(|| sort_bubble_inplace(&mut data, true))
            },
        );
        group.bench_with_input(
            BenchmarkId::new("Out-Of-Place-Reversed", size),
            &data_sorted_reversed,
            |b, v| b.iter(|| sort_bubble(v, true)),
        );
    }

    // TODO generate a bunch of different vectors
    // for i in [20u64, 21u64].iter() {
    //     group.bench_with_input(BenchmarkId::new("In-Place", i), i, |b, i| {
    //         b.iter(|| sort_bubble_inplace(*i, true))
    //     });
    //     group.bench_with_input(BenchmarkId::new("Out-Of-Place", i), i, |b, i| {
    //         b.iter(|| sort_bubble(*i, true))
    //     });
    // }
}

criterion_group!(benches, bench_bubblesorts);
// criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
