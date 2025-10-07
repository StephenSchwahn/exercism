use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;
use rand::{thread_rng, Rng};

pub fn private_key(p: u64) -> u64 {
    thread_rng().gen_range(2..p)
}

fn modpow(p: u64, g: u64, a: u64) -> u64 {
    (BigUint::from(g))
    .modpow(&BigUint::from(a), &BigUint::from(p))
    .to_u64()
    .expect("Number doesn't fit in u64")
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modpow(p, g, a)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modpow(p, b_pub, a)
}
