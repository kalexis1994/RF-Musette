//! The tongue's first mode, for a profiled tongue.
//!
//! A uniform clamped-free beam has its first two bending modes at 1 : 6.27.
//! The accordion F4 tongue Ziegenhals measured has them at 355 and 1645 Hz,
//! 1 : 4.6, "because of the profile" (IfM Zwota 2009): tongues are ground
//! thinner toward the tip. The profile decides the tongue's modal mass and so
//! its stiffness at the tip, which decides how far the air moves it. So the
//! profile is derived from the measured ratio rather than assumed flat.
//!
//! The family is a linear taper of thickness at constant width,
//! t(s) = t₀ (1 - c s), s the position from the clamp over the length. For a
//! taper c the Euler-Bernoulli problem (E I w'''')'' = ω² ρ A w is solved by
//! Rayleigh-Ritz on the clamped polynomials s², ..., s⁷, which gives the
//! first two modes to a small fraction of a per cent (a test holds it
//! against the uniform beam's closed form). The taper whose ratio matches is
//! found by bisection, then the root thickness that puts the first mode at
//! the measured frequency.
//!
//! A linear taper is an assumption about the shape, constrained by one
//! measured number; it is the simplest profile that can meet it.

// The linear algebra below is written with indices, as matrices are; the
// iterator forms clippy suggests would hide which element is which.
#![allow(clippy::needless_range_loop)]

use crate::math;

const BASIS: usize = 6;
const QUADRATURE: usize = 400;
/// Samples of the mode shape along the tongue.
pub const SPAN_POINTS: usize = 65;

/// A tongue's first mode: its shape, and what the reed model needs from it.
#[derive(Debug, Clone, Copy)]
pub struct TongueMode {
    /// The ratio this mode was solved for, as asked (the family may not
    /// reach it; see [`Self::ratio`] for what it did reach).
    pub ratio_asked: f64,
    /// The linear taper c: 0 is a uniform tongue.
    pub taper: f64,
    /// f₂ / f₁.
    pub ratio: f64,
    /// λ₁ = ω₁² ρ A₀ L⁴ / (E I₀), with A₀, I₀ at the root.
    pub eigenvalue: f64,
    /// ψ(s) at [`SPAN_POINTS`] evenly spaced positions, ψ(1) = 1.
    pub shape: [f64; SPAN_POINTS],
    /// Relative thickness t(s) / t₀ at the same positions.
    pub thickness: [f64; SPAN_POINTS],
    /// ∫ψ ds.
    pub shape_integral: f64,
    /// ∫ (t/t₀) ψ² ds: the modal mass over the mass a root-thick tongue
    /// would have.
    pub mass_integral: f64,
    /// ∫ψ'² ds.
    pub slope_squared: f64,
    /// ψ'(1).
    pub tip_slope: f64,
    /// The second bending mode, ψ₂(s), likewise sampled and ψ₂(1) = 1: not
    /// in the shipping reed, kept for the experiments that test it.
    pub second_shape: [f64; SPAN_POINTS],
    /// ∫ψ₂ ds.
    pub second_shape_integral: f64,
    /// ∫ (t/t₀) ψ₂² ds.
    pub second_mass_integral: f64,
}

fn integrate(f: impl Fn(f64) -> f64) -> f64 {
    let h = 1.0 / QUADRATURE as f64;
    let mut sum = f(0.0) + f(1.0);
    for i in 1..QUADRATURE {
        sum += f(i as f64 * h) * if i % 2 == 1 { 4.0 } else { 2.0 };
    }
    sum * h / 3.0
}

/// The basis functions s^(k+2) and their first and second derivatives.
fn basis(k: usize, s: f64) -> (f64, f64, f64) {
    let n = (k + 2) as i32;
    let nf = n as f64;
    (
        pow_i(s, n),
        nf * pow_i(s, n - 1),
        nf * (nf - 1.0) * pow_i(s, n - 2),
    )
}

fn pow_i(x: f64, n: i32) -> f64 {
    let mut result = 1.0;
    for _ in 0..n {
        result *= x;
    }
    result
}

type Matrix = [[f64; BASIS]; BASIS];

