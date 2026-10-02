//! The few transcendental functions the engine needs, for `no_std`.
//!
//! All but the square root run at control rate only -- building tables
//! when a parameter moves, never per sample -- so they are written for
//! accuracy, in `f64`, rather than speed. Each states its accuracy, and a
//! test holds it against `std`. The square root runs in every reed's step
//! and is the hardware's.

const LN_2: f64 = core::f64::consts::LN_2;
const PI: f64 = core::f64::consts::PI;

/// Square root, correctly rounded as IEEE 754 asks: WebAssembly's
/// `f64.sqrt` in the plugin, the standard library's natively -- the same
/// operation, so every target computes the same bits. Zero and negatives
/// give zero; infinity and NaN pass through.
///
/// Milestone 10: it was a software root (a bit-level seed and six Newton
/// steps), one ulp off for a quarter of the engine's arguments, wrong for
/// subnormals, and six divisions long in every reed's step.
#[inline]
pub fn sqrt(x: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    // The scalar `f64.sqrt` is not yet stable in `core::arch`; SIMD's,
    // which the plugin is built with, is the same IEEE operation per lane.
    #[cfg(target_arch = "wasm32")]
    {
        use core::arch::wasm32::{f64x2_extract_lane, f64x2_splat, f64x2_sqrt};
        f64x2_extract_lane::<0>(f64x2_sqrt(f64x2_splat(x)))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::primitive::f64::sqrt(x)
    }
}

/// `e^x` by splitting off powers of two and a Taylor series on the rest;
/// relative error below 1e-15 over the range the engine uses.
pub fn exp(x: f64) -> f64 {
    if x > 709.0 {
        return f64::INFINITY;
    }
    if x < -745.0 {
        return 0.0;
    }
    let k = round(x / LN_2);
    let r = x - k * LN_2;
    let mut term = 1.0;
    let mut sum = 1.0;
    for n in 1..24 {
        term *= r / n as f64;
        sum += term;
    }
    scale_by_power_of_two(sum, k as i32)
}

/// Natural logarithm through the exponent and an `atanh` series on the
/// mantissa; relative error below 1e-15. Zero and negatives give -inf.
pub fn ln(x: f64) -> f64 {
    if x <= 0.0 {
        return f64::NEG_INFINITY;
    }
    let bits = x.to_bits();
    let mut exponent = ((bits >> 52) & 0x7ff) as i32 - 1023;
    let mut mantissa = f64::from_bits((bits & 0x000f_ffff_ffff_ffff) | (1023u64 << 52));
    if mantissa > core::f64::consts::SQRT_2 {
        mantissa *= 0.5;
        exponent += 1;
    }
    let z = (mantissa - 1.0) / (mantissa + 1.0);
    let z2 = z * z;
    let mut term = z;
    let mut sum = 0.0;
    for n in 0..40 {
        sum += term / (2 * n + 1) as f64;
        term *= z2;
    }
    2.0 * sum + exponent as f64 * LN_2
}

/// `x^y` for positive `x`; zero for anything else.
pub fn pow(x: f64, y: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    exp(y * ln(x))
}

/// Sine and cosine together, by reduction to a quarter turn and Taylor
/// series; absolute error below 1e-15 for arguments the engine uses.
pub fn sin_cos(x: f64) -> (f64, f64) {
    let quarter = round(x / (0.5 * PI));
    let r = x - quarter * (0.5 * PI);
    let r2 = r * r;
    let (mut s, mut c) = (r, 1.0);
    let (mut ts, mut tc) = (r, 1.0);
    for n in 1..14 {
        ts *= -r2 / ((2 * n) * (2 * n + 1)) as f64;
        tc *= -r2 / ((2 * n - 1) * (2 * n)) as f64;
        s += ts;
        c += tc;
    }
    match (quarter as i64).rem_euclid(4) {
        0 => (s, c),
        1 => (c, -s),
        2 => (-s, -c),
        _ => (-c, s),
    }
}

pub fn cosh_sinh(x: f64) -> (f64, f64) {
    let (a, b) = (exp(x), exp(-x));
    (0.5 * (a + b), 0.5 * (a - b))
}

/// Round half away from zero.
pub fn round(x: f64) -> f64 {
    if x >= 0.0 {
        (x + 0.5) as i64 as f64
    } else {
        -((-x + 0.5) as i64 as f64)
    }
}

fn scale_by_power_of_two(value: f64, power: i32) -> f64 {
    let mut value = value;
    let mut power = power;
    while power > 1000 {
        value *= f64::from_bits(2046u64 << 52);
        power -= 1023;
    }
    while power < -1000 {
        value *= f64::from_bits(1u64 << 52);
        power += 1022;
    }
    value * f64::from_bits(((1023 + power) as u64) << 52)
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    fn close(a: f64, b: f64, relative: f64) -> bool {
        (a - b).abs() <= relative * b.abs().max(1e-300)
    }

    #[test]
    fn every_function_matches_std_within_its_stated_accuracy() {
        for i in 0..2000 {
            let x = (i as f64 - 1000.0) * 0.0137;
            assert!(close(exp(x), x.exp(), 1e-14), "exp {x}");
            let (s, c) = sin_cos(x);
            assert!((s - x.sin()).abs() < 1e-14, "sin {x}");
            assert!((c - x.cos()).abs() < 1e-14, "cos {x}");
            let positive = (i as f64 + 1.0) * 0.731;
            assert!(
                close(sqrt(positive), positive.sqrt(), 1e-15),
                "sqrt {positive}"
            );
            assert!(close(ln(positive), positive.ln(), 1e-14), "ln {positive}");
            assert!(
                close(pow(positive, 0.37), positive.powf(0.37), 1e-13),
                "pow {positive}"
            );
        }
        assert!(close(sqrt(4.0e-12), 2.0e-6, 1e-15));
        // The root is the correctly rounded one, subnormals and the
        // extremes included.
        for x in [5.0e-324, 1.0e-310, 2.0, 1.0e300, f64::MAX, f64::INFINITY] {
            assert_eq!(sqrt(x).to_bits(), x.sqrt().to_bits(), "{x}");
        }
        assert!(sqrt(f64::NAN).is_nan());
        assert!(close(ln(1.0e-9), (1.0e-9f64).ln(), 1e-14));
        assert_eq!(sqrt(-1.0), 0.0);
        assert_eq!(pow(0.0, 2.0), 0.0);
    }
}
