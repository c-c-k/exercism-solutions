macro_rules! setup_planets {
    ($($name:ident, $ratio:literal);*) => {
        $(
        pub struct $name;
        impl Planet for $name {
            fn years_during(d: &Duration) -> f64 {
                d.seconds / EARTH_YEAR_SECONDS / $ratio
            }
        }
        )*
    };
}

const EARTH_YEAR_SECONDS: f64 = 31_557_600_f64;

#[derive(Debug)]
pub struct Duration {
    seconds: f64,
}

impl From<u64> for Duration {
    fn from(seconds: u64) -> Self {
        Duration {
            seconds: seconds as f64,
        }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

setup_planets!(
    Mercury, 0.2408467;
    Venus, 0.61519726;
    Earth, 1.0;
    Mars, 1.8808158;
    Jupiter, 11.862615;
    Saturn, 29.447498;
    Uranus, 84.016846;
    Neptune, 164.79132
);