/// The two lowest eigenpairs of K a = λ M a, by Cholesky reduction and
/// Jacobi rotations.
fn lowest_modes(k: &Matrix, m: &Matrix) -> [(f64, [f64; BASIS]); 2] {
    // M = L Lᵀ.
    let mut l = [[0.0; BASIS]; BASIS];
    for i in 0..BASIS {
        for j in 0..=i {
            let mut sum = m[i][j];
            for p in 0..j {
                sum -= l[i][p] * l[j][p];
            }
            l[i][j] = if i == j {
                math::sqrt(sum)
            } else {
                sum / l[j][j]
            };
        }
    }
    // C = L⁻¹ K L⁻ᵀ, column by column.
    let solve_lower = |b: [f64; BASIS]| {
        let mut x = [0.0; BASIS];
        for i in 0..BASIS {
            let mut sum = b[i];
            for p in 0..i {
                sum -= l[i][p] * x[p];
            }
            x[i] = sum / l[i][i];
        }
        x
    };
    let mut y = [[0.0; BASIS]; BASIS]; // L⁻¹ K
    for j in 0..BASIS {
        let column = solve_lower(core::array::from_fn(|i| k[i][j]));
        for i in 0..BASIS {
            y[i][j] = column[i];
        }
    }
    let mut c = [[0.0; BASIS]; BASIS]; // (L⁻¹ (L⁻¹ K)ᵀ)ᵀ
    for i in 0..BASIS {
        c[i] = solve_lower(y[i]);
    }
    for i in 0..BASIS {
        for j in 0..i {
            let mean = 0.5 * (c[i][j] + c[j][i]);
            c[i][j] = mean;
            c[j][i] = mean;
        }
    }
    // Jacobi rotations until the off-diagonal is gone.
    let mut v = [[0.0; BASIS]; BASIS];
    for (i, row) in v.iter_mut().enumerate() {
        row[i] = 1.0;
    }
    for _ in 0..100 {
        let mut off = 0.0;
        for i in 0..BASIS {
            for j in 0..BASIS {
                if i != j {
                    off += c[i][j] * c[i][j];
                }
            }
        }
        if off < 1e-24 * (1.0 + c[0][0] * c[0][0]) {
            break;
        }
        for p in 0..BASIS {
            for q in p + 1..BASIS {
                if c[p][q].abs() < 1e-300 {
                    continue;
                }
                let theta = (c[q][q] - c[p][p]) / (2.0 * c[p][q]);
                let t = theta.signum() / (theta.abs() + math::sqrt(theta * theta + 1.0));
                let t = if theta == 0.0 { 1.0 } else { t };
                let cs = 1.0 / math::sqrt(t * t + 1.0);
                let sn = t * cs;
                for r in 0..BASIS {
                    let (a, b) = (c[r][p], c[r][q]);
                    c[r][p] = cs * a - sn * b;
                    c[r][q] = sn * a + cs * b;
                }
                for r in 0..BASIS {
                    let (a, b) = (c[p][r], c[q][r]);
                    c[p][r] = cs * a - sn * b;
                    c[q][r] = sn * a + cs * b;
                }
                for row in v.iter_mut() {
                    let (a, b) = (row[p], row[q]);
                    row[p] = cs * a - sn * b;
                    row[q] = sn * a + cs * b;
                }
            }
        }
    }
    let mut order: [usize; BASIS] = core::array::from_fn(|i| i);
    order.sort_unstable_by(|a, b| c[*a][*a].total_cmp(&c[*b][*b]));
    let back = |column: usize| {
        // a = L⁻ᵀ y.
        let yv: [f64; BASIS] = core::array::from_fn(|i| v[i][column]);
        let mut a = [0.0; BASIS];
        for i in (0..BASIS).rev() {
            let mut sum = yv[i];
            for p in i + 1..BASIS {
                sum -= l[p][i] * a[p];
            }
            a[i] = sum / l[i][i];
        }
        a
    };
    [
        (c[order[0]][order[0]], back(order[0])),
        (c[order[1]][order[1]], back(order[1])),
    ]
}

