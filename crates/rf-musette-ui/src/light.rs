//! The one light the whole panel is drawn under, after RF-5's.
//!
//! Every highlight, cast shadow and shading gradient takes its direction
//! from [`LIGHT_SOURCE_BEARING_DEGREES`] and [`LIGHT_ELEVATION_DEGREES`].
//! The stylesheet reads them through the CSS variables [`css_variables`]
//! writes on the document root; the celluloid's pearl ([`crate::texture`])
//! is lit by the same vector when it is generated. Moving the light is these
//! two constants, and the pearl image written again.

/// Where the light comes from as seen on the screen, clockwise from straight
/// up: 0 is overhead, 315 the upper left -- RF-5's, so RackForge's
/// instruments share one room.
pub const LIGHT_SOURCE_BEARING_DEGREES: f64 = 315.0;

/// Height of the light above the panel, in degrees.
pub const LIGHT_ELEVATION_DEGREES: f64 = 40.0;

/// Unit vector from a surface towards the light, in screen coordinates
/// (x to the right, y down).
pub fn toward_light() -> (f64, f64) {
    let bearing = LIGHT_SOURCE_BEARING_DEGREES.to_radians();
    (bearing.sin(), -bearing.cos())
}

/// Unit vector along which shadows fall: away from the light.
pub fn shadow_direction() -> (f64, f64) {
    let (x, y) = toward_light();
    (-x, -y)
}

/// CSS gradient angle running from the lit side to the shaded side.
pub fn shading_angle_degrees() -> f64 {
    (LIGHT_SOURCE_BEARING_DEGREES + 180.0).rem_euclid(360.0)
}

/// Declarations for the document root, read by every shadow and lighting
/// gradient in the stylesheet.
pub fn css_variables() -> String {
    let (light_x, light_y) = toward_light();
    let (shadow_x, shadow_y) = shadow_direction();
    format!(
        "--light-x:{light_x:.4};--light-y:{light_y:.4};--shadow-x:{shadow_x:.4};--shadow-y:{shadow_y:.4};--light-angle:{:.2}deg;--grille-pitch:{};--grille-bars:{};--grille-glint:{};--grille-sheen:{};--coat-gloss:{}",
        shading_angle_degrees(),
        grille_pitch(),
        grille_bars(),
        grille_glint(),
        grille_sheen(),
        coat_gloss()
    )
}

pub(crate) fn normalize([x, y, z]: [f64; 3]) -> [f64; 3] {
    let length = (x * x + y * y + z * z).sqrt();
    [x / length, y / length, z / length]
}

pub(crate) fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// Unit vector towards the light in panel space: x right, y down, z out of
/// the panel towards the viewer.
pub fn light_vector() -> [f64; 3] {
    let (x, y) = toward_light();
    let elevation = LIGHT_ELEVATION_DEGREES.to_radians();
    [x * elevation.cos(), y * elevation.cos(), elevation.sin()]
}

/// The celluloid's clear coat: a smooth acrylic layer over the pearl, its
/// refractive index that of PMMA. It reflects a little of the light at its
/// face (Fresnel) and bends the rest towards the normal before it reaches
/// the pearl.
pub const COAT_INDEX: f64 = 1.49;
/// The coat's polish, as Ward's roughness: smooth, broadened a little by
/// the light's own size.
const COAT_ROUGHNESS: f64 = 0.15;
/// How far the eye and the light stand from the screen, in its widths.
const COAT_DISTANCE: f64 = 1.1;

/// Schlick's approximation of the reflectance of the coat's face at
/// `cos_theta` from its normal.
pub fn coat_fresnel(cos_theta: f64) -> f64 {
    let r0 = ((COAT_INDEX - 1.0) / (COAT_INDEX + 1.0)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cos_theta.clamp(0.0, 1.0)).powi(5)
}

/// The panel light inside the coat: bent towards the normal by Snell's law,
/// so it reaches the pearl from higher than it stands outside.
pub fn refracted_light_vector() -> [f64; 3] {
    let light = light_vector();
    let sin_inside = (1.0 - light[2] * light[2]).sqrt() / COAT_INDEX;
    let (x, y) = toward_light();
    let cos_inside = (1.0 - sin_inside * sin_inside).sqrt();
    [x * sin_inside, y * sin_inside, cos_inside]
}

/// The share of the light that reaches the pearl and comes back out to an
/// eye straight in front: through the face twice.
pub fn coat_transmission() -> f64 {
    (1.0 - coat_fresnel(light_vector()[2])) * (1.0 - coat_fresnel(1.0))
}

