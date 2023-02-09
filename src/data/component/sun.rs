use bevy::prelude::*;

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
/// a main sequence star
pub struct MainSequenceStar;

#[derive(Component, Copy, Clone, Debug)]
/// a giant star
pub struct GiantStar;

#[derive(Component, Copy, Clone, Debug)]
/// a neutron star
pub struct NeutronStar;

#[derive(Component, Copy, Clone, Debug)]
/// a dwarf star
pub struct WhiteDwarf;
