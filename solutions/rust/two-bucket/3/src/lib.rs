use std::cmp;

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

#[derive(PartialEq, Eq, Debug)]
pub struct WaterBucket {
    pub label: Bucket,
    pub capacity: u8,
    pub water_level: u8,
}

impl WaterBucket {
    pub fn fill(&mut self) {
        self.water_level = self.capacity;
    }

    pub fn empty(&mut self) {
        self.water_level = 0u8;
    }

    // Fill other bucket from this bucket
    pub fn pour_into(&mut self, other: &mut WaterBucket) {
        let poured_amount = cmp::min(self.water_level, other.capacity - other.water_level);
        self.water_level -= poured_amount;
        other.water_level += poured_amount
    }
}

fn gcd(a: u8, b: u8) -> u8 {
    if b == 0 { a } else { gcd(b, a % b) }
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    match (capacity_1, capacity_2) {
        (capacity_1, capacity_2) if capacity_1 < goal && capacity_2 < goal  => None,
        (capacity_1, capacity_2) if goal % gcd(capacity_1, capacity_2) != 0 => None,
        (capacity_1, capacity_2) => {
            let mut this_bucket = WaterBucket {
                label: if *start_bucket == Bucket::One { Bucket::One } else { Bucket::Two },
                water_level: 0,
                capacity: if *start_bucket == Bucket::One { capacity_1 } else { capacity_2 }
            };

            let mut other_bucket = WaterBucket {
                label: if *start_bucket == Bucket::One { Bucket::Two } else { Bucket::One },
                water_level: 0,
                capacity: if *start_bucket == Bucket::One { capacity_2 } else { capacity_1 }
            };

            let mut counter = 0u8;
            while this_bucket.water_level != goal && other_bucket.water_level != goal {
                if this_bucket.water_level == 0 {
                    this_bucket.fill();
                } else if other_bucket.water_level == other_bucket.capacity {
                    other_bucket.empty();
                } else if other_bucket.capacity == goal {
                    other_bucket.fill();
                } else {
                    this_bucket.pour_into(&mut other_bucket);
                }
                counter += 1;
            };
 
            Some(BucketStats {
                moves: counter,
                goal_bucket: if this_bucket.water_level == goal { this_bucket.label } else { other_bucket.label },
                other_bucket: if this_bucket.water_level == goal { other_bucket.water_level } else { this_bucket.water_level },
            })
        }
    }
}
