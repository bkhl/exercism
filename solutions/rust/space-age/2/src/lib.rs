#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { seconds: s }
    }
}

pub trait Planet {
    const REVOLUTIONS_PER_EARTH_YEAR: f64;

    fn years_during(d: &Duration) -> f64 {
        Earth::years_during(d) / Self::REVOLUTIONS_PER_EARTH_YEAR
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Earth {
    const REVOLUTIONS_PER_EARTH_YEAR: f64 = 1.0;

    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / 31_557_600.0
    }
}

impl Planet for Mercury {
    const REVOLUTIONS_PER_EARTH_YEAR: f64 = 0.240_846_7;
}

impl Planet for Venus {
    const REVOLUTIONS_PER_EARTH_YEAR: f64 = 0.615_197_26;
}

impl Planet for Mars {
    const REVOLUTIONS_PER_EARTH_YEAR: f64 = 1.880_815_8;
}

impl Planet for Jupiter {
    const REVOLUTIONS_PER_EARTH_YEAR: f64 = 11.862_615;
}

impl Planet for Saturn {
    const REVOLUTIONS_PER_EARTH_YEAR: f64 = 29.447_498;
}

impl Planet for Uranus {
    const REVOLUTIONS_PER_EARTH_YEAR: f64 = 84.016_846;
}

impl Planet for Neptune {
    const REVOLUTIONS_PER_EARTH_YEAR: f64 = 164.791_32;
}
