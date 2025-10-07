use std::fmt::{Display, Formatter, Result};
use std::convert::From;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Clock {
    hours: u8,
    minutes: u8,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // -54, -11_513 => "18:7"
        let normalized_minutes = (60 + (minutes % 60)) % 60;
        let normalized_hours = (24 + (hours % 24)) % 24;
        let rollover_hours = (minutes as f64 / 60_f64).floor() as i32;
        let hours = (24 + ((normalized_hours + rollover_hours) % 24)) % 24;

        Clock { 
            hours: hours as u8, 
            minutes: normalized_minutes as u8 
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours as i32 , self.minutes as i32 + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl From<Clock> for String {
    fn from(value: Clock) -> Self {
        value.to_string()
    }
}

impl From<String> for Clock {
    fn from(value: String) -> Self {
        let tokens: Vec<&str> = value.split(":").collect();
        Clock::new(tokens[0].parse::<i32>().unwrap(), tokens[1].parse::<i32>().unwrap())
    }
}