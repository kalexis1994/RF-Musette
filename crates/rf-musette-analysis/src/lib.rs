//! Measuring the reed: what the tests assert and the laboratory reports.
//!
//! Everything here drives the engine's own reed step, so what is measured is
//! the model that ships, not a copy of it.

pub mod scenes;

use rf_musette_dsp::pallet::{Pallet, PalletDesign};
use rf_musette_dsp::reed::{self, ReedDesign, ReedModel, ReedState, Tube};

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
    let mut tube = Tube::default();
    let frames = (seconds * rate) as usize;
    let h = 1.0 / rate;
    let mut trace = Trace {
        rate,
        zeta: Vec::with_capacity(frames),
        flow_rate: Vec::with_capacity(frames),
    };
    for n in 0..frames {
        let t = (n as f64 + 0.5) * h;
        let rate_of_flow = reed::step(&model, &mut state, &mut tube, supply(t), f64::INFINITY, h);
        trace.zeta.push(state.zeta);
        trace.flow_rate.push(rate_of_flow);
    }
    trace
}

/// Blows `design`, its pallet fully open, from a bellows that at audio
/// frequencies is only its air -- a compliance of `volume` m³ -- while the
/// arm delivers the air the reed draws on average (over `mean_time` s) and
/// brings the mean back to `ask` Pa over `regulation` seconds. From rest,
/// the bellows at `ask`. The diagnosis tool that found the bass's coupling
/// to the bellows (docs/ROADMAP.md, 8e), before the engine's bellows was
/// made so; [`simulate_bellows`] drives the engine's own.
#[allow(clippy::too_many_arguments)]
pub fn simulate_fed(
    design: ReedDesign,
    pallet: PalletDesign,
    rate: f64,
    seconds: f64,
    ask: f64,
    volume: f64,
    regulation: f64,
    mean_time: f64,
) -> Trace {
    let compliance = volume / (reed::AIR_DENSITY * reed::SPEED_OF_SOUND * reed::SPEED_OF_SOUND);
    let open = Pallet {
        position: 1.0,
        target: 1.0,
    };
    let area = open.area(&pallet, design.tone_hole_area);
    let model = ReedModel::new(design);
    let mut state = ReedState::default();
    let mut tube = Tube::default();
    let mut bellows = ask;
    let mut mean_draw = 0.0;
    let frames = (seconds * rate) as usize;
    let h = 1.0 / rate;
    let mut trace = Trace {
        rate,
        zeta: Vec::with_capacity(frames),
        flow_rate: Vec::with_capacity(frames),
    };
    for _ in 0..frames {
        let rate_of_flow = reed::step(&model, &mut state, &mut tube, bellows, area, h);
        // C P' = Q̄ + (ask − P) C / τ − Q_hole: the arm delivers the mean
        // draw and slowly restores the mean pressure; the hole draws.
        mean_draw += h / mean_time * (state.hole_flow - mean_draw);
        bellows += h * ((ask - bellows) / regulation + (mean_draw - state.hole_flow) / compliance);
        bellows = bellows.max(0.0);
        trace.zeta.push(state.zeta);
        trace.flow_rate.push(rate_of_flow);
    }
    trace
}

/// Blows `design` from the engine's own bellows -- the arm asking `ask` Pa,
/// the parameters' bellows -- its pallet fully open, from rest, the bellows
/// at `ask`. What the reed sees is what it sees in the engine.
pub fn simulate_bellows(
    parameters: &rf_musette_dsp::Parameters,
    design: ReedDesign,
    rate: f64,
    seconds: f64,
    ask: f64,
) -> Trace {
    let mut arm = *parameters;
    arm.set(
        rf_musette_dsp::parameters::BELLOWS_RESPONSE,
        rf_musette_dsp::parameters::ARM,
    );
    let wind_design = arm.wind_design().expect("the arm's bellows");
    let mut wind = rf_musette_dsp::wind::Wind {
        pressure: ask,
        mean: 0.0,
    };
    let open = Pallet {
        position: 1.0,
        target: 1.0,
    };
    let area = open.area(&parameters.pallet_design(), design.tone_hole_area);
    let model = ReedModel::new(design);
    let mut state = ReedState::default();
    let mut tube = Tube::default();
    let frames = (seconds * rate) as usize;
    let h = 1.0 / rate;
    let mut trace = Trace {
        rate,
        zeta: Vec::with_capacity(frames),
        flow_rate: Vec::with_capacity(frames),
    };
    for _ in 0..frames {
        let supply = wind.step(&wind_design, ask, state.hole_flow, h);
        let rate_of_flow = reed::step(&model, &mut state, &mut tube, supply, area, h);
        trace.zeta.push(state.zeta);
        trace.flow_rate.push(rate_of_flow);
    }
    trace
}

