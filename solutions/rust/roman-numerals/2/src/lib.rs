use std::fmt::{Display, Formatter, Result};

pub struct Roman(u32);

const ROMAN_FACTORS: [(&str, u32); 13] = [
    ("M", 1000),
    ("CM", 900),
    ("D", 500),
    ("CD", 400),
    ("C", 100),
    ("XC", 90),
    ("L", 50),
    ("XL", 40),
    ("X", 10),
    ("IX", 9),
    ("V", 5),
    ("IV", 4),
    ("I", 1),
];

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let string = ROMAN_FACTORS
            .iter()
            .fold((String::new(), self.0), |roman, &factor| {
                if roman.1 >= factor.1 {
                    let applications = roman.1 / factor.1;
                    (
                        roman.0 + &(factor.0).repeat(applications as usize),
                        (roman.1 - (factor.1 * applications)),
                    )
                } else {
                    roman
                }
            })
            .0;
        write!(f, "{}", string)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Roman(num)
    }
}
