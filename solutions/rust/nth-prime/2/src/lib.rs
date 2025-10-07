use std::collections::HashMap;

fn is_prime(n: u32, map: &mut HashMap<u32, bool>) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 { 
        return true;
    }
    if let Some(prime) = map.get(&n) {
        return *prime;
    } 
     
    // calculate if number is prime
    let limit: u32 = (n as f64).sqrt().ceil() as u32 ;
    for i in 2..=limit {
        if n % i == 0 {
            map.insert(n, false);
            return false;
        }
    }
    map.insert(n, true);
    return true;
}

pub fn nth(n: u32) -> u32 {
    let mut memoized: HashMap<u32, bool> = HashMap::from([
        (1u32, false),
        (2u32, true),
        (3u32, true),
    ]);
    
    let mut count: i64 = -1;
    let mut num = 2;
    while count < n.into() {
        if is_prime(num, &mut memoized) {
            count += 1;
        }
        if count == n.into() {
            return num;
        }
        num += 1;
    }
    return 2;
}
