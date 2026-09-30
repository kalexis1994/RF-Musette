//! Measuring the reed: what the tests assert and the laboratory reports.
//!
//! Everything here drives the engine's own reed step, so what is measured is
//! the model that ships, not a copy of it.

use rf_musette_dsp::pallet::{Pallet, PalletDesign};
use rf_musette_dsp::reed::{self, ReedDesign, ReedModel, ReedState};

/// One simulated reed, sampled at the rate it was computed at.
pub struct Trace {
    pub rate: f64,
    /// Tip displacement from rest, m.
    pub zeta: Vec<f64>,
    /// Rate of change of the volume flow, m³/s²: the radiated sound, up to
    /// a constant.
    pub flow_rate: Vec<f64>,
}

/// Blows `design` with the supply pressure `supply(t)` (Pa) for `seconds`,
/// at `rate` samples per second, from rest.
pub fn simulate(design: ReedDesign, rate: f64, seconds: f64, supply: impl Fn(f64) -> f64) -> Trace {
    let model = ReedModel::new(design);
    let mut state = ReedState::default();
    let frames = (seconds * rate) as usize;
    let h = 1.0 / rate;
    let mut trace = Trace {
        rate,
        zeta: Vec::with_capacity(frames),
        flow_rate: Vec::with_capacity(frames),
    };
    for n in 0..frames {
        let t = (n as f64 + 0.5) * h;
        let rate_of_flow = reed::step(&model, &mut state, supply(t), f64::INFINITY, h);
        trace.zeta.push(state.zeta);
        trace.flow_rate.push(rate_of_flow);
    }
    trace
}

/// Blows `design` at a steady supply `pressure` (Pa) -- the bellows already
/// pressed -- while the key is taken to `depth(t)` (0 up, 1 fully down) and
/// the pallet follows it as `pallet` says. From rest.
pub fn simulate_keyed(
    design: ReedDesign,
    pallet: PalletDesign,
    rate: f64,
    seconds: f64,
    pressure: f64,
    depth: impl Fn(f64) -> f64,
) -> Trace {
    let model = ReedModel::new(design);
    let mut state = ReedState::default();
    let mut valve = Pallet::default();
    let frames = (seconds * rate) as usize;
    let h = 1.0 / rate;
    let mut trace = Trace {
        rate,
        zeta: Vec::with_capacity(frames),
        flow_rate: Vec::with_capacity(frames),
    };
    for n in 0..frames {
        valve.press(depth(n as f64 * h));
        valve.advance(&pallet, h);
        let area = valve.area(&pallet, design.tone_hole_area);
        let rate_of_flow = reed::step(&model, &mut state, pressure, area, h);
        trace.zeta.push(state.zeta);
        trace.flow_rate.push(rate_of_flow);
    }
    trace
}

/// The level of one component of `signal` over time, dB: a Hann-windowed
/// DFT at `frequency` over `periods` periods, every `hop` seconds. Returns
/// (time at the window's centre, level) pairs.
pub fn component_envelope(
    signal: &[f64],
    rate: f64,
    frequency: f64,
    periods: f64,
    hop: f64,
) -> Vec<(f64, f64)> {
    let length = (periods * rate / frequency) as usize;
    let step = ((hop * rate) as usize).max(1);
    let omega = 2.0 * std::f64::consts::PI * frequency / rate;
    let mut envelope = Vec::new();
    let mut start = 0;
    while start + length <= signal.len() {
        let (mut re, mut im) = (0.0, 0.0);
        for i in 0..length {
            let w = 0.5 - 0.5 * (2.0 * std::f64::consts::PI * i as f64 / length as f64).cos();
            let x = w * signal[start + i];
            let phase = omega * (start + i) as f64;
            re += x * phase.cos();
            im -= x * phase.sin();
        }
        let level = 20.0 * ((re * re + im * im).sqrt() + 1e-300).log10();
        envelope.push(((start + length / 2) as f64 / rate, level));
        start += step;
    }
    envelope
}

/// Llanos-Vázquez et al.'s attack time (Acta Acustica 100, 2014): from the
/// first harmonic's first reaching -50 dB to its first reaching -5 dB, both
/// relative to its steady level, here the mean over the envelope's last
/// fifth. `None` if either level is never reached.
pub fn attack_time(envelope: &[(f64, f64)]) -> Option<f64> {
    let tail = &envelope[envelope.len() * 4 / 5..];
    let steady = tail.iter().map(|(_, level)| level).sum::<f64>() / tail.len() as f64;
    let start = envelope
        .iter()
        .find(|(_, level)| *level >= steady - 50.0)?
        .0;
    let end = envelope.iter().find(|(_, level)| *level >= steady - 5.0)?.0;
    Some(end - start)
}

