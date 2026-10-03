//! Two reports of the player's (2026-10-01), measured before anything was
//! changed: the "chick" at an opening of the bellows (8j -- the wheel as the
//! bellows it was heard on has since been withdrawn) and the noise as a note
//! is let go (8k): the release's envelope and high band, the hole's flow as
//! the pallet seals, the sealed reed's ring-down. Diagnoses; they print and
//! assert nothing.

use rf_musette_dsp::Engine;
use rf_musette_dsp::parameters;

const RATE: f32 = 48_000.0;

fn high_pass_energy(x: &[f32], at: usize, length: usize) -> f64 {
    // A first difference twice: a crude high-pass, steep enough to show a
    // click against a tone whose energy sits below 2 kHz.
    let w = &x[at.saturating_sub(2)..(at + length).min(x.len())];
    let mut sum = 0.0;
    for k in 2..w.len() {
        let d = f64::from(w[k]) - 2.0 * f64::from(w[k - 1]) + f64::from(w[k - 2]);
        sum += d * d;
    }
    sum / length as f64
}

/// The user's second report (2026-10-01): a noise as a note is let go --
/// heard on real accordions, sometimes, but here always and strongly.
/// A note held under a steady push (expression), dry, then released: the
/// envelope and the band above 3 kHz (a double difference, as above), 1 ms
/// at a time, from 5 ms before the release to 40 ms after, against the
/// held tone's.
#[test]
#[ignore = "diagnosis: prints the release of a note"]
fn the_release_of_a_note() {
    for &(name, note) in &[("F4", 65u8), ("C5", 72), ("A3", 57)] {
        let mut engine = Engine::new(RATE).unwrap();
        // The bellows played: Key Touch off, or it rests (9h again).
        assert!(engine.set_parameter(rf_musette_dsp::parameters::KEY_TOUCH, 0.0));
        assert!(engine.set_parameter(parameters::MIC_LAYOUT, 6.0));
        engine.bellows_mut().expression_wide(0.6);
        engine.note_on(note, 1.0);
        let mut block = [0.0f32; 48];
        let mut out = Vec::new();
        for _ in 0..1000 {
            engine.render(&mut block);
            out.extend_from_slice(&block);
        }
        let release = out.len();
        engine.note_off(note);
        for _ in 0..100 {
            engine.render(&mut block);
            out.extend_from_slice(&block);
        }
        let ms = (RATE / 1000.0) as usize;
        let held_peak = out[release - 100 * ms..release]
            .iter()
            .fold(0.0f32, |m, x| m.max(x.abs()));
        let held_high = high_pass_energy(&out, release - 100 * ms, 100 * ms).max(1e-30);
        println!("{name}: held peak {held_peak:.3}");
        // The tail: the level every 25 ms after, to 300 ms.
        let mut tail = Vec::new();
        for _ in 0..300 {
            engine.render(&mut block);
            tail.extend_from_slice(&block);
        }
        let after = &out[release..];
        let whole: Vec<f32> = after.iter().chain(tail.iter()).copied().collect();
        let marks: Vec<String> = (1..=12)
            .map(|k| {
                let at = k * 25 * ms;
                let peak = whole[at..at + 5 * ms]
                    .iter()
                    .fold(0.0f32, |m, x| m.max(x.abs()));
                format!(
                    "{} ms {:+.0}",
                    k * 25,
                    20.0 * (peak / held_peak).max(1e-6).log10()
                )
            })
            .collect();
        println!("  tail (dB of the held peak): {}", marks.join(", "));
        for t in -5i32..40 {
            let at = (release as i32 + t * ms as i32) as usize;
            let peak = out[at..at + ms].iter().fold(0.0f32, |m, x| m.max(x.abs()));
            let high = high_pass_energy(&out, at, ms);
            println!(
                "  {t:+3} ms  level {:+6.1} dB  high band {:+6.1} dB",
                20.0 * (peak / held_peak).max(1e-6).log10(),
                10.0 * (high / held_high).max(1e-12).log10()
            );
        }
    }
}

