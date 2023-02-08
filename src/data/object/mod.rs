pub mod sun;

pub use super::PseudorandomGenerable;
use bevy::prelude::*;

//////////////////// Trait ////////////////////

/// describe common property for an astronomical object
pub trait AstronomicalObject: PseudorandomGenerable {
    fn mass(&self) -> f32; // unit in solar mass
    fn radius(&self) -> f32; // unit in solar radius
}

/// an object that is luminous
pub trait LuminousObject: AstronomicalObject {
    fn luminosity(&self) -> f32; // unit in solar luminosity
    fn temperature(&self) -> f32; // unit in kelvin
}

////////////////// Component //////////////////////

#[derive(Component, Copy, Clone, Debug, PartialEq)]
/// the mass of astronomical object, unit see trait `AstronomicalObject`
pub struct Mass(pub f32);

impl From<f32> for Mass {
    fn from(value: f32) -> Self {
        Mass(value)
    }
}

impl Into<f32> for Mass {
    fn into(self) -> f32 {
        self.0
    }
}

#[derive(Component, Copy, Clone, Debug, PartialEq)]
/// the radius of astronomical object, unit see trait `AstronomicalObject`
pub struct Radius(pub f32);

impl From<f32> for Radius {
    fn from(value: f32) -> Self {
        Radius(value)
    }
}

impl Into<f32> for Radius {
    fn into(self) -> f32 {
        self.0
    }
}

////////////////// Constant //////////////////////

/// the mass for SDSS_J0917+46, the white dwarf with smallest mass human every
/// discovered.
/// source: https://ui.adsabs.harvard.edu/abs/2007ApJ...660.1451K/abstract
const SDSS_J0917_46_MASS: f32 = 0.17;

/// the Chandrasekhar limit, a theory upper bound for the mass of white dwarf;
/// therefore it's also a lower bound for the mass of neutron star.
/// source: https://en.wikipedia.org/wiki/Chandrasekhar_limit
const CHANDRASEKHAR_LIMIT: f32 = 1.40;
