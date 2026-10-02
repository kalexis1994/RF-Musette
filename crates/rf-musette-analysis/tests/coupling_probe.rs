//! Milestone 8m's coupling: what moves the F4's swing, its air and its
//! start. Diagnosis; prints.

use rf_musette_dsp::Parameters;
use rf_musette_dsp::compass::design;
use rf_musette_dsp::parameters::RANK_MIDDLE;
use rf_musette_dsp::reed::{self, ReedDesign, ReedModel, ReedState, Tube};

const RATE: f64 = 192_000.0;

/// Swing (mm), mean hole flow (cm³/s) at `pressure` from rest.
fn held(model: &ReedModel, pressure: f64) -> (f64, f64) {
    let h = 1.0 / RATE;
    let mut state = ReedState::default();
    let mut tube = Tube::default();
    let (mut low, mut high, mut flow, mut count) = (f64::MAX, f64::MIN, 0.0, 0.0);
    for n in 0..(2.0 * RATE) as usize {
        reed::step(model, &mut state, &mut tube, pressure, f64::INFINITY, h);
        if n as f64 > 1.5 * RATE {
            low = low.min(state.zeta);
            high = high.max(state.zeta);
            flow += state.hole_flow;
            count += 1.0;
        }
    }
    (0.5 * (high - low) * 1e3, flow / count * 1e6)
}

/// Growth rate, 1/s, of a micrometre's disturbance at `pressure`.
fn growth(model: &ReedModel, pressure: f64) -> f64 {
    let h = 1.0 / RATE;
    let mut state = ReedState::equilibrium(model, pressure);
    let mut tube = Tube::steady(pressure);
    let rest = state.zeta;
    state.zeta += 1e-6;
    let period = (RATE / model.design.frequency) as usize;
    let mut envelope = |periods: usize| {
        let mut peak = 0.0f64;
        for _ in 0..periods * period {
            reed::step(model, &mut state, &mut tube, pressure, f64::INFINITY, h);
            peak = peak.max((state.zeta - rest).abs());
        }
        peak
    };
    envelope(10);
    let first = envelope(5);
    envelope(30);
    let second = envelope(5);
    (second / first).ln() / (35.0 * period as f64 * h)
}

#[test]
#[ignore = "diagnosis: prints the F4 against its cell"]
fn what_moves_the_f4() {
    let p = Parameters::default();
    let f4 = design(&p, 65, RANK_MIDDLE).unwrap();
    println!(
        "hole mm², tube ×, volume ×: swing mm, flow cm³/s at 300 Pa; growth 1/s at 100, 200, 400 Pa"
    );
    for (hole, length, volume) in [
        (100.0, 1.0, 1.0),
        (150.0, 1.0, 1.0),
        (150.0, 0.5, 1.0),
        (150.0, 0.25, 1.0),
        (100.0, 0.5, 1.0),
        (100.0, 0.25, 1.0),
        (150.0, 1.0, 2.0),
        (150.0, 0.5, 2.0),
    ] {
        let mut model = ReedModel::new(ReedDesign {
            tone_hole_area: hole * 1e-6,
            cell_volume: f4.cell_volume * volume,
            ..f4
        });
        model.tube_seconds *= length;
        let (swing, flow) = held(&model, 300.0);
        let rates: Vec<String> = [100.0, 200.0, 400.0]
            .iter()
            .map(|pressure| format!("{:.1}", growth(&model, *pressure)))
            .collect();
        println!(
            "{hole:>5} {length:>5} {volume:>4}: {swing:.2} mm, {flow:.1} cm³/s; {}",
            rates.join(" ")
        );
    }
}

#[test]
#[ignore = "diagnosis: prints the F4 tip to the closed end and to the opening"]
fn the_f4_turned_round() {
    let p = Parameters::default();
    let f4 = design(&p, 65, RANK_MIDDLE).unwrap();
    for hole in [100.0, 150.0] {
        let model = ReedModel::new(ReedDesign {
            tone_hole_area: hole * 1e-6,
            ..f4
        });
        for (name, model) in [
            ("tip closed", model.clone()),
            ("tip opening", model.turned_round()),
        ] {
            let (swing, flow) = held(&model, 300.0);
            let rates: Vec<String> = [100.0, 200.0, 400.0]
                .iter()
                .map(|pressure| format!("{:.1}", growth(&model, *pressure)))
                .collect();
            println!(
                "hole {hole}, {name}: {swing:.2} mm, {flow:.1} cm³/s; {}",
                rates.join(" ")
            );
        }
    }
}

