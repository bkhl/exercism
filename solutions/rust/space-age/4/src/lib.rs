#[derive(Debug)]
pub struct Duration {
    seconds: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { seconds: s as f64 }
    }
}

pub trait Planet {
    const SECONDS_PER_YEAR: f64;

    fn years_during(d: &Duration) -> f64 {
        d.seconds / Self::SECONDS_PER_YEAR
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
    const SECONDS_PER_YEAR: f64 = 31_557_600.0;

    fn years_during(d: &Duration) -> f64 {
        d.seconds / 31_557_600.0
    }
}

impl Planet for Mercury {
    const SECONDS_PER_YEAR: f64 = 7_600_543.82;
}

impl Planet for Venus {
    const SECONDS_PER_YEAR: f64 = 19_414_149.05;
}

impl Planet for Mars {
    const SECONDS_PER_YEAR: f64 = 59_354_032.69;
}

impl Planet for Jupiter {
    const SECONDS_PER_YEAR: f64 = 374_355_659.12;
}

impl Planet for Saturn {
    const SECONDS_PER_YEAR: f64 = 929_292_362.88;
}

impl Planet for Uranus {
    const SECONDS_PER_YEAR: f64 = 2_651_370_019.33;
}

impl Planet for Neptune {
    const SECONDS_PER_YEAR: f64 = 5_200_418_560.03;
}
