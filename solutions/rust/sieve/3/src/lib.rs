pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    (2..=upper_bound).fold(Vec::new(), |mut acc, elem| {
        match acc
            .iter()
            .map(|prime| elem % prime)
            .filter(|candidate| *candidate == 0)
            .collect::<Vec<u64>>()
            .len() {
                0 => { acc.push(elem); acc }
                _ => acc
            }
    })
}