/// Whether `design`, its mode on `aim`, speaks from rest on the engine's
/// bellows asked each of `pressures`.
pub fn speaks_on_bellows(
    parameters: &rf_musette_dsp::Parameters,
    mut design: ReedDesign,
    aim: f64,
    pressures: &[f64],
) -> bool {
    design.frequency = aim;
    let seconds = (600.0 / aim).clamp(1.5, 6.0);
    pressures.iter().all(|pressure| {
        let trace = simulate_bellows(parameters, design, 96_000.0, seconds, *pressure);
        speaks(&trace, seconds, design.set)
    })
}

/// The pitch below which a reed is expected to speak from 50 Pa to the
/// bellows' ceiling, Hz: the low reeds, the ones a maker loads.
pub const LOW_REED: f64 = 300.0;

/// How long a low reed's inlet duct must be, as a multiple of the tone
/// hole's depth, for it to speak on the bellows -- found as a maker finds
/// it, by trying (docs/ROADMAP.md, 8e). Seen from the reed, the bellows' air
/// behind its hole is a cavity below the hole and bellows' resonance, which
/// "adds friction" (Fletcher & Rossing, after Llanos p236); a longer duct,
/// more inertance, moves the resonance under the reed's pitch. From 50 Pa if
/// it can, else the least pressure it can, to 300 Pa and the ceiling; the
/// least duct, raised by quarters, the bracket halved four times. 1 for a
/// reed that needs none, or that is not a low reed speaking across the range
/// from an ideal pressure. `None` if no duct up to forty times makes it.
pub fn duct_reed(
    parameters: &rf_musette_dsp::Parameters,
    design: ReedDesign,
    aim: f64,
) -> Option<f64> {
    let ceiling = parameters
        .get(rf_musette_dsp::parameters::BELLOWS_CEILING)
        .unwrap_or(1000.0);
    if aim >= LOW_REED || !holds(design, aim, &[TUNING_PRESSURE, ceiling]) {
        return Some(1.0);
    }
    let speaks_with = |duct: f64, pressures: &[f64]| {
        let trial = ReedDesign {
            tone_hole_depth: design.tone_hole_depth * duct,
            ..design
        };
        speaks_on_bellows(parameters, trial, aim, pressures)
    };
    [50.0, 100.0, 150.0, 200.0, TUNING_PRESSURE]
        .into_iter()
        .find_map(|floor| {
            // The ceiling first: it is where a soft reed fails.
            let pressures = [ceiling, floor, TUNING_PRESSURE.max(floor)];
            if speaks_with(1.0, &pressures) {
                return Some(1.0);
            }
            let (mut low, mut high) = (1.0, 1.25);
            while !speaks_with(high, &pressures) {
                low = high;
                high *= 1.25;
                if high > 40.0 {
                    return None;
                }
            }
            for _ in 0..4 {
                let middle = (low * high).sqrt();
                if speaks_with(middle, &pressures) {
                    high = middle;
                } else {
                    low = middle;
                }
            }
            Some(high)
        })
}

