use std::fmt::{Display, Formatter, Result};

pub struct Roman {
    thousands: u8,
    five_hundreds: u8,
    hundreds: u8,
    fifties: u8,
    tens: u8,
    fives: u8,
    ones: u8,
}

impl Display for Roman {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        let nine_hundred = self.hundreds == 4 && self.five_hundreds == 1;
        let four_hundred = self.hundreds == 4 && self.five_hundreds == 0;
        let ninety = self.tens == 4 && self.fifties == 1;
        let forty = self.tens == 4 && self.fifties == 0;
        let nine = self.ones == 4 && self.fives == 1;
        let four = self.ones == 4 && self.fives == 0;

        write!(_f, "{}{}{}{}{}{}{}",
               "M".repeat(self.thousands.into()),
               if nine_hundred { "CM".to_owned() } else { "D".repeat(self.five_hundreds.into()) },
               if four_hundred { "CD".to_owned() } else if !nine_hundred { "C".repeat(self.hundreds.into()) } else { "".to_owned() },
               if ninety { "XC".to_owned() } else { "L".repeat(self.fifties.into()) },
               if forty { "XL".to_owned() } else if !ninety { "X".repeat(self.tens.into()) } else { "".to_owned() },
               if nine { "IX".to_owned() } else { "V".repeat(self.fives.into()) },
               if four { "IV".to_owned() } else if !nine { "I".repeat(self.ones.into()) } else { "".to_owned() }
        )
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Roman {
            thousands: (num / 1000) as u8,
            five_hundreds: ((num % 1000) / 500) as u8,
            hundreds: ((num % 500) / 100) as u8,
            fifties: ((num % 100) / 50) as u8,
            tens: ((num % 50) / 10) as u8,
            fives: ((num % 10) / 5) as u8,
            ones: ((num % 5)) as u8,
        }
    }
}