/// Where in the pallet's travel its curtain starts to narrow the hole, and
/// what the reed does in its cell once the pallet has shut: its
/// displacement's envelope after the release, against the held tone's.
#[test]
#[ignore = "diagnosis: prints the curtain and the sealed reed's ring-down"]
fn the_curtain_and_the_sealed_reed() {
    use rf_musette_dsp::pallet::rim;
    let p = parameters::Parameters::default();
    let design = p.reed_design();
    let pallet = p.pallet_design();
    let full = rim(design.tone_hole_area) * pallet.lift;
    println!(
        "hole {:.1} mm², curtain at full lift {:.1} mm²: the curtain narrows the hole below {:.0} % of the travel",
        design.tone_hole_area * 1e6,
        full * 1e6,
        100.0 * design.tone_hole_area / full
    );
    let mut engine = Engine::new(RATE).unwrap();
    assert!(engine.set_parameter(rf_musette_dsp::parameters::KEY_TOUCH, 0.0));
    assert!(engine.set_parameter(parameters::MIC_LAYOUT, 6.0));
    engine.bellows_mut().expression_wide(0.6);
    engine.note_on(65, 1.0);
    let mut block = [0.0f32; 48];
    for _ in 0..1000 {
        engine.render(&mut block);
    }
    let pull = rf_musette_dsp::PULL_REED;
    // The sounding rank: the one whose tongue swings most.
    let rank = (0..parameters::RANKS)
        .max_by(|a, b| {
            let swing = |r: usize| {
                engine
                    .reed(65, r, pull)
                    .map_or(0.0, |(_, s)| s.velocity.abs())
            };
            swing(*a).total_cmp(&swing(*b))
        })
        .unwrap();
    let held = (0..480)
        .map(|_| {
            let mut one = [0.0f32; 1];
            engine.render(&mut one);
            engine
                .reed(65, rank, pull)
                .map_or(0.0, |(_, s)| s.zeta.abs())
        })
        .fold(0.0f64, f64::max);
    println!("rank {rank}, held swing {:.3} mm", held * 1e3);
    engine.note_off(65);
    for window in 0..30 {
        let mut most = 0.0f64;
        for _ in 0..(RATE as usize / 100) {
            let mut one = [0.0f32; 1];
            engine.render(&mut one);
            if let Some((_, s)) = engine.reed(65, rank, pull) {
                most = most.max(s.zeta.abs());
            }
        }
        println!(
            "  {:3} ms after release: the tongue's swing {:+6.1} dB of the held swing",
            window * 10,
            20.0 * (most / held).max(1e-9).log10()
        );
    }
}

/// What the hole passes in the substeps before its pallet seals, against
/// the held tone's flow: whatever is left is cut to nothing in one step.
#[test]
#[ignore = "diagnosis: prints the hole's flow as the pallet seals"]
fn the_flow_as_the_pallet_seals() {
    for &note in &[65u8, 57] {
        let mut engine = Engine::new(RATE).unwrap();
        assert!(engine.set_parameter(rf_musette_dsp::parameters::KEY_TOUCH, 0.0));
        assert!(engine.set_parameter(parameters::MIC_LAYOUT, 6.0));
        assert!(engine.set_parameter(parameters::OVERSAMPLING, 1.0));
        engine.bellows_mut().expression_wide(0.6);
        engine.note_on(note, 1.0);
        let mut one = [0.0f32; 1];
        for _ in 0..48_000 {
            engine.render(&mut one);
        }
        let pull = rf_musette_dsp::PULL_REED;
        let rank = 2;
        let mut held = 0.0f64;
        for _ in 0..4800 {
            engine.render(&mut one);
            held = held.max(engine.reed(note, rank, pull).unwrap().1.hole_flow.abs());
        }
        engine.note_off(note);
        let mut trace = Vec::new();
        for n in 0..960 {
            engine.render(&mut one);
            let s = engine.reed(note, rank, pull).unwrap().1;
            trace.push((n, s.hole_flow, s.pallet, one[0]));
        }
        let sealed = trace.iter().position(|t| t.2 == 0.0).unwrap();
        println!(
            "key {note}: held hole flow peak {:.3e} m³/s, sealed at sample {sealed}",
            held
        );
        for t in &trace[sealed.saturating_sub(6)..sealed + 3] {
            println!(
                "  {:4}  hole flow {:+.3e} ({:+6.1} dB of held)  curtain {:.3e} m²  out {:+.4}",
                t.0,
                t.1,
                20.0 * (t.1.abs() / held).max(1e-9).log10(),
                t.2,
                t.3
            );
        }
    }
}