/// Whether a trace of `seconds` from rest shows a reed that speaks: a tone
/// in its last half second at least as wide as its set, and not dying -- no
/// smaller than 0.98 of the half second a second before. A tone alone is not
/// enough: a reed started by the pressure's step and slowly dying still has
/// one after seconds (docs/ROADMAP.md, 8d).
pub fn speaks(trace: &Trace, seconds: f64, set: f64) -> bool {
    let (Some(last), Some(before)) = (
        trace.tone(seconds - 0.5, seconds),
        trace.tone(seconds - 1.5, seconds - 1.0),
    ) else {
        return false;
    };
    last.amplitude >= set && last.amplitude >= 0.98 * before.amplitude
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
    let mut tube = Tube::default();
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
        let rate_of_flow = reed::step(&model, &mut state, &mut tube, pressure, area, h);
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
    let mut tube = Tube::steady(pressure);
    let zeta = state.zeta;
    state.zeta += 1.0e-6;
    let h = 1.0 / rate;
    let period = (rate / design.frequency) as usize;
    let envelope = |state: &mut ReedState, tube: &mut Tube, periods: usize| {
        let mut peak = 0.0f64;
        for _ in 0..periods * period {
            reed::step(&model, state, tube, pressure, f64::INFINITY, h);
            peak = peak.max((state.zeta - zeta).abs());
        }
        peak
    };
    // Let the start-up transient pass, then compare two windows.
    envelope(&mut state, &mut tube, 10);
    let first = envelope(&mut state, &mut tube, 5);
    envelope(&mut state, &mut tube, 30);
    let second = envelope(&mut state, &mut tube, 5);
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
///
/// The cell is a tube here as in the scheme (milestone 8m), but solved
/// another way: by the method of lines, its air in [`REFERENCE_SEGMENTS`]
/// segments -- a pressure at each joint, the ends' joints holding half a
/// segment of air, a flow through each segment -- the hole's flow entering
/// at the opening, the closed end passing nothing, the slot's points taking
/// their air from the joints either side of them in proportion and feeling
/// their pressure the same way. The walls' losses are the scheme's: one
/// resistance in series with the hole; and the hole's radiating end its
/// mass beside ρc/A, the flow through the mass one more unknown.
pub fn reference_tone(design: ReedDesign, rate: f64, pressure: f64, seconds: f64) -> Option<Tone> {
    const N: usize = REFERENCE_SEGMENTS;
    const ZETA: usize = 0;
    const W: usize = 1;
    const U: usize = 2;
    const A: usize = 3;
    const RADIATING: usize = 4;
    const NEAR: usize = 5;
    const P: usize = 6;
    const Q: usize = P + N + 1;
    const SIZE: usize = Q + N;
    let model = ReedModel::new(design);
    let d = model.design;
    let c2 = reed::SPEED_OF_SOUND * reed::SPEED_OF_SOUND;
    let length = model.tube_seconds * reed::SPEED_OF_SOUND;
    let section = d.cell_volume / length;
    let dx = length / N as f64;
    let mass = reed::AIR_DENSITY * dx / section;
    let compliance = |joint: usize| {
        let share = if joint == 0 || joint == N { 0.5 } else { 1.0 };
        share * section * dx / (reed::AIR_DENSITY * c2)
    };
    // Each slot point: its joint below, the share of the one above, and
    // its weight.
    let points: Vec<(usize, f64, f64)> = model
        .slot()
        .map(|(at, weight)| {
            let x = at * N as f64;
            let below = (x as usize).min(N - 1);
            (below, x - below as f64, weight)
        })
        .collect();
    let derivative = |s: &[f64; SIZE]| -> [f64; SIZE] {
        let (zeta, w, a) = (s[ZETA], s[W], s[A]);
        let p = points
            .iter()
            .map(|(below, share, weight)| {
                weight * ((1.0 - share) * s[P + below] + share * s[P + below + 1])
            })
            .sum::<f64>();
        // The slot's flow: the jet and the far side's radiation in series,
        // c j|j| + R_s j = p - R_s (S_r w - m), j the jet's flow -- solved
        // exactly, the left side rising with j.
        let alpha_section = d.contraction * model.section(zeta);
        let c = 0.5 * reed::AIR_DENSITY / (alpha_section * alpha_section);
        let r_s = model.slot_radiation;
        let q = p - r_s * (model.effective_area * w - s[NEAR]);
        let jet = q.signum() * (-r_s + (r_s * r_s + 4.0 * c * q.abs()).sqrt()) / (2.0 * c);
        let u = jet + model.effective_area * w;
        let v = jet / alpha_section;
        let dp = 0.5 * reed::AIR_DENSITY * v * v.abs();
        // The voiced swing limit, as `reed::step` documents it.
        let lift = zeta / d.width;
        let speed = (2.0 * p.max(0.0) / reed::AIR_DENSITY).sqrt();
        let limit = d.swing_limit * reed::AIR_DENSITY * speed * d.width * d.length * lift * lift;
        let damping = model.omega / d.q + limit / model.modal_mass;
        let mut out = [0.0; SIZE];
        out[ZETA] = w;
        out[W] = -damping * w - model.omega * model.omega * zeta + model.mu * dp;
        // U is no longer a state: the slot's flow carries no mass of its own.
        out[U] = 0.0;
        out[NEAR] = r_s * (u - s[NEAR]) / model.inertance;
        let radiated = model.radiation_resistance * (a - s[RADIATING]);
        out[A] = (pressure - s[P] - model.wall_resistance * a - radiated) / model.hole_inertance;
        out[RADIATING] = radiated / model.radiation_mass;
        let mut given = [0.0; N + 1];
        for (below, share, weight) in &points {
            given[*below] -= weight * u * (1.0 - share);
            given[below + 1] -= weight * u * share;
        }
        for joint in 0..=N {
            let into = if joint == 0 { a } else { s[Q + joint - 1] };
            let out_of = if joint == N { 0.0 } else { s[Q + joint] };
            out[P + joint] = (into - out_of + given[joint]) / compliance(joint);
        }
        for segment in 0..N {
            out[Q + segment] = (s[P + segment] - s[P + segment + 1]) / mass;
        }
        out
    };
    let h = 1.0 / rate;
    let frames = (seconds * rate) as usize;
    let mut s = [0.0; SIZE];
    let mut trace = Trace {
        rate,
        zeta: Vec::with_capacity(frames),
        flow_rate: Vec::with_capacity(frames),
    };
    for _ in 0..frames {
        let k1 = derivative(&s);
        let k2 = derivative(&core::array::from_fn(|i| s[i] + 0.5 * h * k1[i]));
        let k3 = derivative(&core::array::from_fn(|i| s[i] + 0.5 * h * k2[i]));
        let k4 = derivative(&core::array::from_fn(|i| s[i] + h * k3[i]));
        for i in 0..SIZE {
            s[i] += h / 6.0 * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]);
        }
        trace.zeta.push(s[ZETA]);
        trace.flow_rate.push(k1[A]);
    }
    trace.tone(seconds * 2.0 / 3.0, seconds)
}

