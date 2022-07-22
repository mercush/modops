use expmod::exponents::{plain_expmod_testing, bigint_modpow_testing, bigint_modpow, plain_expmod};
use num_bigint::BigUint;
use rand::Rng;

use criterion::{
  black_box,
  criterion_group,
  criterion_main,
  Criterion
};

fn expmod_benchmark(c: &mut Criterion) {
  const N: usize = 4;
  const MAX_INT: u32 = 4294967295;
  let mut rng = rand::thread_rng();
  let base: [BigUint; N] = black_box([
    BigUint::new((0..1).map(|_| rng.gen_range(0..MAX_INT)).collect()),
    BigUint::new((0..2).map(|_| rng.gen_range(0..MAX_INT)).collect()),
    BigUint::new((0..3).map(|_| rng.gen_range(0..MAX_INT)).collect()),
    BigUint::new((0..4).map(|_| rng.gen_range(0..MAX_INT)).collect())
  ]);
  let exponent: [BigUint; N] = black_box([
    BigUint::new((0..1).map(|_| rng.gen_range(0..MAX_INT)).collect()),
    BigUint::new((0..2).map(|_| rng.gen_range(0..MAX_INT)).collect()),
    BigUint::new((0..3).map(|_| rng.gen_range(0..MAX_INT)).collect()),
    BigUint::new((0..4).map(|_| rng.gen_range(0..MAX_INT)).collect())
  ]);
  let modulus: [BigUint; N] = black_box([
    BigUint::new((0..1).map(|_| rng.gen_range(0..MAX_INT)).collect()),
    BigUint::new((0..2).map(|_| rng.gen_range(0..MAX_INT)).collect()),
    BigUint::new((0..3).map(|_| rng.gen_range(0..MAX_INT)).collect()),
    BigUint::new((0..4).map(|_| rng.gen_range(0..MAX_INT)).collect())
  ]);
  // c.bench_function(
  //   "range 1",
  //   |b| b.iter(|| plain_expmod(&base[0], &exponent[0], &modulus[0]))
  // );
  // c.bench_function(
  //   "range 2",
  //   |b| b.iter(|| plain_expmod(&base[1], &exponent[1], &modulus[1]))
  // );
  // c.bench_function(
  //   "range 3",
  //   |b| b.iter(|| plain_expmod(&base[2], &exponent[2], &modulus[2]))
  // );
  // c.bench_function(
  //   "range 4",
  //   |b| b.iter(|| plain_expmod(&base[3], &exponent[3], &modulus[3]))
  // );
  c.bench_function(
    "range 1",
    |b| b.iter(|| bigint_modpow(&base[0], &exponent[0], &modulus[0]))
  );
  c.bench_function(
    "range 2",
    |b| b.iter(|| bigint_modpow(&base[1], &exponent[1], &modulus[1]))
  );
  c.bench_function(
    "range 3",
    |b| b.iter(|| bigint_modpow(&base[2], &exponent[2], &modulus[2]))
  );
  c.bench_function(
    "range 4",
    |b| b.iter(|| bigint_modpow(&base[3], &exponent[3], &modulus[3]))
  );
}

criterion_group!(benches, expmod_benchmark);
criterion_main!(benches);