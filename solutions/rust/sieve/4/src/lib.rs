pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut numbers: Vec<_> = (0..=upper_bound).map(Option::from).collect();
    
    (2..=upper_bound)
        .filter_map(|idx| {
            let prime = numbers[idx as usize].take()?;

            for invalid_candidate in (prime*2..=upper_bound).step_by(prime as usize) {
                numbers[invalid_candidate as usize] = None
            }

            Some(prime)
        })
        .collect()
}
