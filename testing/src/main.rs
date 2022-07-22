
use num_traits::{One, Zero, ToPrimitive};
use num_bigint::BigUint;
use hex::decode;
use bytes::{Buf, Bytes, BytesMut};
use arrayref::array_ref;

pub fn right_pad(buffer: Bytes, min_size: usize) -> Bytes {
    pad::<false>(buffer, min_size)
}

pub fn left_pad(buffer: Bytes, min_size: usize) -> Bytes {
    pad::<true>(buffer, min_size)
}

fn pad<const LEFT: bool>(buffer: Bytes, min_size: usize) -> Bytes {
    if buffer.len() >= min_size {
        return buffer;
    }

    let point = if LEFT { min_size - buffer.len() } else { 0 };

    let mut b = BytesMut::with_capacity(min_size);
    b.resize(min_size, 0);
    b[point..point + buffer.len()].copy_from_slice(&buffer[..]);
    b.freeze()
}

fn expmod_biguint(base: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
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

fn expmod_run(input: Bytes) -> Option<Bytes> {
    let mut input = right_pad(input, 3 * 32);
    let base_len = usize::try_from(u64::from_be_bytes(*array_ref!(input, 24, 8))).unwrap();
    let exponent_len = usize::try_from(u64::from_be_bytes(*array_ref!(input, 56, 8))).unwrap();
    let modulus_len = usize::try_from(u64::from_be_bytes(*array_ref!(input, 88, 8))).unwrap();

    if modulus_len == 0 {
        return Some(Bytes::new());
    }

    input.advance(96);
    let input = right_pad(input, base_len + exponent_len + modulus_len);

    let base = BigUint::from_bytes_be(&input[..base_len]);
    let exponent = BigUint::from_bytes_be(&input[base_len..base_len + exponent_len]);
    let modulus = BigUint::from_bytes_be(
        &input[base_len + exponent_len..base_len + exponent_len + modulus_len],
    );

    let mut out = vec![0; modulus_len];
    if modulus.is_zero() {
        return Some(out.into());
    }

    // TODO: Improve efficiency

    let b: Vec<u8>;
    if base_len <= 16 && exponent_len <= 16 && modulus_len <= 16 {
        b = expmod_biguint(&base, &exponent, &modulus).to_bytes_be();
    } else {
        b = base.modpow(&exponent, &modulus).to_bytes_be();
    }

    out[modulus_len - b.len()..].copy_from_slice(&b);

    Some(out.into())
}

pub fn main() {
    let b = decode(b"00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001003fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2efffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f").unwrap();
    println!("{:?}", expmod_run(Bytes::from(b)).unwrap())
}