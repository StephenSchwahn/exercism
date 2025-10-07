fn is_prime(n: u32) -> bool {
    match n {
        1 => false,
        2 => true,
        _ => {
            let limit: u32 = (n as f64).sqrt().ceil() as u32;
            (2..=limit).find(|&fac| n % fac == 0).is_none()
        }
    }
}

pub fn nth(n: u32) -> u32 {
    (2..)
        .filter(|num| is_prime(*num))
        .nth(n as usize)
        .unwrap_or_default()
}

