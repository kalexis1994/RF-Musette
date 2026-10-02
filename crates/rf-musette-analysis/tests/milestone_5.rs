//! Milestone 5: one bellows for every reed -- the arm's push, the air the
//! reeds and the vents spend, and the key's pallet shared by its ranks.
//! Predictions as written in docs/ROADMAP.md before the code; each test
//! says which. Through the engine, as a player drives it.

use rf_musette_analysis::cents;
use rf_musette_dsp::parameters::{self, ARM, STIFF};
use rf_musette_dsp::{Engine, PARAMETER_SPECS, PULL_REED, REED_KEY};

const RATE: f32 = 48_000.0;
/// The intent that asks 300 Pa of the default bellows (1 kPa × intent²).
const INTENT_300_PA: f32 = 0.547_722_6;
const CLARINET: f64 = 11.0;
const MUSETTE: f64 = 8.0;
const MASTER: f64 = 6.0;

fn engine(response: f64, register: f64) -> Engine {
    let mut engine = Engine::new(RATE).unwrap();
    assert!(engine.set_parameter(parameters::BELLOWS_RESPONSE, response));
    assert!(engine.set_parameter(parameters::REGISTER, register));
    engine.bellows_mut().expression_wide(INTENT_300_PA);
    engine
}

fn render(engine: &mut Engine, seconds: f32) -> Vec<f32> {
    let mut out = vec![0.0; (seconds * RATE) as usize];
    for block in out.chunks_mut(256) {
        engine.render(block);
    }
    out
}

fn level(signal: &[f32]) -> f64 {
    let rms =
        (signal.iter().map(|x| f64::from(*x).powi(2)).sum::<f64>() / signal.len() as f64).sqrt();
    20.0 * rms.log10()
}

/// The steady pressure and level with the key held, the bellows as given.
/// The level over 20 s: Master's tremolo ranks beat at ~3.5 Hz, and over a
/// shorter window the sum's level hangs on the phase they start at.
fn steady(response: f64, register: f64) -> (f64, f64) {
    let mut engine = engine(response, register);
    engine.note_on(REED_KEY, 1.0);
    let out = render(&mut engine, 21.5);
    (engine.supply().abs(), level(&out[(1.5 * RATE) as usize..]))
}

/// Prediction 2: with the arm and M alone, the steady pressure falls ~3 %
/// from the 300 Pa the push asks.
#[test]
fn the_arm_gives_a_little_way_to_one_reed() {
    let (pressure, _) = steady(ARM, CLARINET);
    let fall = 1.0 - pressure / 300.0;
    println!(
        "Clarinet, arm: {pressure:.1} Pa, {:.1} % below the push",
        100.0 * fall
    );
    assert!((0.01..0.06).contains(&fall), "fall {:.1} %", 100.0 * fall);
}

/// Prediction 3: with Master the fall is ~15 %, so Master sits +4.5 to
/// +5.5 dB over Clarinet, against +5.94 with the bellows stiff -- the arm
/// costing it 0.44 to 1.44 dB. Over a window long enough for the beats
/// (2026-10-01) the stiff Master is +6.33 dB, not +5.94, so the band is
/// read as what the arm costs.
///
/// NOT MET since 8m: the fall is 4.8 %, the tube's reeds drawing far less
/// air (13.4 % with the cell a volume) (docs/ROADMAP.md, 8m).
#[test]
#[ignore = "known defect: the tube's reeds draw less air, a 4.8 % fall (docs/ROADMAP.md, 8m)"]
fn five_ranks_draw_the_pressure_down() {
    let (clarinet, clarinet_level) = steady(ARM, CLARINET);
    let (master, master_level) = steady(ARM, MASTER);
    let (_, stiff_clarinet) = steady(STIFF, CLARINET);
    let (_, stiff_master) = steady(STIFF, MASTER);
    let fall = 1.0 - master / 300.0;
    let over = master_level - clarinet_level;
    println!(
        "arm: Clarinet {clarinet:.1} Pa, Master {master:.1} Pa ({:.1} % below the push); Master over Clarinet {over:+.2} dB (stiff {:+.2})",
        100.0 * fall,
        stiff_master - stiff_clarinet
    );
    assert!((0.08..0.25).contains(&fall), "fall {:.1} %", 100.0 * fall);
    let cost = stiff_master - stiff_clarinet - over;
    assert!((0.44..1.44).contains(&cost), "the arm costs {cost:.2} dB");
}

