pub fn is_armstrong_number(num: u32) -> bool {
    let num_as_string = num.to_string();
    let num_of_digits: u32 = num_as_string.chars().count() as u32;

    let sum: u64 = num_as_string
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u64)
        .map(|x| x.checked_pow(num_of_digits))
        .try_fold(0u64, |acc, x| match x {
            Some(pow) => acc.checked_add(pow),
            None => None
        })
        .unwrap();
    

    sum == num as u64
}
