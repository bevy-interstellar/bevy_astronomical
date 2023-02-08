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