/// The M reed's sounding frequency with the key held part-way down, the
/// bellows stiff, through the given register.
fn bent(register: f64, depth: f64) -> Option<f64> {
    let mut engine = engine(STIFF, register);
    engine.press(REED_KEY, depth);
    let mut zeta = Vec::new();
    let mut one = [0.0f32; 1];
    for n in 0..(3.0 * RATE) as usize {
        engine.render(&mut one);
        if n >= (2.0 * RATE) as usize {
            let (_, state) = engine
                .reed(REED_KEY, parameters::RANK_MIDDLE, PULL_REED)
                .unwrap();
            zeta.push(state.zeta);
        }
    }
    let mean = zeta.iter().sum::<f64>() / zeta.len() as f64;
    let crossings: Vec<f64> = (1..zeta.len())
        .filter(|&i| zeta[i - 1] < mean && zeta[i] >= mean)
        .map(|i| {
            let (a, b) = (zeta[i - 1] - mean, zeta[i] - mean);
            i as f64 - 1.0 + a / (a - b)
        })
        .collect();
    if crossings.len() < 10 {
        return None;
    }
    Some(
        (crossings.len() - 1) as f64 * f64::from(RATE)
            / (crossings[crossings.len() - 1] - crossings[0]),
    )
}

/// Prediction 4: three reeds share the throttle, so the key part-way down
/// bends Musette's M reed deeper than Clarinet's.
///
/// WITHDRAWN: its premise was wrong. Built as written -- each open rank
/// seeing the one hole's curtain over n -- it silenced Master, five reeds
/// drawing through a single 150 mm² hole. On an instrument each rank's cell
/// has its own hole under the key's pallet, and the curtain grows with the
/// holes it covers, so each rank sees about its own; how much they still
/// share depends on the pallet's geometry, which nothing read gives. With a
/// curtain per rank the two bend alike (−1.2 cents at depth 0.5).
#[test]
#[ignore = "withdrawn: the pallet's shared geometry is unmeasured (docs/ROADMAP.md, 5)"]
fn a_shared_pallet_bends_more_reeds_deeper() {
    let open = bent(CLARINET, 1.0).unwrap();
    for depth in [0.5, 0.4, 0.3] {
        let clarinet = bent(CLARINET, depth).map(|f| cents(open, f));
        let musette = bent(MUSETTE, depth).map(|f| cents(open, f));
        println!("depth {depth}: Clarinet {clarinet:+.1?} cents, Musette {musette:+.1?} cents");
        if let (Some(c), Some(m)) = (clarinet, musette) {
            assert!(
                m < c,
                "Musette {m:+.1} not below Clarinet {c:+.1} at {depth}"
            );
        }
    }
}

/// Prediction 5: the air button fully open with the push held: the
/// pressure falls by at least half and the level by at least 6 dB.
///
/// NOT MET with the assumed constants: 289 → 206 Pa (−29 %), −2.9 dB.
/// The arm, at v_max 1 m/s, keeps most of its push against the 5.4 L/s the
/// 400 mm² vent lets out. Both constants are assumed; neither is moved to
/// meet the prediction. An air button's size and an arm's speed on a
/// bellows are what would settle it.
#[test]
#[ignore = "not met with the assumed arm and air button (docs/ROADMAP.md, 5)"]
fn the_air_button_lets_the_pressure_go() {
    let mut engine = engine(ARM, CLARINET);
    engine.note_on(REED_KEY, 1.0);
    let before = render(&mut engine, 2.0);
    let (held, held_level) = (engine.supply().abs(), level(&before[RATE as usize..]));
    assert!(engine.set_parameter(parameters::AIR_VALVE, 1.0));
    let after = render(&mut engine, 2.0);
    let (vented, vented_level) = (engine.supply().abs(), level(&after[RATE as usize..]));
    println!("held {held:.1} Pa, {held_level:.1} dB; vented {vented:.1} Pa, {vented_level:.1} dB");
    assert!(vented < 0.5 * held, "pressure {held:.1} → {vented:.1} Pa");
    assert!(
        held_level - vented_level > 6.0,
        "level fell {:.1} dB",
        held_level - vented_level
    );
}

/// Prediction 6: no reachable parameter set makes the engine blow up or
/// produce a non-number, the bellows' constants and the air button
/// included.
#[test]
fn no_reachable_bellows_blows_up() {
    let mut seed = 0x9e37_79b9_7f4a_7c15u64;
    for trial in 0..24 {
        let mut engine = Engine::new(RATE).unwrap();
        // The bellows' parameters, every one after the register.
        for (index, spec) in PARAMETER_SPECS
            .iter()
            .enumerate()
            .skip(parameters::BELLOWS_RESPONSE)
        {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            let unit = (seed >> 11) as f64 / (1u64 << 53) as f64;
            let value = if spec.choices.is_empty() {
                spec.minimum + unit * (spec.maximum - spec.minimum)
            } else {
                f64::from(spec.choices[(unit * spec.choices.len() as f64) as usize].0)
            };
            assert!(engine.set_parameter(index, value));
        }
        assert!(engine.set_parameter(parameters::REGISTER, MASTER));
        engine
            .bellows_mut()
            .expression_wide(if trial % 2 == 0 { 1.0 } else { 0.2 });
        engine.note_on(REED_KEY, 1.0);
        let out = render(&mut engine, 0.5);
        assert!(
            out.iter().all(|x| x.is_finite()) && engine.supply().is_finite(),
            "trial {trial}"
        );
    }
}