/// The segments of [`reference_tone`]'s tube: 40, for a 50 mm cell 1.3 mm
/// each, under a sixtieth of the wavelength at 4 kHz.
pub const REFERENCE_SEGMENTS: usize = 40;

/// Where a reed sounds, Hz, blown steadily at `pressure` from rest: long
/// enough for a low reed to settle, read over the last half second.
pub fn sounding(design: ReedDesign, pressure: f64) -> Option<Tone> {
    let seconds = (600.0 / design.frequency).clamp(1.5, 6.0);
    steady(design, 192_000.0, pressure, seconds, 0.5).map(|(tone, _)| tone)
}

/// The pressure the treble is tuned at, Pa: the IfM Zwota's playing pressure
/// for its comparisons (poster 2008, section 5.4).
pub const TUNING_PRESSURE: f64 = 300.0;

/// The pressure a reed is tuned at, Pa: [`TUNING_PRESSURE`], or for a reed
/// that does not speak there, the lowest of a few steps above it that it
/// speaks at -- as a tuner would have to. `None` if none does. Speaks: holds
/// a tone, as [`holds`] asks; a tone dying from the pressure's step is not
/// one to tune on, and moved a few cents it may be gone (milestone 8m).
pub fn tuning_pressure(design: ReedDesign) -> Option<f64> {
    [TUNING_PRESSURE, 400.0, 500.0, 700.0, 1000.0]
        .into_iter()
        .find(|pressure| holds(design, design.frequency, &[*pressure]))
}