/// Where on the screen the coat mirrors the light, in units of the distance
/// to the eye from the screen's centre: the eye reflected through the face,
/// joined to the light.
fn coat_mirror_point() -> (f64, f64) {
    let light = light_vector();
    (light[0] / (1.0 + light[2]), light[1] / (1.0 + light[2]))
}

/// The coat's gloss at a point (units of the distance), as a share of the
/// light a white matte surface would return there: Ward's normalised
/// isotropic lobe with Fresnel's reflectance, rho / (4 alpha^2
/// sqrt(cos_i cos_o)) exp(-tan^2(theta_h) / alpha^2).
fn coat_gloss_at(x: f64, y: f64) -> f64 {
    let light = light_vector();
    let to_eye = normalize([-x, -y, 1.0]);
    let to_light = normalize([light[0] - x, light[1] - y, light[2]]);
    let half = normalize([
        to_eye[0] + to_light[0],
        to_eye[1] + to_light[1],
        to_eye[2] + to_light[2],
    ]);
    let tan2 = (half[0] * half[0] + half[1] * half[1]) / (half[2] * half[2]);
    let reflectance = coat_fresnel(dot(half, to_eye));
    reflectance / (4.0 * COAT_ROUGHNESS * COAT_ROUGHNESS * (to_light[2] * to_eye[2]).sqrt())
        * (-tan2 / (COAT_ROUGHNESS * COAT_ROUGHNESS)).exp()
}

/// The coat's gloss as a radial gradient on the viewport, centred where the
/// coat mirrors the light (for an upper-left light, above the screen's top
/// edge, so its lower flank is what shows); its stops are the lobe out to
/// where it fades.
pub fn coat_gloss() -> String {
    let (mx, my) = coat_mirror_point();
    let reach = 3.0 * std::f64::consts::SQRT_2 * COAT_ROUGHNESS;
    let steps = 16;
    let stops: Vec<String> = (0..=steps)
        .map(|step| {
            let t = f64::from(step) / f64::from(steps);
            let gloss = coat_gloss_at(mx + t * reach, my).min(0.85);
            format!("rgba(255, 246, 240, {gloss:.3}) {:.2}%", t * 100.0)
        })
        .collect();
    format!(
        "radial-gradient(circle {:.2}vw at calc(50vw + {:.2}vw) calc(50vh + {:.2}vw), {}, transparent)",
        reach * COAT_DISTANCE * 100.0,
        mx * COAT_DISTANCE * 100.0,
        my * COAT_DISTANCE * 100.0,
        stops.join(", ")
    )
}

/// Ward's (1992) anisotropic specular term for a facet with `normal`, rough
/// `alpha_x` along `tangent` and `alpha_y` across it, lit by the panel light
/// and seen from straight in front, written with the half vector H in the
/// facet's frame as exp(-((H.t / H.n)^2 / alpha_x^2 + (H.b / H.n)^2 /
/// alpha_y^2)). A facet turned away from the light reflects nothing.
pub fn ward_specular(normal: [f64; 3], tangent: [f64; 3], alpha_x: f64, alpha_y: f64) -> f64 {
    let light = light_vector();
    if dot(light, normalize(normal)) <= 0.0 {
        return 0.0;
    }
    let half = normalize([light[0], light[1], light[2] + 1.0]);
    ward_term(half, normal, tangent, alpha_x, alpha_y)
}

/// Ward's term for a given half vector: the light and the eye need not be
/// at infinity, as across a surface wide against their distance.
pub(crate) fn ward_term(
    half: [f64; 3],
    normal: [f64; 3],
    tangent: [f64; 3],
    alpha_x: f64,
    alpha_y: f64,
) -> f64 {
    let n = normalize(normal);
    let along = dot(tangent, n);
    let t = normalize([
        tangent[0] - n[0] * along,
        tangent[1] - n[1] * along,
        tangent[2] - n[2] * along,
    ]);
    let b = cross(n, t);
    let h_n = dot(half, n);
    if h_n <= 0.0 {
        return 0.0;
    }
    let x = dot(half, t) / h_n;
    let y = dot(half, b) / h_n;
    (-(x * x / (alpha_x * alpha_x) + y * y / (alpha_y * alpha_y))).exp()
}

