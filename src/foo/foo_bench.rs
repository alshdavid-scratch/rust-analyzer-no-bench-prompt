use divan::Bencher;
use divan::bench;

use super::foo::copy_from_slice;

#[bench]
fn benchmark(b: Bencher) {
  let src = (0..100).collect::<Vec<i32>>();
  let mut dst = vec![0; src.len()];

  b.bench_local(|| {
    copy_from_slice(&mut dst, &src);
  });
}
