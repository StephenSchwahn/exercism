pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    (2..=upper_bound)
        .fold(Vec::new(), |mut acc, elem| {
            for prime in &acc {
                if elem % prime == 0 {
                    return acc
                }
            }
            acc.push(elem);
            acc
        })
}
