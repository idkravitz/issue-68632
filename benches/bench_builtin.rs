#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
	use test::Bencher;
	use issue_68632::{max_subarray_bad, max_subarray_good};
	
	const N: usize = 1000000;

	#[bench]
	fn benchmark_bad(b: &mut Bencher) {
		b.iter(|| max_subarray_bad(&vec![0; N]));
	}


	#[bench]
	fn benchmark_good(b: &mut Bencher) {
		b.iter(|| max_subarray_good(&vec![0; N]));
	}
}