/// The half vector at `u` across a surface, in units of the distance from
/// its centre to the eye (straight in front) and to the light (along
/// [`light_vector`]): it turns as the point moves, which is what draws a
/// highlight as a lobe and not as an even wash.
fn half_vector_at(u: f64) -> [f64; 3] {
    let light = light_vector();
    let to_eye = normalize([-u, 0.0, 1.0]);
    let to_light = normalize([light[0] - u, light[1], light[2]]);
    normalize([
        to_eye[0] + to_light[0],
        to_eye[1] + to_light[1],
        to_eye[2] + to_light[2],
    ])
}

/// The grille's bars: polished chrome strips, round in section, running
/// across the panel. Every bar holds every normal in its section, so some
/// line of each one faces the half vector; along the bar the strip is
/// polished smooth, so the light it returns falls off fast as the half
/// vector turns across the panel. Ward's term therefore draws the glint as
/// a band across the bars -- the anisotropic streak of brushed metal or a
/// record's grooves -- on the side towards the light.
const BAR_PITCH: f64 = 7.0;
const BAR_WIDTH: f64 = 5.0;
/// Roughness along the bar (polished) and across it.
const BAR_ALONG: f64 = 0.18;
const BAR_ACROSS: f64 = 0.3;
/// How far the eye and the light stand, in widths of the grille.
const GRILLE_DISTANCE: f64 = 1.1;

/// A bar's normal at `s`, -1 at its top edge to 1 at its foot (y down).
fn bar_normal(s: f64) -> [f64; 3] {
    let s = s.clamp(-0.95, 0.95);
    [0.0, s, (1.0 - s * s).sqrt()]
}

/// Where across the grille (0 left, 1 right) the streak is brightest, and
/// how bright it is at `x`: Ward's term along the bar at the half vector
/// there, for the bar's facet that faces it best.
pub fn grille_streak(x: f64) -> f64 {
    let half = half_vector_at((x - 0.5) / GRILLE_DISTANCE);
    let best = normalize([0.0, half[1], half[2]]);
    ward_term(half, best, [1.0, 0.0, 0.0], BAR_ALONG, BAR_ACROSS)
}

/// The bars' pitch, for the stylesheet: one bar and its gap make one tile,
/// which CSS repeats with `round`, stretching the pitch a little so a whole
/// number of bars fills the grille and none is cut.
pub fn grille_pitch() -> String {
    format!("{BAR_PITCH:.0}px")
}

/// Where `y` pixels into a bar falls, as a share of its tile: the gap above
/// it, then the bar, so the grille ends on a whole bar's foot.
fn in_tile(y: f64) -> f64 {
    100.0 * (BAR_PITCH - BAR_WIDTH + y) / BAR_PITCH
}

/// The gap above a bar, as a share of its tile.
fn gap_in_tile() -> f64 {
    in_tile(0.0)
}

/// A gap and its bar, top to bottom: each mixes the panel's light chrome
/// into black by how much light its section returns -- the room above
/// reflected on its upper side, and the panel light's diffuse.
pub fn grille_bars() -> String {
    let light = light_vector();
    let steps = 10;
    let mut stops = Vec::new();
    for step in 0..=steps {
        let y = BAR_WIDTH * f64::from(step) / f64::from(steps);
        let n = bar_normal(2.0 * y / BAR_WIDTH - 1.0);
        let level = (0.1 + 0.3 * dot(n, light).max(0.0) + 0.25 * (-n[1]).max(0.0)).clamp(0.0, 1.0);
        stops.push(format!(
            "color-mix(in srgb, var(--chrome-light) {:.0}%, var(--lacquer)) {:.2}%",
            level * 100.0,
            in_tile(y)
        ));
    }
    format!(
        "linear-gradient(180deg, var(--lacquer) 0 {:.2}%, {})",
        gap_in_tile(),
        stops.join(", ")
    )
}

/// Where in each bar's section the glint falls: Ward's term across the bar
/// at the streak's own half vector, as a mask for the streak.
pub fn grille_glint() -> String {
    let peak = (0..=100)
        .map(|step| f64::from(step) / 100.0)
        .max_by(|a, b| grille_streak(*a).total_cmp(&grille_streak(*b)))
        .unwrap_or(0.5);
    let half = half_vector_at((peak - 0.5) / GRILLE_DISTANCE);
    let steps = 10;
    let mut stops = Vec::new();
    for step in 0..=steps {
        let y = BAR_WIDTH * f64::from(step) / f64::from(steps);
        let n = bar_normal(2.0 * y / BAR_WIDTH - 1.0);
        let glint = ward_term(half, n, [1.0, 0.0, 0.0], BAR_ALONG, BAR_ACROSS);
        stops.push(format!("rgba(0, 0, 0, {glint:.3}) {:.2}%", in_tile(y)));
    }
    format!(
        "linear-gradient(180deg, transparent 0 {:.2}%, {})",
        gap_in_tile(),
        stops.join(", ")
    )
}

