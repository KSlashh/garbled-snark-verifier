use num_bigint::{BigInt, BigUint};
use num_integer::Integer;
use num_traits::{One, Zero};
use rand::{Rng, rng};

pub const BIT_SIZE: usize = BYTES_SIZE * 8;
pub const BYTES_SIZE: usize = 128;

pub fn mod_add(a: &BigUint, b: &BigUint, m: &BigUint) -> BigUint {
    (a + b) % m
}

pub fn mod_sub(a: &BigUint, b: &BigUint, m: &BigUint) -> BigUint {
    ((a + m) - b) % m
}

pub fn mod_mul(a: &BigUint, b: &BigUint, m: &BigUint) -> BigUint {
    (a * b) % m
}

pub fn mod_pow(base: &BigUint, exp: &BigUint, m: &BigUint) -> BigUint {
    base.modpow(exp, m)
}

pub fn mod_inv(a: &BigUint, m: &BigUint) -> Option<BigUint> {
    let a = BigInt::from(a.clone());
    let m = BigInt::from(m.clone());
    let (g, x, _) = extended_gcd(&a, &m);
    if g != BigInt::one() {
        None
    } else {
        Some(((x % &m + &m) % &m).to_biguint().unwrap())
    }
}

pub fn mod_bipow_mul(a: &BigUint, e1: &BigUint, b: &BigUint, e2: &BigUint, m: &BigUint) -> BigUint {
    let a_pow = mod_pow(a, e1, m);
    let b_pow = mod_pow(b, e2, m);
    mod_mul(&a_pow, &b_pow, m)
}

fn extended_gcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    if b.is_zero() {
        (a.clone(), BigInt::one(), BigInt::zero())
    } else {
        let (gcd, x1, y1) = extended_gcd(b, &(a % b));
        (gcd, y1.clone(), x1 - (a / b) * y1)
    }
}

pub fn is_invertible(a: &BigUint, m: &BigUint) -> bool {
    a.gcd(m) == BigUint::one()
}

pub fn random_biguint() -> BigUint {
    BigUint::from_bytes_le(&rng().random::<[u8; BYTES_SIZE]>())
}
