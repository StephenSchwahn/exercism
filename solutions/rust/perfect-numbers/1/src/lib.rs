#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    let factors = |num: u64| (1..num).filter(move |candidate| num % candidate == 0);
    let aliquot_sum = factors(num).sum();

    match num {
        0 => None,
        x if x < aliquot_sum => Some(Classification::Abundant),
        x if x == aliquot_sum => Some(Classification::Perfect),
        x if x > aliquot_sum => Some(Classification::Deficient),
        _ => None,
    }
}
