use bevy::prelude::Vec3;

use super::PseudorandomGenerable;

/// describe common property for an orbit
pub trait Orbit: PseudorandomGenerable {
    /// compute a position for a given time. it is guaranteed to only call this
    /// function with monotonic increasing `time`. however, notice that time
    /// interval between call are not predictable and this function should
    /// return result in a short period of time, so iterative approach is
    /// not recommended.
    fn position(time: f32) -> Vec3;

    // return the distance between center and apoapsis point, in astronomical unit
    fn apoapsis() -> f32;

    // return the distance between center and periapsis point, in astronomical unit
    fn periapsis() -> f32;
}