/// The F4's middle reed let go at the player's 624 Pa, step by step around
/// the pallet's seating: curtain, hole flow, radiated rate, slot pressure.
#[test]
#[ignore = "diagnosis: prints the last steps of a release under pressure"]
fn the_seating_step_by_step() {
    use rf_musette_dsp::compass::design;
    use rf_musette_dsp::pallet::Pallet;
    use rf_musette_dsp::parameters::{PALLET_OPENING, RANK_MIDDLE};
    use rf_musette_dsp::reed::{self, ReedModel, ReedState, Tube};
    let mut p = rf_musette_dsp::Parameters::default();
    assert!(p.set(PALLET_OPENING, 20.0));
    let pallet = p.pallet_design();
    let reed = design(&p, 65, RANK_MIDDLE).unwrap();
    let model = ReedModel::new(reed);
    let rate = 96_000.0;
    let h = 1.0 / rate;
    let mut state = ReedState::default();
    let mut tube = Tube::default();
    let mut valve = Pallet::default();
    let release = (1.0 * rate) as usize;
    let mut seated = None;
    let mut rows = Vec::new();
    for n in 0..(1.1 * rate) as usize {
        valve.press(if n < release { 1.0 } else { 0.0 });
        valve.advance(&pallet, h);
        let area = valve.area(&pallet, reed.tone_hole_area);
        let out = reed::step(&model, &mut state, &mut tube, 624.0, area, h);
        if n >= release {
            if area == 0.0 && seated.is_none() {
                seated = Some(n);
            }
            rows.push((n, area, state.hole_flow, out, state.cell_pressure));
        }
    }
    let seated = seated.unwrap();
    let peak_before = rows
        .iter()
        .filter(|r| r.0 + 960 < release + 960 && r.0 < release)
        .count();
    let _ = peak_before;
    let steady = rows[..96].iter().map(|r| r.3.abs()).fold(0.0, f64::max);
    println!(
        "released at step {release}, seated at {seated} ({:.2} ms); |rate| in the first ms {steady:.3e}",
        (seated - release) as f64 / rate * 1e3
    );
    for r in rows
        .iter()
        .filter(|r| r.0 + 6 >= seated && r.0 <= seated + 6)
    {
        println!(
            "step {:+3}: curtain {:.3e} m², hole flow {:+.3e} m³/s, rate {:+.3e}, slot pressure {:+.1} Pa",
            r.0 as i64 - seated as i64,
            r.1,
            r.2,
            r.3,
            r.4
        );
    }
}

