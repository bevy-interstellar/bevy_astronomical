use bevy::prelude::*;
use rand_xoshiro::rand_core::SeedableRng;

use crate::data::utils::random;

pub use super::{AstronomicalObject, LuminousObject, Mass, PseudorandomGenerable, Radius};

pub trait StarBundle: LuminousObject {}

/// indicate it is a sun-like astronomical object
#[derive(Component, Copy, Clone, Debug)]
pub struct SunBase {
    /// the temperature of surface, unit see trait `LuminousObject`
    pub temperature: f32,
    /// the luminosity, unit see trait `LuminousObject`
    pub luminosity: f32,
    /// time multiplier
    pub activeness: f32,
}

/// indicate it has solar corona and flare
#[derive(Component, Copy, Clone, Debug)]
pub struct SolarCorona {
    /// time multiplier
    pub activeness: f32,
    /// threshold for flare
    pub f_threshold: f32,
    /// probability for a place to have flare if it's above threshold
    pub f_density: f32,
    /// the height of flare,
    pub f_intensity: f32,
}

#[derive(Component, Copy, Clone, Debug)]
pub struct MainSequenceStar;

#[derive(Component, Copy, Clone, Debug)]
pub struct GiantStar;

#[derive(Component, Copy, Clone, Debug)]
pub struct NeutronStar;

#[derive(Bundle, Copy, Clone, Debug)]
pub struct NeutronStarBundle {
    mass: Mass,
    radius: Radius,
    base: SunBase,
    marker: NeutronStar,
}

#[derive(Component, Copy, Clone, Debug)]
pub struct WhiteDwarf;
#[derive(Bundle, Copy, Clone, Debug)]

pub struct WhiteDwarfBundle {
    mass: Mass,
    radius: Radius,
    base: SunBase,
    marker: WhiteDwarf,
}

impl WhiteDwarfBundle {
    const MIN_MASS: f32 = super::SDSS_J0917_46_MASS;
    const MAX_MASS: f32 = super::CHANDRASEKHAR_LIMIT;
    const MIN_TEMPERATURE: f32 = 6000.0;
    const MAX_TEMPERATURE: f32 = 30000.0;

    pub fn new(mass: f32, temperature: f32) -> Self {
        let radius = Self::mass_to_radius(mass);
        let luminosity = Self::temperature_to_luminosity(temperature);

        WhiteDwarfBundle {
            mass: Mass(mass),
            radius: Radius(radius),
            base: SunBase {
                temperature: temperature,
                luminosity: luminosity,
                activeness: 1.0,
            },
            marker: WhiteDwarf,
        }
    }

    /// source: https://en.wikipedia.org/wiki/White_dwarf
    pub fn mass_to_radius(m: f32) -> f32 {
        0.01 * m.powf(-1.0 / 3.0)
    }

    pub fn temperature_to_luminosity(t: f32) -> f32 {
        let x = t.ln();
        (1.73444446 * x.powi(2) + -30.48605897 * x + 125.58164008).exp()
    }
}

impl Default for WhiteDwarfBundle {
    fn default() -> Self {
        // parameter for Sirius B
        WhiteDwarfBundle::new(0.978, 25200.0)
    }
}

impl PseudorandomGenerable for WhiteDwarfBundle {
    fn from_seed(seed: u64) -> Self {
        let mut rng = rand_xoshiro::Xoroshiro128StarStar::seed_from_u64(seed);
        let mass = random::range_normal(Self::MIN_MASS, 0.6, Self::MAX_MASS, &mut rng);
        let temperature =
            random::range_uniform(Self::MIN_TEMPERATURE, Self::MAX_TEMPERATURE, &mut rng);
        Self::new(mass, temperature)
    }
}

impl AstronomicalObject for WhiteDwarfBundle {
    fn mass(&self) -> f32 {
        self.mass.0
    }
    fn radius(&self) -> f32 {
        self.radius.0
    }
}

impl LuminousObject for WhiteDwarfBundle {
    fn luminosity(&self) -> f32 {
        self.base.luminosity
    }
    fn temperature(&self) -> f32 {
        self.base.temperature
    }
}

impl StarBundle for WhiteDwarfBundle {}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn white_dwarf_test() {
        println!("testtest");
        let x = WhiteDwarfBundle::new(1.0, 6034.03403403);

        assert_approx_eq!(f32, x.radius(), 0.01, ulps = 2);
        assert_approx_eq!(f32, x.luminosity(), 0.00023206168, ulps = 2);
    }
}