/// The tuning table for `parameters`, as a tuner makes it: for every rank
/// and key, the cents a reed's mode must sit above its target so that it
/// sounds on it at its [`tuning_pressure`]. Two passes, the second
/// correcting the first. `None` in a cell whose reed does not speak.
pub fn tune(
    parameters: &rf_musette_dsp::Parameters,
) -> [[Option<f64>; rf_musette_dsp::compass::KEYS]; rf_musette_dsp::parameters::RANKS] {
    use rf_musette_dsp::compass::{FIRST_KEY, KEYS};
    let mut table = [[None; KEYS]; rf_musette_dsp::parameters::RANKS];
    for (rank, row) in table.iter_mut().enumerate() {
        for (index, cell) in row.iter_mut().enumerate() {
            *cell = tune_reed(parameters, FIRST_KEY + index as u8, rank);
        }
    }
    table
}

/// One cell of [`tune`]: the cents `key`'s `rank` reed's mode must sit above
/// its target.
pub fn tune_reed(parameters: &rf_musette_dsp::Parameters, key: u8, rank: usize) -> Option<f64> {
    use rf_musette_dsp::compass::{target, untuned};
    tune_design(
        untuned(parameters, key, rank)?,
        target(parameters, key, rank),
    )
}

/// The bass side's tuning table, as [`tune`] makes the treble's: for every
/// bass-side rank and pitch class, the cents its reed's mode must sit above
/// its target.
pub fn tune_bass(
    parameters: &rf_musette_dsp::Parameters,
) -> [[Option<f64>; rf_musette_dsp::compass::BASS_KEYS]; rf_musette_dsp::parameters::BASS_RANKS] {
    use rf_musette_dsp::compass::{BASS_KEYS, bass_target, bass_untuned};
    let mut table = [[None; BASS_KEYS]; rf_musette_dsp::parameters::BASS_RANKS];
    for (rank, row) in table.iter_mut().enumerate() {
        for (pitch_class, cell) in row.iter_mut().enumerate() {
            *cell = bass_untuned(parameters, pitch_class, rank).and_then(|design| {
                tune_design(design, bass_target(parameters, pitch_class, rank)?)
            });
        }
    }
    table
}

/// Whether `design`, its mode on `aim`, holds a tone from rest at each of
/// `pressures`.
pub fn holds(mut design: ReedDesign, aim: f64, pressures: &[f64]) -> bool {
    design.frequency = aim;
    let seconds = (600.0 / aim).clamp(1.5, 6.0);
    pressures.iter().all(|pressure| {
        let trace = simulate(design, 192_000.0, seconds, |_| *pressure);
        speaks(&trace, seconds, design.set)
    })
}