/// The player (2026-10-02, 8n): the noise at a release is in 1-4 kHz, in
/// the instrument alone. The cell's lowest resonance as the pallet comes
/// down: tan kL = Z₀/(ωM), M the hole's mass and the curtain's, ρw/A,
/// which grows without bound as the curtain shuts -- from a quarter wave
/// open towards a half wave closed. With the curtain at each area, and when
/// in a release (Pallet Closing 10 ms, Pad Seating 12 ms) it is there.
#[test]
#[ignore = "diagnosis: prints the cell's resonance against the curtain"]
fn the_cell_as_the_pallet_closes() {
    use rf_musette_dsp::compass::design;
    use rf_musette_dsp::pallet::{Pallet, rim};
    use rf_musette_dsp::parameters::{Parameters, RANK_MIDDLE};
    use rf_musette_dsp::reed::{AIR_DENSITY, CELL_LENGTH_RATIO, ReedModel, SPEED_OF_SOUND};
    let p = Parameters::default();
    let pallet = p.pallet_design();
    for (name, key) in [("D4", 62u8), ("F4", 65), ("A5", 81)] {
        let reed = design(&p, key, RANK_MIDDLE).unwrap();
        let model = ReedModel::new(reed);
        let length = CELL_LENGTH_RATIO * reed.length;
        let z0 = AIR_DENSITY * SPEED_OF_SOUND / (reed.cell_volume / length);
        let resonance = |mass: f64| {
            let mut previous = f64::NAN;
            for step in 1..400_000 {
                let f = step as f64 * 0.1;
                let w = 2.0 * std::f64::consts::PI * f;
                let g = (w / SPEED_OF_SOUND * length).tan() - z0 / (w * mass);
                if previous < 0.0 && g >= 0.0 {
                    return f;
                }
                previous = g;
            }
            f64::NAN
        };
        // The release, step by step: when the curtain passes each area.
        let mut valve = Pallet {
            position: 1.0,
            ..Default::default()
        };
        let h = 1.0e-5;
        let mut trace = Vec::new();
        for n in 0..10_000 {
            valve.advance(&pallet, h);
            trace.push((n as f64 * h, valve.area(&pallet, reed.tone_hole_area)));
        }
        println!(
            "{name}: reed {:.0} Hz, cell {:.1} mm long; half wave {:.0} Hz",
            reed.frequency,
            length * 1e3,
            SPEED_OF_SOUND / (2.0 * length)
        );
        let full = rim(reed.tone_hole_area) * pallet.lift;
        for share in [1.0, 0.5, 0.2, 0.1, 0.05, 0.02, 0.01, 0.005, 0.002, 0.001] {
            let area = (full * share).min(reed.tone_hole_area);
            let mass = model.hole_inertance + AIR_DENSITY * 2.0e-3 / area;
            let at = trace
                .iter()
                .find(|(_, a)| *a <= area)
                .map_or(f64::NAN, |(t, _)| t * 1e3);
            println!(
                "  curtain {:7.2} mm² (at {at:5.1} ms): resonance {:6.0} Hz",
                area * 1e6,
                resonance(mass)
            );
        }
    }
}

