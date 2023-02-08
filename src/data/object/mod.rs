use bevy::prelude::*;

//////////////////// Trait ////////////////////

/// describe common property for an astronomical object
pub trait AstronomicalObject: PseudorandomGenerable {
    fn mass() -> f32; // unit in solar mass
    fn radius() -> f32; // unit in solar radius
}

/// an object that is luminous
pub trait LuminousObject: AstronomicalObject {
    fn luminosity() -> f32;
}

////////////////// Component //////////////////////

#[derive(Component, Copy, Clone, Debug, PartialEq)]
/// the mass of astronomical object, unit in solar mass (M☉)
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
// the radius of astronomical object, unit in solar radius (R☉)
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
