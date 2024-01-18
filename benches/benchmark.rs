use criterion::{criterion_group, criterion_main, Criterion, BenchmarkGroup, measurement::Measurement};
use heap_n_sort::*;
use rand::random;

fn bench_group<T: Ord, M: Measurement>(g: &mut BenchmarkGroup<'_,M>, len: usize, g_name: &str, mut rnd: impl FnMut() -> T) {
    g.bench_function(&format!("stdquick_{g_name}"), |b| {
            let mut arr = (0..10u32.pow(len as _))
                .map(|_| rnd())
                .collect::<Vec<_>>();
            b.iter(|| arr.sort());
        })
        .bench_function(&format!("stdheap2_{g_name}"), |b| {
            let mut arr = (0..10u32.pow(len as _))
                .map(|_| rnd())
                .collect::<Vec<_>>();
            b.iter(|| std_heapsort(&mut arr, |x, y| x < y));
        })
        .bench_function(&format!("heap2_{g_name}"), |b| {
            let mut arr = (0..10u32.pow(len as _))
                .map(|_| rnd())
                .collect::<Vec<_>>();
            b.iter(|| heapsort::<_, 2>(&mut arr));
        })
        .bench_function(&format!("elifheap3_{g_name}"), |b| {
            let mut arr = (0..10u32.pow(len as _))
                .map(|_| rnd())
                .collect::<Vec<_>>();
            b.iter(|| heap_3_sort(&mut arr, |x, y| x < y));
        })
        .bench_function(&format!("heap3_{g_name}"), |b| {
            let mut arr = (0..10u32.pow(len as _))
                .map(|_| rnd())
                .collect::<Vec<_>>();
            b.iter(|| heapsort::<_, 3>(&mut arr));
        })
        .bench_function(&format!("heap4_{g_name}"), |b| {
            let mut arr = (0..10u32.pow(len as _))
                .map(|_| rnd())
                .collect::<Vec<_>>();
            b.iter(|| heapsort::<_, 4>(&mut arr));
        })
        .bench_function(&format!("heap5_{g_name}"), |b| {
            let mut arr = (0..10u32.pow(len as _))
                .map(|_| rnd())
                .collect::<Vec<_>>();
            b.iter(|| heapsort::<_, 5>(&mut arr));
        })
        .bench_function(&format!("heap6_{g_name}"), |b| {
            let mut arr = (0..10u32.pow(len as _))
                .map(|_| rnd())
                .collect::<Vec<_>>();
            b.iter(|| heapsort::<_, 6>(&mut arr));
        })
        .bench_function(&format!("heap7_{g_name}"), |b| {
            let mut arr = (0..10u32.pow(len as _))
                .map(|_| rnd())
                .collect::<Vec<_>>();
            b.iter(|| heapsort::<_, 7>(&mut arr));
        })
        .bench_function(&format!("heap8_{g_name}"), |b| {
            let mut arr = (0..10u32.pow(len as _))
                .map(|_| rnd())
                .collect::<Vec<_>>();
            b.iter(|| heapsort::<_, 8>(&mut arr));
        });
}

fn boxing<T>(f: fn()-> T) -> impl Fn() -> Box<T> {
    move || Box::new(f())
}

fn bench(c: &mut Criterion) {
    bench_group(&mut c.benchmark_group("u32_1M"), 6, "u32_1M", random::<u32>);
    bench_group(&mut c.benchmark_group("u32_1k"), 3, "u32_1k", random::<u32>);
    bench_group(&mut c.benchmark_group("u32_box_1M"), 6, "u32_box_1M", boxing(random::<u32>));
    bench_group(&mut c.benchmark_group("u32_box_1k"), 3, "u32_box_1k", boxing(random::<u32>));
    bench_group(&mut c.benchmark_group("Array4_1M"), 6, "Field4_1M", random::<[u32;4]>);
    bench_group(&mut c.benchmark_group("Array4_1k"), 3, "Field4_1k", random::<[u32;4]>);
    bench_group(&mut c.benchmark_group("Array4_box_1M"), 6, "Field4_box_1M", boxing(random::<[u32;4]>));
    bench_group(&mut c.benchmark_group("Array4_box_1k"), 3, "Field4_box_1k", boxing(random::<[u32;4]>));
    bench_group(&mut c.benchmark_group("Array32_1M"), 6, "Field32_1M", random::<[u32;32]>);
    bench_group(&mut c.benchmark_group("Array32_1k"), 3, "Field32_1k", random::<[u32;32]>);
    bench_group(&mut c.benchmark_group("Array32_box_1M"), 6, "Field32_box_1M", boxing(random::<[u32;32]>));
    bench_group(&mut c.benchmark_group("Array32_box_1k"), 3, "Field32_box_1k", boxing(random::<[u32;32]>));
}

criterion_group!(benches, bench);
criterion_main!(benches);