/// The player's puff at a release (8n), one reed alone -- no tremolo to beat
/// against short windows -- at the wheel's 79 % (601 Pa), cycle by cycle:
/// each cycle of the tongue (maximum to maximum) and the first ten
/// harmonics of the radiated rate over it, each against its own held level.
/// Which harmonics outlast the others as the curtain shuts, and where the
/// cell then resonates.
#[test]
#[ignore = "diagnosis: prints a release cycle by cycle"]
fn the_release_cycle_by_cycle() {
    use rf_musette_dsp::compass::design;
    use rf_musette_dsp::pallet::Pallet;
    use rf_musette_dsp::parameters::{PALLET_OPENING, Parameters, RANK_MIDDLE};
    use rf_musette_dsp::reed::{self, ReedModel, ReedState, Tube};
    const HARMONICS: usize = 10;
    let mut p = Parameters::default();
    assert!(p.set(PALLET_OPENING, 35.0));
    let pallet = p.pallet_design();
    for (name, key) in [("D4", 62u8), ("F4", 65), ("A5", 81)] {
        let design = design(&p, key, RANK_MIDDLE).unwrap();
        let model = ReedModel::new(design);
        let rate = 96_000.0;
        let h = 1.0 / rate;
        let mut state = ReedState::default();
        let mut tube = Tube::default();
        let mut valve = Pallet::default();
        let release = (1.5 * rate) as usize;
        let (mut out, mut zeta, mut area) = (Vec::new(), Vec::new(), Vec::new());
        for n in 0..(1.6 * rate) as usize {
            valve.press(if n < release { 1.0 } else { 0.0 });
            valve.advance(&pallet, h);
            let a = valve.area(&pallet, design.tone_hole_area);
            out.push(reed::step(&model, &mut state, &mut tube, 601.0, a, h));
            zeta.push(state.zeta);
            area.push(a);
        }
        // Cycles: the tongue's maxima.
        let maxima: Vec<usize> = (1..zeta.len() - 1)
            .filter(|&i| zeta[i] > zeta[i - 1] && zeta[i] >= zeta[i + 1])
            .filter(|&i| i > release - (0.1 * rate) as usize)
            .collect();
        let cycles: Vec<(usize, usize)> = maxima.windows(2).map(|w| (w[0], w[1])).collect();
        let analyse = |(from, to): (usize, usize)| {
            let n = (to - from) as f64;
            let harmonics: Vec<f64> = (1..=HARMONICS)
                .map(|k| {
                    let (mut re, mut im) = (0.0, 0.0);
                    for (j, x) in out[from..to].iter().enumerate() {
                        let phase = 2.0 * std::f64::consts::PI * k as f64 * j as f64 / n;
                        re += x * phase.cos();
                        im -= x * phase.sin();
                    }
                    (re * re + im * im).sqrt() / n
                })
                .collect();
            let swing = zeta[from..to].iter().copied().fold(f64::MIN, f64::max)
                - zeta[from..to].iter().copied().fold(f64::MAX, f64::min);
            (harmonics, swing, rate / n)
        };
        let held: Vec<_> = cycles
            .iter()
            .filter(|c| c.1 < release)
            .map(|c| analyse(*c))
            .collect();
        let reference: Vec<f64> = (0..HARMONICS)
            .map(|k| held.iter().map(|c| c.0[k]).sum::<f64>() / held.len() as f64)
            .collect();
        let held_swing = held.iter().map(|c| c.1).sum::<f64>() / held.len() as f64;
        println!(
            "{name}: reed {:.0} Hz; the held rate's harmonics, dB re h1: {}",
            design.frequency,
            reference
                .iter()
                .map(|h| format!("{:+.0}", 20.0 * (h / reference[0]).log10()))
                .collect::<Vec<_>>()
                .join(" ")
        );
        println!("   t ms  curtain mm²  f Hz  swing dB | h1..h10 dB re each one's held level");
        for c in cycles
            .iter()
            .filter(|c| c.0 + (0.004 * rate) as usize >= release)
        {
            let (harmonics, swing, f) = analyse(*c);
            if 20.0 * (swing / held_swing).log10() < -40.0 {
                break;
            }
            println!(
                "  {:+5.1} {:9.2} {:7.0} {:+6.1} | {}",
                (c.0 as f64 - release as f64) / rate * 1e3,
                area[c.0] * 1e6,
                f,
                20.0 * (swing / held_swing).log10(),
                harmonics
                    .iter()
                    .zip(&reference)
                    .map(|(h, r)| format!("{:+4.0}", 20.0 * (h / r).max(1e-9).log10()))
                    .collect::<Vec<_>>()
                    .join("")
            );
        }
    }
}

