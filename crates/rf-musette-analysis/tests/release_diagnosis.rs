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
