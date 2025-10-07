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

pub struct Prime {
    curr: u64,
    next: u64,
}

impl Prime {
    pub fn new() -> Prime {
        Prime { curr: 2, next: 3 }
    }
}

impl Iterator for Prime {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let prime = self.curr;
        self.curr = self.next;
        loop {
            self.next += match self.next % 6 {
                1 => 4,
                _ => 2,
            };
            if is_prime(self.next) {
                break;
            }
        }
        Some(prime)
    }
}

pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_factors: Vec<u64> = Vec::new();
    let mut test_num: u64 = n;

    for factor in Prime::new().into_iter() {
        if test_num == 1 {
            break;
        }
        while test_num % factor == 0 {
            prime_factors.push(factor);
            test_num /= factor;
        }
    }
    prime_factors
}
