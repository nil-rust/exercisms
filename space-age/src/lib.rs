// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: u64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self {
            seconds: s,
        }
    }
}

pub trait Planet {
    fn year_earth_equivalent() -> f64;

    fn years_during(d: &Duration) -> f64 {
        let year_equivalence = Self::year_earth_equivalent();
        let earth_years = (d.seconds as f64) / 60.0 / 60.0 / 24.0 / 365.25;

        earth_years / year_equivalence
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

impl Planet for Mercury { fn year_earth_equivalent() -> f64 { 0.2408467 } }
impl Planet for Venus { fn year_earth_equivalent() -> f64 { 0.61519726  } }
impl Planet for Earth { fn year_earth_equivalent() -> f64 { 1.0 } }
impl Planet for Mars { fn year_earth_equivalent() -> f64 { 1.8808158 } }
impl Planet for Jupiter { fn year_earth_equivalent() -> f64 { 11.862615 } }
impl Planet for Saturn { fn year_earth_equivalent() -> f64 { 29.447498 } }
impl Planet for Uranus { fn year_earth_equivalent() -> f64 { 84.016846 } }
impl Planet for Neptune { fn year_earth_equivalent() -> f64 { 164.79132 } }
