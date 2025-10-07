fn is_prime(n: u64) -> bool {
    match n {
        1 => false,
        2 => true,
        _ => {
            let limit: u64 = (n as f64).sqrt().ceil() as u64;
            (2..=limit).find(|&fac| n % fac == 0).is_none()
        }
    }
}

pub fn factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
    let mut test_num: u64 = n;
    let mut factor = 2u64;
    loop {
        if test_num == 1 {
            break;
        }
        if test_num % factor == 0 {
            factors.push(factor);
            test_num /= factor;
        } else {
            // Get next prime
            factor = factor + 1;
            while !is_prime(factor) {
                factor += 2
            }
        }
    }
    factors
}