/// The streak across the grille as a horizontal gradient of white.
pub fn grille_sheen() -> String {
    let steps = 40;
    let stops: Vec<String> = (0..=steps)
        .map(|step| {
            let x = f64::from(step) / f64::from(steps);
            format!(
                "rgba(255, 255, 255, {:.3}) {:.1}%",
                0.95 * grille_streak(x),
                x * 100.0
            )
        })
        .collect();
    format!("linear-gradient(90deg, {})", stops.join(", "))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1.0e-9
    }

    #[test]
    fn upper_left_light_casts_shadows_down_and_right() {
        let (x, y) = toward_light();
        assert!(x < 0.0 && y < 0.0 && close(x, y));
        let (x, y) = shadow_direction();
        assert!(x > 0.0 && y > 0.0 && close(x * x + y * y, 1.0));
        assert!(close(shading_angle_degrees(), 135.0));
    }

    #[test]
    fn ward_lights_the_facets_that_face_the_light() {
        let tilt = 35.0_f64.to_radians();
        let (s, c) = (tilt.sin(), tilt.cos());
        let across = [1.0, 0.0, 0.0];
        let down = [0.0, 1.0, 0.0];
        let face = ward_specular([0.0, 0.0, 1.0], across, 0.4, 0.4);
        let top = ward_specular([0.0, -s, c], across, 0.4, 0.4);
        let left = ward_specular([-s, 0.0, c], down, 0.4, 0.4);
        let right = ward_specular([s, 0.0, c], down, 0.4, 0.4);
        assert!(close(top, left), "an upper-left light treats both alike");
        assert!(top > face && face > right && right < 0.01);
    }

    /// The streak runs across the bars on the side towards the light,
    /// narrow along them; in each bar the glint sits on its upper side,
    /// which faces the light above.
    #[test]
    fn the_grille_glints_towards_the_light() {
        let peak = (0..=100)
            .map(|step| f64::from(step) / 100.0)
            .max_by(|a, b| grille_streak(*a).total_cmp(&grille_streak(*b)))
            .unwrap();
        assert!(peak < 0.5 && peak > 0.05, "{peak}");
        assert!(grille_streak(peak) > 0.9);
        assert!(grille_streak(0.95) < 0.1 * grille_streak(peak));
        let glint = grille_glint();
        let alphas: Vec<f64> = glint
            .split("rgba(0, 0, 0, ")
            .skip(1)
            .map(|rest| rest.split(')').next().unwrap().parse().unwrap())
            .collect();
        // Eleven steps down the bar: 1 px in against 4 px in.
        assert!(alphas[2] > alphas[8], "{glint}");
        // The gap first, then the bar to the tile's foot: the grille ends
        // on a whole bar.
        let bars = grille_bars();
        assert!(bars.starts_with("linear-gradient(180deg, var(--lacquer) 0 28.57%, color-mix("));
        assert!(bars.ends_with(" 100.00%)"), "{bars}");
    }

    /// Inside the coat the light stands higher and a little weaker; the
    /// gloss peaks where the coat mirrors the light and is the coat's
    /// reflectance spread over Ward's lobe.
    #[test]
    fn the_coat_bends_the_light_and_mirrors_it() {
        let inside = refracted_light_vector();
        assert!(inside[2] > light_vector()[2]);
        assert!(close(dot(inside, inside), 1.0));
        let elevation = inside[2].asin().to_degrees();
        assert!((elevation - 59.1).abs() < 0.5, "{elevation}");
        let transmission = coat_transmission();
        assert!(transmission > 0.9 && transmission < 0.93, "{transmission}");
        let (mx, my) = coat_mirror_point();
        assert!(mx < 0.0 && my < 0.0);
        let peak = coat_gloss_at(mx, my);
        assert!(peak > coat_gloss_at(mx + 0.1, my) && peak > coat_gloss_at(0.0, 0.0));
        assert!(peak > 0.3 && peak < 1.0, "{peak}");
    }

    #[test]
    fn css_variables_carry_the_same_light() {
        let css = css_variables();
        assert!(css.contains("--light-x:-0.7071"));
        assert!(css.contains("--shadow-y:0.7071"));
        assert!(css.contains("--light-angle:135.00deg"));
    }
}
