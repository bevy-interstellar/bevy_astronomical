// /// anything physically exists
// pub mod object;

// /// the movement of orbit
// pub mod orbit;

// /// unit conversion tool
// mod conversion;

/// can be randomly generated by a seed.
pub trait PseudorandomGenerable {
    /// must guarantee reproducibility across platform.
    fn from_seed(seed: u64) -> Self;
}
