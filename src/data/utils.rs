macro_rules! conversion {
    ($u1:ident, $u2:ident, $factor:expr) => {
        paste::paste! {
            pub fn [<$u1 _to_ $u2>](x: f32) -> f32 {
                x / $factor
            }

            pub fn [<$u2 _to_ $u1>](x: f32) -> f32 {
                x * $factor
            }
        }
    };
}

pub mod length {
    conversion!(meter, km, 1000.0);
    conversion!(km, sol_rad, 695700.0);
    conversion!(km, au, 149597870.7);
    conversion!(au, ly, 63241.077);
}

pub mod random {
    use rand::distributions::Uniform;
    use rand::Rng;
    use rand_distr::Normal;

    /// generated a random number based on lower bound `lb`, mean `mu`, upper
    /// bound `ub`. it follows normal distribution, and ensure no value will
    /// be outside of lb or ub.
    pub fn range_normal<R: Rng + ?Sized>(lb: f32, mu: f32, ub: f32, rng: &mut R) -> f32 {
        let distance = f32::min(ub - mu, mu - lb);
        let sigma = distance / 3.0;
        let normal = Normal::from_mean_cv(mu, (sigma / mu).abs()).unwrap();
        let value = rng.sample(normal);
        f32::min(ub, f32::max(lb, value))
    }

    pub fn range_uniform<R: Rng + ?Sized>(lb: f32, ub: f32, rng: &mut R) -> f32 {
        let uniform = Uniform::new_inclusive(lb, ub);
        let value = rng.sample(uniform);
        return value;
    }
}

#[cfg(test)]
mod tests {
    use super::length::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn kilometer_test() {
        assert_approx_eq!(f32, meter_to_km(1000.0), 1.0, ulps = 5);
        assert_approx_eq!(f32, km_to_meter(1.0), 1000.0, ulps = 5);
    }

    #[test]
    fn solar_radius_test() {
        assert_approx_eq!(f32, km_to_sol_rad(695700.0), 1.0, ulps = 5);
        assert_approx_eq!(f32, sol_rad_to_km(1.0), 695700.0, ulps = 5);
    }

    #[test]
    fn astronomical_unit_test() {
        assert_approx_eq!(f32, km_to_au(149597870.7), 1.0, ulps = 5);
        assert_approx_eq!(f32, au_to_km(1.0), 149597870.7, ulps = 5);
    }

    #[test]
    fn light_year_test() {
        assert_approx_eq!(f32, au_to_ly(63241.077), 1.0, ulps = 5);
        assert_approx_eq!(f32, ly_to_au(1.0), 63241.077, ulps = 5);
    }
}
