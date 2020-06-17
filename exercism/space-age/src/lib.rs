// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
    earth_years: f64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { 
            seconds: s, 
            earth_years: (s as f64) / 31_557_600.0
        }
    }
}

pub trait Planet {
    const ORBITAL_PERIOD: f64;
    fn years_during(d: &Duration) -> f64 {
        d.earth_years / Self::ORBITAL_PERIOD
    }
}

macro_rules! implement_planet_trait {
    ($type:ident, $value:literal) => {
        pub struct $type;
        impl Planet for $type {
            const ORBITAL_PERIOD: f64 = $value;
        }
    };
}

implement_planet_trait!(Mercury, 0.2408467);
implement_planet_trait!(Venus, 0.61519726);
implement_planet_trait!(Earth, 1.0);
implement_planet_trait!(Mars, 1.8808158);
implement_planet_trait!(Jupiter, 11.862615);
implement_planet_trait!(Saturn, 29.447498);
implement_planet_trait!(Uranus, 84.016846);
implement_planet_trait!(Neptune, 164.79132);