impl TongueMode {
    /// Solves the tongue with linear taper `taper` (0 ≤ c < 1).
    pub fn with_taper(taper: f64) -> Self {
        let taper = taper.clamp(0.0, 0.95);
        let relative = |s: f64| 1.0 - taper * s;
        let mut k = [[0.0; BASIS]; BASIS];
        let mut m = [[0.0; BASIS]; BASIS];
        for i in 0..BASIS {
            for j in 0..=i {
                let stiffness = integrate(|s| {
                    let t = relative(s);
                    t * t * t * basis(i, s).2 * basis(j, s).2
                });
                let mass = integrate(|s| relative(s) * basis(i, s).0 * basis(j, s).0);
                k[i][j] = stiffness;
                k[j][i] = stiffness;
                m[i][j] = mass;
                m[j][i] = mass;
            }
        }
        let [(first, a), (second, a2)] = lowest_modes(&k, &m);
        let at = |coefficients: &[f64; BASIS], s: f64| -> (f64, f64) {
            let mut value = 0.0;
            let mut slope = 0.0;
            for (index, coefficient) in coefficients.iter().enumerate() {
                let (b, db, _) = basis(index, s);
                value += coefficient * b;
                slope += coefficient * db;
            }
            (value, slope)
        };
        let tip = at(&a, 1.0).0;
        let normalised = |s: f64| {
            let (value, slope) = at(&a, s);
            (value / tip, slope / tip)
        };
        let tip2 = at(&a2, 1.0).0;
        let second_at = |s: f64| at(&a2, s).0 / tip2;
        let mut shape = [0.0; SPAN_POINTS];
        let mut second_shape = [0.0; SPAN_POINTS];
        let mut thickness = [0.0; SPAN_POINTS];
        for i in 0..SPAN_POINTS {
            let s = i as f64 / (SPAN_POINTS - 1) as f64;
            shape[i] = normalised(s).0;
            second_shape[i] = second_at(s);
            thickness[i] = relative(s);
        }
        let ratio = math::sqrt(second / first);
        Self {
            ratio_asked: ratio,
            taper,
            ratio,
            eigenvalue: first,
            shape,
            thickness,
            shape_integral: integrate(|s| normalised(s).0),
            mass_integral: integrate(|s| relative(s) * normalised(s).0 * normalised(s).0),
            slope_squared: integrate(|s| {
                let slope = normalised(s).1;
                slope * slope
            }),
            tip_slope: normalised(1.0).1,
            second_shape,
            second_shape_integral: integrate(second_at),
            second_mass_integral: integrate(|s| relative(s) * second_at(s) * second_at(s)),
        }
    }

    /// The linear taper whose first two modes stand in `ratio`, clamped to
    /// what the family can reach.
    pub fn with_ratio(ratio: f64) -> Self {
        let mut mode = Self::solve_ratio(ratio);
        mode.ratio_asked = ratio;
        mode
    }

    fn solve_ratio(ratio: f64) -> Self {
        let uniform = Self::with_taper(0.0);
        if ratio >= uniform.ratio {
            return uniform;
        }
        let steepest = Self::with_taper(0.95);
        if ratio <= steepest.ratio {
            return steepest;
        }
        // The ratio falls monotonically with the taper.
        let (mut low, mut high) = (0.0, 0.95);
        for _ in 0..40 {
            let middle = 0.5 * (low + high);
            if Self::with_taper(middle).ratio > ratio {
                low = middle;
            } else {
                high = middle;
            }
        }
        Self::with_taper(0.5 * (low + high))
    }

    /// The root thickness, m, that puts the first mode at `frequency` for a
    /// tongue of `length` in a metal of Young's modulus `modulus` and
    /// `density`: ω₁² = λ₁ E t₀² / (12 ρ L⁴).
    pub fn root_thickness(&self, frequency: f64, length: f64, modulus: f64, density: f64) -> f64 {
        let omega = 2.0 * core::f64::consts::PI * frequency;
        omega * length * length * math::sqrt(12.0 * density / (modulus * self.eigenvalue))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_uniform_tongue_is_the_textbook_beam() {
        let mode = TongueMode::with_taper(0.0);
        let beta4 = 1.875_104_068_711_961f64.powi(4);
        assert!(
            (mode.eigenvalue / beta4 - 1.0).abs() < 1e-5,
            "{}",
            mode.eigenvalue
        );
        assert!((mode.ratio - 6.267).abs() < 0.01, "{}", mode.ratio);
        assert!((mode.shape[SPAN_POINTS - 1] - 1.0).abs() < 1e-12);
        assert!(
            (mode.mass_integral - 0.25).abs() < 1e-4,
            "{}",
            mode.mass_integral
        );
        assert!(
            (mode.shape_integral - 0.391_496).abs() < 1e-4,
            "{}",
            mode.shape_integral
        );
        assert!((mode.tip_slope - 1.3765).abs() < 2e-3, "{}", mode.tip_slope);
    }

    #[test]
    fn a_taper_brings_the_modes_together_and_the_measured_ratio_is_reached() {
        let mut previous = f64::MAX;
        for taper in [0.0, 0.2, 0.4, 0.6, 0.8] {
            let ratio = TongueMode::with_taper(taper).ratio;
            assert!(ratio < previous, "taper {taper}: {ratio}");
            previous = ratio;
        }
        let measured = TongueMode::with_ratio(4.6);
        assert!((measured.ratio - 4.6).abs() < 1e-3, "{measured:?}");
    }
}
