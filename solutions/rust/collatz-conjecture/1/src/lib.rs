pub fn collatz(n: u64) -> Option<u64> {
    // todo!("return Some(x) where x is the number of steps required to reach 1 starting with {n}")
    let mut count = 0;
    let mut n = n;
    loop {
        match n {
            0 => return None,
            1 => return Some(count),
            _ if n % 2 == 0 => {
                n /= 2;
                count += 1
            }
            _ if n % 2 == 1 => match n.checked_mul(3) {
                Some(mult) => match mult.checked_add(1) {
                    Some(added) => {
                        n = added;
                        count += 1;
                    }
                    None => return None,
                },
                None => return None,
            },
            _ => panic!("inconceivable")
        }
    }
}