/// The same reeds with the curtain held still at each area, the steady
/// tone's harmonics against the open pallet's: whether the release's burst
/// near 2.5 kHz (8n) is the cell's filter at that curtain, there with the
/// pallet still, or the curtain's motion.
#[test]
#[ignore = "diagnosis: prints the steady tone at fixed curtains"]
fn the_tone_at_fixed_curtains() {
    use rf_musette_dsp::compass::design;
    use rf_musette_dsp::parameters::{Parameters, RANK_MIDDLE};
    use rf_musette_dsp::reed::{self, ReedModel, ReedState, Tube};
    const HARMONICS: usize = 10;
    let p = Parameters::default();
    for (name, key) in [("D4", 62u8), ("F4", 65)] {
        let design = design(&p, key, RANK_MIDDLE).unwrap();
        let model = ReedModel::new(design);
        let rate = 96_000.0;
        let h = 1.0 / rate;
        let spectrum = |area: f64| -> Option<Vec<f64>> {
            let mut state = ReedState::default();
            let mut tube = Tube::default();
            let (mut out, mut zeta) = (Vec::new(), Vec::new());
            for n in 0..(1.2 * rate) as usize {
                // Opened full first, so every curtain starts from a tone.
                let a = if n < (0.4 * rate) as usize {
                    design.tone_hole_area
                } else {
                    area
                };
                out.push(reed::step(&model, &mut state, &mut tube, 601.0, a, h));
                zeta.push(state.zeta);
            }
            let from = (0.9 * rate) as usize;
            let maxima: Vec<usize> = (from + 1..zeta.len() - 1)
                .filter(|&i| zeta[i] > zeta[i - 1] && zeta[i] >= zeta[i + 1])
                .collect();
            if maxima.len() < 10 {
                return None;
            }
            let (a, b) = (maxima[0], maxima[maxima.len() - 1]);
            let cycles = (maxima.len() - 1) as f64;
            let n = (b - a) as f64;
            Some(
                (1..=HARMONICS)
                    .map(|k| {
                        let (mut re, mut im) = (0.0, 0.0);
                        for (j, x) in out[a..b].iter().enumerate() {
                            let phase =
                                2.0 * std::f64::consts::PI * k as f64 * cycles * j as f64 / n;
                            re += x * phase.cos();
                            im -= x * phase.sin();
                        }
                        (re * re + im * im).sqrt() / n
                    })
                    .collect(),
            )
        };
        let open = spectrum(design.tone_hole_area).unwrap();
        println!(
            "{name} ({:.0} Hz): steady harmonics, dB re the open pallet's",
            design.frequency
        );
        for area in [60.0, 40.0, 30.0, 20.0, 15.0, 12.0, 10.0, 8.0, 6.0, 4.0] {
            match spectrum(area * 1e-6) {
                Some(s) => println!(
                    "  curtain {area:5.1} mm² | {}",
                    s.iter()
                        .zip(&open)
                        .map(|(x, o)| format!("{:+4.0}", 20.0 * (x / o).max(1e-9).log10()))
                        .collect::<Vec<_>>()
                        .join("")
                ),
                None => println!("  curtain {area:5.1} mm² | silent"),
            }
        }
    }
}

