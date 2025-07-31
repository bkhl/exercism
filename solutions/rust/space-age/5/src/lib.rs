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

macro_rules! planet {
    ($planet:ident, $seconds_per_year:expr) => {
        pub struct $planet;

        impl Planet for $planet {
            const SECONDS_PER_YEAR: f64 = $seconds_per_year;
        }
    };
}

planet!(Earth, 31_557_600.0);
planet!(Jupiter, 374_355_659.12);
planet!(Mars, 59_354_032.69);
planet!(Mercury, 7_600_543.82);
planet!(Neptune, 5_200_418_560.03);
planet!(Saturn, 929_292_362.88);
planet!(Uranus, 2_651_370_019.33);
planet!(Venus, 19_414_149.05);
