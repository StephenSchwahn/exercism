pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut primes: Vec<u64> = Vec::new();
    'outer: for n in 2..=upper_bound {
        for prime in &primes {
            if n % prime == 0 {
                continue 'outer;
            }
        }
        primes.push(n);
    }

    primes
}