/// The least tip load, over the unloaded tongue's modal mass, that lets a
/// low reed hold a tone at 50 Pa, at 300 Pa and at the bellows' ceiling --
/// found as a maker finds it, by trying (docs/ROADMAP.md, milestone 8). Only
/// a reed softer than the softest unloaded one that speaks across the range
/// is tried; every other carries none. `None` if no load up to fifty times
/// the stiffness makes it speak.
pub fn load_reed(
    parameters: &rf_musette_dsp::Parameters,
    bare: ReedDesign,
    aim: f64,
) -> Option<f64> {
    use rf_musette_dsp::compass::{load_for, yield_stiffening};
    let mut design = bare;
    design.frequency = aim;
    let start = yield_stiffening(parameters, &design);
    if start <= 1.0 {
        return Some(0.0);
    }
    let ceiling = parameters
        .get(rf_musette_dsp::parameters::BELLOWS_CEILING)
        .unwrap_or(1000.0);
    // From 50 Pa if it can; a loaded reed's threshold may stand above it,
    // and then from the least pressure it can (docs/ROADMAP.md, 8e).
    [50.0, 100.0, 150.0, 200.0, TUNING_PRESSURE]
        .into_iter()
        .find_map(|floor| {
            // The ceiling first: it is where a soft reed fails.
            let pressures = [ceiling, floor, TUNING_PRESSURE.max(floor)];
            let speaks = |stiffening: f64| {
                let mut loaded = design;
                loaded.tip_load = load_for(stiffening);
                holds(loaded, aim, &pressures)
            };
            // Raise the stiffening by a quarter from the yield rule's until
            // the reed speaks, then halve the bracket six times.
            let (mut low, mut high) = (1.0, start);
            while !speaks(high) {
                low = high;
                high *= 1.25;
                if high > 20.0 {
                    return None;
                }
            }
            for _ in 0..6 {
                let middle = (low * high).sqrt();
                if speaks(middle) {
                    high = middle;
                } else {
                    low = middle;
                }
            }
            Some(load_for(high))
        })
}

/// How a maker finishes one reed: its tip load, its inlet duct (a multiple
/// of the tone hole's depth), and the cents its mode sits above its aim.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Finish {
    pub load: f64,
    pub duct: f64,
    pub cents: f64,
}

/// Every reed's finish, as `rf-musette-lab tune` writes it: the treble's,
/// then the bass side's.
#[allow(clippy::type_complexity)]
pub fn voice(
    parameters: &rf_musette_dsp::Parameters,
) -> (
    [[Option<Finish>; rf_musette_dsp::compass::KEYS]; rf_musette_dsp::parameters::RANKS],
    [[Option<Finish>; rf_musette_dsp::compass::BASS_KEYS]; rf_musette_dsp::parameters::BASS_RANKS],
) {
    use rf_musette_dsp::compass::{
        BASS_KEYS, FIRST_KEY, KEYS, bare, bass_bare, bass_target, target,
    };
    // A finish as the tables keep it, f32 to three places: what the engine
    // builds, and so what the reed is tuned as.
    let kept = |value: f64| f64::from(format!("{value:.3}").parse::<f32>().unwrap_or(f32::NAN));
    let finish = |bare: ReedDesign, aim: f64| {
        let load = kept(load_reed(parameters, bare, aim)?);
        let loaded = ReedDesign {
            tip_load: load,
            ..bare
        };
        let duct = kept(duct_reed(parameters, loaded, aim)?);
        let cents = tune_design(
            ReedDesign {
                tone_hole_depth: loaded.tone_hole_depth * duct,
                ..loaded
            },
            aim,
        )?;
        Some(Finish { load, duct, cents })
    };
    let mut treble = [[None; KEYS]; rf_musette_dsp::parameters::RANKS];
    for (rank, row) in treble.iter_mut().enumerate() {
        for (index, cell) in row.iter_mut().enumerate() {
            let key = FIRST_KEY + index as u8;
            *cell = bare(parameters, key, rank)
                .and_then(|design| finish(design, target(parameters, key, rank)));
        }
    }
    let mut bass = [[None; BASS_KEYS]; rf_musette_dsp::parameters::BASS_RANKS];
    for (rank, row) in bass.iter_mut().enumerate() {
        for (pitch_class, cell) in row.iter_mut().enumerate() {
            *cell = bass_bare(parameters, pitch_class, rank)
                .and_then(|design| finish(design, bass_target(parameters, pitch_class, rank)?));
        }
    }
    (treble, bass)
}

/// The cents `design`'s mode must sit above `aim` for it to sound on `aim`
/// at its [`tuning_pressure`].
pub fn tune_design(mut design: ReedDesign, aim: f64) -> Option<f64> {
    design.frequency = aim;
    let pressure = tuning_pressure(design)?;
    let mut correction = 0.0;
    for _ in 0..2 {
        design.frequency = aim * 2f64.powf(correction / 1200.0);
        correction += cents(sounding(design, pressure)?.frequency, aim);
    }
    Some(correction)
}