/// What a steady tone looks like, measured over a window.
#[derive(Debug, Clone, Copy)]
pub struct Tone {
    /// Hz, from interpolated upward zero crossings of the displacement
    /// about its mean.
    pub frequency: f64,
    /// Half the peak-to-peak tip displacement, m.
    pub amplitude: f64,
    /// Mean tip displacement from rest, m (positive into the slot).
    pub mean: f64,
    /// RMS of the flow's rate of change, m³/s².
    pub flow_rate_rms: f64,
}

impl Trace {
    /// Measures the window from `start` to `end` seconds. `None` when the
    /// reed is not oscillating there.
    pub fn tone(&self, start: f64, end: f64) -> Option<Tone> {
        let a = (start * self.rate) as usize;
        let b = ((end * self.rate) as usize).min(self.zeta.len());
        let window = self.zeta.get(a..b)?;
        let mean = window.iter().sum::<f64>() / window.len() as f64;
        let (low, high) = window
            .iter()
            .fold((f64::MAX, f64::MIN), |(lo, hi), z| (lo.min(*z), hi.max(*z)));
        let amplitude = 0.5 * (high - low);
        if amplitude < 1.0e-6 {
            return None;
        }
        let mut crossings = Vec::new();
        for i in 1..window.len() {
            let (x0, x1) = (window[i - 1] - mean, window[i] - mean);
            if x0 < 0.0 && x1 >= 0.0 {
                crossings.push((i - 1) as f64 + x0 / (x0 - x1));
            }
        }
        if crossings.len() < 3 {
            return None;
        }
        let periods = (crossings.len() - 1) as f64;
        let frequency = periods * self.rate / (crossings[crossings.len() - 1] - crossings[0]);
        let flow = &self.flow_rate[a..b];
        let flow_rate_rms = (flow.iter().map(|x| x * x).sum::<f64>() / flow.len() as f64).sqrt();
        Some(Tone {
            frequency,
            amplitude,
            mean,
            flow_rate_rms,
        })
    }

    /// Levels of the first `count` harmonics of `signal` over a window, dB
    /// relative to the first, by a Hann-windowed DFT at each harmonic of
    /// `frequency`.
    pub fn harmonics(signal: &[f64], rate: f64, frequency: f64, count: usize) -> Vec<f64> {
        let n = signal.len();
        let mut levels = Vec::with_capacity(count);
        for k in 1..=count {
            let omega = 2.0 * std::f64::consts::PI * frequency * k as f64 / rate;
            let (mut re, mut im) = (0.0, 0.0);
            for (i, x) in signal.iter().enumerate() {
                let w = 0.5 - 0.5 * (2.0 * std::f64::consts::PI * i as f64 / n as f64).cos();
                re += w * x * (omega * i as f64).cos();
                im -= w * x * (omega * i as f64).sin();
            }
            levels.push((re * re + im * im).sqrt());
        }
        let first = levels[0].max(1e-300);
        levels.iter().map(|l| 20.0 * (l / first).log10()).collect()
    }
}

/// Cents between two frequencies.
pub fn cents(from: f64, to: f64) -> f64 {
    1200.0 * (to / from).log2()
}

/// The steady tone at a constant supply pressure: `seconds` of blowing,
/// measured over the last `window`.
pub fn steady(
    design: ReedDesign,
    rate: f64,
    pressure: f64,
    seconds: f64,
    window: f64,
) -> Option<(Tone, Trace)> {
    let trace = simulate(design, rate, seconds, |_| pressure);
    let tone = trace.tone(seconds - window, seconds)?;
    Some((tone, trace))
}

/// Growth rate, 1/s, of a small disturbance of the reed's equilibrium under
/// a steady `pressure`: positive means the reed will speak from rest.
///
/// The equilibrium is exact for this model: with the flow steady the whole
/// supply falls across the reed, Δp = P, the tongue sits at ζ = μP/ω0², and
/// the jet carries u = α S_u(ζ) √(2P/ρ). The disturbance is one micrometre
/// of tip displacement; the rate is read from its envelope over 40 periods.
pub fn growth_rate(design: ReedDesign, rate: f64, pressure: f64) -> f64 {
    let model = ReedModel::new(design);
    let mut state = ReedState::equilibrium(&model, pressure);
    let zeta = state.zeta;
    state.zeta += 1.0e-6;
    let h = 1.0 / rate;
    let period = (rate / design.frequency) as usize;
    let envelope = |state: &mut ReedState, periods: usize| {
        let mut peak = 0.0f64;
        for _ in 0..periods * period {
            reed::step(&model, state, pressure, f64::INFINITY, h);
            peak = peak.max((state.zeta - zeta).abs());
        }
        peak
    };
    // Let the start-up transient pass, then compare two windows.
    envelope(&mut state, 10);
    let first = envelope(&mut state, 5);
    envelope(&mut state, 30);
    let second = envelope(&mut state, 5);
    let elapsed = 35.0 * period as f64 * h;
    (second.max(1e-30) / first.max(1e-30)).ln() / elapsed
}

