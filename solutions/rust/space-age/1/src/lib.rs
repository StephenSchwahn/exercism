// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

#[macro_export]
macro_rules! years_during {
    ($e:expr) => {
        fn years_during(d: &Duration) -> f64 {
            (d.0 as f64 / 31557600.0) / $e
        }
    }
}

pub trait Planet {
    years_during!(1.0);
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    years_during!(0.2408467);
}
impl Planet for Venus {
    years_during!(0.61519726);
}
impl Planet for Earth {
    years_during!(1.0);
}
impl Planet for Mars {
    years_during!(1.8808158);
}
impl Planet for Jupiter {
    years_during!(11.862615);
}
impl Planet for Saturn {
    years_during!(29.447498);
}
impl Planet for Uranus {
    years_during!(84.016846);
}
impl Planet for Neptune {
    years_during!(164.79132);
}
