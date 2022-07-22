pub mod exponents {
  use num_traits::{One, Zero, ToPrimitive};
  use num_bigint::BigUint;

  pub fn plain_expmod(base: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
      let mut acc: BigUint = One::one();
      let mut b: BigUint = base.clone();
      if base.is_zero() {
          return BigUint::zero();
      }
      if let Some(mut expt) = exponent.to_u128() {
          while expt != 0 {
              if expt & 1 == 0 {
                  b = &b * &b;
                  b %= modulus;
                  expt >>= 1;
              } else {
                  acc *= &b;
                  acc %= modulus;
                  expt -= 1
              }
          }
          acc
      } else {
          unimplemented!("")
      }
  }
  pub fn bigint_modpow(x: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
      BigUint::modpow(&x,  &exponent, &modulus)
  }

  pub fn plain_expmod_testing(base: &[BigUint], exponent: &[BigUint], modulus: &[BigUint]) -> () {
      for i in 0..base.len() {
          plain_expmod(&base[i],&exponent[i],&modulus[i]);
      }
  }
  pub fn bigint_modpow_testing(base: &[BigUint], exponent: &[BigUint], modulus: &[BigUint]) -> () {
      for i  in 0..base.len() {
          bigint_modpow(&base[i],&exponent[i],&modulus[i]);
      }
  }
}