/// The lowest supply pressure, Pa, at which the equilibrium is unstable --
/// where the reed speaks from rest -- between `low` and `high`. The range is
/// scanned upward in steps of a tenth of an octave (the unstable region need
/// not reach `high`), then the first crossing is bisected. `None` if the
/// equilibrium is stable throughout.
pub fn linear_threshold(design: ReedDesign, rate: f64, low: f64, high: f64) -> Option<f64> {
    if growth_rate(design, rate, low) > 0.0 {
        return Some(low);
    }
    let step = 2f64.powf(0.1);
    let mut below = low;
    let mut above = None;
    let mut pressure = low * step;
    while pressure <= high {
        if growth_rate(design, rate, pressure) > 0.0 {
            above = Some(pressure);
            break;
        }
        below = pressure;
        pressure *= step;
    }
    let (mut a, mut b) = (below, above?);
    for _ in 0..30 {
        let middle = (a * b).sqrt();
        if growth_rate(design, rate, middle) > 0.0 {
            b = middle;
        } else {
            a = middle;
        }
        if b / a < 1.01 {
            break;
        }
    }
    Some(b)
}

/// The pressure, Pa, at which a sounding reed falls silent as the supply is
/// let down slowly: blown at `start` for a second, then ramped to zero over
/// `seconds`. Silent means less than `sounding` metres half peak-to-peak
/// over ten nominal periods. `None` if it never sounded, or never stopped.
pub fn offset_pressure(
    design: ReedDesign,
    rate: f64,
    start: f64,
    seconds: f64,
    sounding: f64,
) -> Option<f64> {
    let supply = |t: f64| {
        if t < 1.0 {
            start
        } else {
            (start * (1.0 - (t - 1.0) / seconds)).max(0.0)
        }
    };
    // Two seconds at zero after the ramp, so a reed that sounds all the way
    // down has time to die out and be counted.
    let trace = simulate(design, rate, 3.0 + seconds, supply);
    let span = ((10.0 / design.frequency) * rate) as usize;
    let amplitude_at = |i: usize| {
        let window = &trace.zeta[i - span..i];
        let (low, high) = window
            .iter()
            .fold((f64::MAX, f64::MIN), |(lo, hi), z| (lo.min(*z), hi.max(*z)));
        0.5 * (high - low)
    };
    if amplitude_at((rate * 1.0) as usize) < sounding {
        return None;
    }
    let mut i = (rate * 1.0) as usize;
    while i < trace.zeta.len() {
        if amplitude_at(i) < sounding {
            return Some(supply(i as f64 / rate));
        }
        i += span / 4;
    }
    None
}

/// The same equations as `reed::step`, integrated independently: classical
/// RK4 on the exact Bernoulli law, at whatever small step the caller gives.
/// A check that what the shipping scheme computes is the model, not the
/// scheme. Returns the steady tone over the last third of `seconds`.
pub fn reference_tone(design: ReedDesign, rate: f64, pressure: f64, seconds: f64) -> Option<Tone> {
    let model = ReedModel::new(design);
    let d = model.design;
    let derivative = |s: [f64; 5]| -> [f64; 5] {
        let [zeta, w, u, a, p] = s;
        let jet = u - model.effective_area * w;
        let v = jet / (d.contraction * model.section(zeta));
        let dp = 0.5 * reed::AIR_DENSITY * v * v.abs();
        [
            w,
            -model.omega / d.q * w - model.omega * model.omega * zeta + model.mu * dp,
            (p - dp) / model.inertance,
            (pressure - p) / model.hole_inertance,
            (a - u) / model.cell_compliance,
        ]
    };
    let h = 1.0 / rate;
    let frames = (seconds * rate) as usize;
    let mut s = [0.0; 5];
    let mut trace = Trace {
        rate,
        zeta: Vec::with_capacity(frames),
        flow_rate: Vec::with_capacity(frames),
    };
    for _ in 0..frames {
        let k1 = derivative(s);
        let k2 = derivative(core::array::from_fn(|i| s[i] + 0.5 * h * k1[i]));
        let k3 = derivative(core::array::from_fn(|i| s[i] + 0.5 * h * k2[i]));
        let k4 = derivative(core::array::from_fn(|i| s[i] + h * k3[i]));
        for i in 0..5 {
            s[i] += h / 6.0 * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]);
        }
        trace.zeta.push(s[0]);
        trace.flow_rate.push(k1[3]);
    }
    trace.tone(seconds * 2.0 / 3.0, seconds)
}