/// The F4's finger attack, Clarinet, against the voiced start.
#[test]
#[ignore = "diagnosis: prints the F4's attack against the Attack Kick"]
fn the_attack_against_the_kick() {
    use rf_musette_analysis::{attack_time, component_envelope};
    use rf_musette_dsp::compass::target;
    use rf_musette_dsp::parameters::{self, STIFF};
    use rf_musette_dsp::{Engine, REED_KEY};
    let rate = 48_000.0f32;
    let render = |engine: &mut Engine, seconds: f32| {
        let mut out = vec![0.0f32; (seconds * rate) as usize];
        for block in out.chunks_mut(256) {
            engine.render(block);
        }
        out.into_iter().map(f64::from).collect::<Vec<f64>>()
    };
    let p = Parameters::default();
    for kick in [1.0, 0.6, 0.5, 0.4, 0.3] {
        let attacks: Vec<String> = [100.0, 400.0]
            .iter()
            .map(|pressure: &f64| {
                let mut engine = Engine::new(rate).unwrap();
                assert!(engine.set_parameter(parameters::BELLOWS_RESPONSE, STIFF));
                assert!(engine.set_parameter(parameters::ATTACK_KICK, kick));
                engine
                    .bellows_mut()
                    .expression_wide((pressure / 1000.0).sqrt() as f32);
                render(&mut engine, 0.3);
                engine.note_on(REED_KEY, 1.0);
                let out = render(&mut engine, 2.0);
                let frequency = target(&p, REED_KEY, RANK_MIDDLE);
                attack_time(&component_envelope(
                    &out,
                    f64::from(rate),
                    frequency,
                    4.0,
                    0.001,
                ))
                .map_or("--".into(), |a| format!("{:.0} ms", a * 1e3))
            })
            .collect();
        println!("kick {kick}: {}", attacks.join(", "));
    }
}

/// The 16′ C2's finger attack at 400 Pa against the voiced start, as
/// milestone 8's prediction 8 measures it.
#[test]
#[ignore = "diagnosis: prints the 16′ C2's attack against the Attack Kick"]
fn the_bass_attack_against_the_kick() {
    use rf_musette_analysis::{attack_time, component_envelope};
    use rf_musette_dsp::Engine;
    use rf_musette_dsp::compass::bass_target;
    use rf_musette_dsp::parameters::{self, BASS_16, STIFF};
    let rate = 48_000.0f32;
    let render = |engine: &mut Engine, seconds: f32| {
        let mut out = vec![0.0f32; (seconds * rate) as usize];
        for block in out.chunks_mut(256) {
            engine.render(block);
        }
        out.into_iter().map(f64::from).collect::<Vec<f64>>()
    };
    let frequency = bass_target(&Parameters::default(), 0, BASS_16).unwrap();
    for kick in [1.0, 0.6, 0.5, 0.4, 0.3] {
        let mut engine = Engine::new(rate).unwrap();
        assert!(engine.set_parameter(parameters::BELLOWS_RESPONSE, STIFF));
        assert!(engine.set_parameter(parameters::BASS_REGISTER, 6.0));
        assert!(engine.set_parameter(parameters::ATTACK_KICK, kick));
        engine.bellows_mut().expression_wide(0.4f32.sqrt());
        render(&mut engine, 0.3);
        engine.bass_on(36, 1.0);
        let out = render(&mut engine, 2.0);
        let attack = attack_time(&component_envelope(
            &out,
            f64::from(rate),
            frequency,
            4.0,
            0.001,
        ));
        println!("kick {kick}: {:?} ms", attack.map(|a| (a * 1e3).round()));
    }
}
