use rand::{thread_rng, Rng};

pub fn private_key(p: u64) -> u64 {
    thread_rng().gen_range(2..p)
}

fn modpow(base: u64, exponent: u64, modulus: u64) -> u64 {
    if modulus == 1 { return 0 }
    
    // Convert all into u128s until the end of operations so that 
    // we can deal with "really" large numbers
    let mut result = 1u128;
    let mut base = base as u128 % modulus as u128;
    let mut exponent = exponent as u128;
    let modulus = modulus as u128;

    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }
        exponent = exponent >> 1;
        base = (base * base) % modulus;
    }
    
    // At this point, we *should* be back down to a u64
    result as u64
}   

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modpow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modpow(b_pub, a, p)
}
