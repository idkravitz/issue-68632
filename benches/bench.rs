use issue_68632::{max_subarray_bad, max_subarray_good};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

const N: usize = 1000000;

fn benchmark_bad(c: &mut Criterion) {
    c.bench_function(&format!("max_subarray([..]) N = {}", N), |b| {
        b.iter(|| max_subarray_bad(black_box(&vec![0; N])))
    });
}


fn benchmark_good(c: &mut Criterion) {
    c.bench_function(&format!("max_subarray([..]) N = {}", N), |b| {
        b.iter(|| max_subarray_good(black_box(&vec![0; N])))
    });
}


criterion_group!(benches, benchmark_bad, benchmark_good);
criterion_main!(benches);