/// The release's burst near 2.5 kHz (8n) against the pallet's closing time:
/// the most any of h5-h10 rises above its held level, cycle by cycle, while
/// the tongue still swings within 20 dB of its held swing.
#[test]
#[ignore = "diagnosis: prints the burst against the closing time"]
fn the_burst_against_the_closing_time() {
    use rf_musette_dsp::compass::design;
    use rf_musette_dsp::pallet::Pallet;
    use rf_musette_dsp::parameters::{PALLET_CLOSING, PALLET_OPENING, Parameters, RANK_MIDDLE};
    use rf_musette_dsp::reed::{self, ReedModel, ReedState, Tube};
    let sweeps: Vec<(&str, usize, Vec<f64>)> = match std::env::var("BURST_SWEEP").as_deref() {
        Ok("lift") => vec![(
            "lift mm",
            rf_musette_dsp::parameters::PALLET_LIFT,
            vec![1.5, 2.0, 3.0, 4.0, 6.0],
        )],
        _ => vec![("closing ms", PALLET_CLOSING, vec![3.0, 10.0, 30.0, 100.0])],
    };
    let (label, index, values) = &sweeps[0];
    for (name, key) in [("D4", 62u8), ("F4", 65), ("A5", 81)] {
        for &closing in values {
            let mut p = Parameters::default();
            assert!(p.set(PALLET_OPENING, 35.0));
            assert!(p.set(*index, closing));
            let pallet = p.pallet_design();
            let design = design(&p, key, RANK_MIDDLE).unwrap();
            let model = ReedModel::new(design);
            let rate = 96_000.0;
            let h = 1.0 / rate;
            let mut state = ReedState::default();
            let mut tube = Tube::default();
            let mut valve = Pallet::default();
            let release = (1.5 * rate) as usize;
            let (mut out, mut zeta, mut area) = (Vec::new(), Vec::new(), Vec::new());
            for n in 0..(1.8 * rate) as usize {
                valve.press(if n < release { 1.0 } else { 0.0 });
                valve.advance(&pallet, h);
                let a = valve.area(&pallet, design.tone_hole_area);
                out.push(reed::step(&model, &mut state, &mut tube, 601.0, a, h));
                zeta.push(state.zeta);
                area.push(a);
            }
            let maxima: Vec<usize> = (1..zeta.len() - 1)
                .filter(|&i| zeta[i] > zeta[i - 1] && zeta[i] >= zeta[i + 1])
                .filter(|&i| i > release - (0.1 * rate) as usize)
                .collect();
            let analyse = |from: usize, to: usize| -> (Vec<f64>, f64) {
                let n = (to - from) as f64;
                let hs = (1..=10)
                    .map(|k| {
                        let (mut re, mut im) = (0.0, 0.0);
                        for (j, x) in out[from..to].iter().enumerate() {
                            let ph = 2.0 * std::f64::consts::PI * k as f64 * j as f64 / n;
                            re += x * ph.cos();
                            im -= x * ph.sin();
                        }
                        (re * re + im * im).sqrt() / n
                    })
                    .collect();
                let swing = zeta[from..to].iter().copied().fold(f64::MIN, f64::max)
                    - zeta[from..to].iter().copied().fold(f64::MAX, f64::min);
                (hs, swing)
            };
            let held: Vec<_> = maxima
                .windows(2)
                .filter(|w| w[1] < release)
                .map(|w| analyse(w[0], w[1]))
                .collect();
            let reference: Vec<f64> = (0..10)
                .map(|k| held.iter().map(|c| c.0[k]).sum::<f64>() / held.len() as f64)
                .collect();
            let swing0 = held.iter().map(|c| c.1).sum::<f64>() / held.len() as f64;
            let mut worst = (f64::MIN, 0usize, 0.0, 0.0, 0.0);
            for w in maxima.windows(2).filter(|w| w[0] >= release) {
                let (hs, swing) = analyse(w[0], w[1]);
                if 20.0 * (swing / swing0).log10() < -20.0 {
                    break;
                }
                for k in 4..10 {
                    let rise = 20.0 * (hs[k] / reference[k]).log10();
                    if rise > worst.0 {
                        worst = (
                            rise,
                            k + 1,
                            (w[0] - release) as f64 / rate * 1e3,
                            area[w[0]] * 1e6,
                            20.0 * (hs[0] / reference[0]).log10(),
                        );
                    }
                }
            }
            println!(
                "{name} {label} {closing:5.1}: h{} rises {:+.1} dB at {:.1} ms (curtain {:.1} mm²), h1 then {:+.1} dB",
                worst.1, worst.0, worst.2, worst.3, worst.4
            );
        }
    }
}

