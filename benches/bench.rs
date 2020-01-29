use issue_68632::{max_subarray_bad, max_subarray_good};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

const N: usize = 1000000;

fn benchmark_bad(c: &mut Criterion) {
//    const N: usize = 1000000;
    c.bench_function(&format!("max_subarray([..]) N = {}", N), |b| {
        b.iter(|| max_subarray_bad(black_box(&vec![0; N])))
    });
}


fn benchmark_good(c: &mut Criterion) {
//    const N: usize = 1000000;
    c.bench_function(&format!("max_subarray([..]) N = {}", N), |b| {
        b.iter(|| max_subarray_good(black_box(&vec![0; N])))
    });
}


criterion_group!(benches, benchmark_bad, benchmark_good);
criterion_main!(benches);

// use criterion::{black_box, criterion_group, criterion_main, Criterion};

// fn benchmark_recursive(c: &mut Criterion) {
//     const N: usize = 20000;
//     c.bench_function(&format!("max_subarray_recursive([.. N = {}])", N), |b| {
//         b.iter(|| max_subarray::max_subarray_recursive(black_box(&vec![0; N])))
//     });
//     c.bench_function(&format!("max_subarray_recursive_old([.. N = {}])", N), |b| {
//         b.iter(|| max_subarray::max_subarray_recursive_old(black_box(&vec![0; N])))
//     });
//     c.bench_function(&format!("max_subarray([.. N = {}])", N), |b| {
//         b.iter(|| max_subarray::max_subarray(black_box(&vec![0; N])))
//     });
// }

// fn benchmark_linear(c: &mut Criterion) {
//     const N: usize = 1000000;
//     c.bench_function(&format!("max_subarray([.. N = {}])", N), |b| {
//         b.iter(|| max_subarray::max_subarray(black_box(&vec![0; N])))
//     });
//     c.bench_function(&format!("max_subarray2([.. N = {}])", N), |b| {
//         b.iter(|| max_subarray::max_subarray2(black_box(&vec![0; N])))
//     });
//     c.bench_function(&format!("max_subarray3([.. N = {}])", N), |b| {
//         b.iter(|| max_subarray::max_subarray3(black_box(&vec![0; N])))
//     });
// }


// criterion_group!(benches/*, benchmark_recursive*/, benchmark_linear);
// criterion_main!(benches);