/// What the reed's own flow does in the release's bright cycles (8n): per
/// cycle, the share of it the slot passes air (above a tenth of its peak),
/// its crest factor, its 1st and 8th harmonics and the hole flow's 8th,
/// the drop left across the reed (bellows less the cell's mean), and the
/// tongue's swing -- held, through the release, and with the curtain held
/// still at the burst's area for comparison.
#[test]
#[ignore = "diagnosis: prints the reed's flow through the release"]
fn the_reed_flow_in_the_burst() {
    use rf_musette_dsp::compass::design;
    use rf_musette_dsp::pallet::Pallet;
    use rf_musette_dsp::parameters::{PALLET_OPENING, Parameters, RANK_MIDDLE};
    use rf_musette_dsp::reed::{self, ReedModel, ReedState, Tube};
    let mut p = Parameters::default();
    assert!(p.set(PALLET_OPENING, 35.0));
    let pallet = p.pallet_design();
    let design = design(&p, 62, RANK_MIDDLE).unwrap();
    let model = ReedModel::new(design);
    let rate = 96_000.0;
    let h = 1.0 / rate;
    let supply = 601.0;
    struct Run {
        u: Vec<f64>,
        hole: Vec<f64>,
        cell: Vec<f64>,
        zeta: Vec<f64>,
        area: Vec<f64>,
    }
    let run = |curtain: Option<f64>| {
        let mut state = ReedState::default();
        let mut tube = Tube::default();
        let mut valve = Pallet::default();
        let release = (1.5 * rate) as usize;
        let mut r = Run {
            u: vec![],
            hole: vec![],
            cell: vec![],
            zeta: vec![],
            area: vec![],
        };
        for n in 0..(1.6 * rate) as usize {
            let a = match curtain {
                Some(still) if n >= (0.5 * rate) as usize => still,
                Some(_) => design.tone_hole_area,
                None => {
                    valve.press(if n < release { 1.0 } else { 0.0 });
                    valve.advance(&pallet, h);
                    valve.area(&pallet, design.tone_hole_area)
                }
            };
            reed::step(&model, &mut state, &mut tube, supply, a, h);
            r.u.push(state.flow);
            r.hole.push(state.hole_flow);
            r.cell.push(state.cell_pressure);
            r.zeta.push(state.zeta);
            r.area.push(a);
        }
        r
    };
    let harmonic = |x: &[f64], k: f64| {
        let n = x.len() as f64;
        let (mut re, mut im) = (0.0, 0.0);
        for (j, v) in x.iter().enumerate() {
            let ph = 2.0 * std::f64::consts::PI * k * j as f64 / n;
            re += v * ph.cos();
            im -= v * ph.sin();
        }
        (re * re + im * im).sqrt() / n
    };
    let report = |label: &str, r: &Run, from: usize, to: usize, reference: Option<&[f64; 3]>| {
        let maxima: Vec<usize> = (from.max(1)..to.min(r.zeta.len() - 1))
            .filter(|&i| r.zeta[i] > r.zeta[i - 1] && r.zeta[i] >= r.zeta[i + 1])
            .collect();
        let mut held = [0.0f64; 3];
        for w in maxima.windows(2) {
            let (a, b) = (w[0], w[1]);
            let u = &r.u[a..b];
            let peak = u.iter().copied().fold(f64::MIN, f64::max);
            let duty = u.iter().filter(|v| **v > 0.1 * peak).count() as f64 / u.len() as f64;
            let reversed = u.iter().filter(|v| **v < 0.0).count() as f64 / u.len() as f64;
            let rms = (u.iter().map(|v| v * v).sum::<f64>() / u.len() as f64).sqrt();
            let u1 = harmonic(u, 1.0);
            let u8 = harmonic(u, 8.0);
            let a8 = harmonic(&r.hole[a..b], 8.0);
            let drop = supply - r.cell[a..b].iter().sum::<f64>() / (b - a) as f64;
            let swing = r.zeta[a..b].iter().copied().fold(f64::MIN, f64::max)
                - r.zeta[a..b].iter().copied().fold(f64::MAX, f64::min);
            held = [u1, u8, a8];
            let db = |x: f64, k: usize| {
                reference.map_or(String::from("   ref"), |r| {
                    format!("{:+6.1}", 20.0 * (x / r[k]).log10())
                })
            };
            println!(
                "  {label} {:+6.1} ms curtain {:6.1} mm² | slot open {:3.0} % reversed {:3.0} % crest {:4.1} | u h1 {} h8 {} | hole h8 {} | drop {:5.0} Pa | swing {:.2} mm",
                (a as f64 - 1.5 * rate) / rate * 1e3,
                r.area[a] * 1e6,
                100.0 * duty,
                100.0 * reversed,
                peak / rms,
                db(u1, 0),
                db(u8, 1),
                db(a8, 2),
                drop,
                swing * 1e3
            );
        }
        held
    };
    let release = (1.5 * rate) as usize;
    let moving = run(None);
    println!("D4, one reed, 601 Pa: the last held cycle is the reference");
    let reference = report("held ", &moving, release - 700, release, None);
    report(
        "cut  ",
        &moving,
        release,
        release + (0.02 * rate) as usize,
        Some(&reference),
    );
    for still in [30.0e-6, 15.0e-6] {
        let r = run(Some(still));
        let end = r.zeta.len();
        report("still", &r, end - 700, end, Some(&reference));
    }
}
