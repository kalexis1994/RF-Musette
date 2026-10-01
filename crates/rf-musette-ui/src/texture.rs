//! The celluloid's pearl, generated after RF-5's panel finish.
//!
//! Made offline, not when the panel starts: the test at the foot of this
//! file writes the images to `package/web/assets/` (with
//! `RF_MUSETTE_WRITE_FINISHES=1`) and otherwise checks the shipped ones
//! still match, so the page only loads images and stacks them with CSS.
//!
//! Pearloid is chunks of pearl celluloid swirled in solvent, cured and
//! sliced (photographs: a red pearloid sheet, Rothko & Frost RF0031; a
//! Santucci accordion in ruby red; `docs/SOURCES.md`). Its face is a mosaic
//! of angular shards of many sizes, overlapping at different depths in the
//! clear red: each shard a plate of aligned flakes, often creased into two
//! facets, so it lights as a whole -- a few bright, most dim -- and the
//! deeper ones fade into the red. Its density is even across the sheet.
//!
//! Here each depth is a layer of shards: the cells of a weighted Voronoi
//! diagram, so they differ in size, each shrunk by its own margin so the
//! layer below shows between them. A shard has a tilt, a crease (a second
//! tilt past a line through it), a slight bend and the fine shimmer of its
//! flakes; its normal is lit by the panel light as it reaches the pearl
//! through the acrylic coat ([`light::refracted_light_vector`]): a soft
//! diffuse and Ward's lobe. A shard is opaque celluloid, so its colour is
//! its light -- dark red turned away, saturated red, pink-white at the
//! glint, the light having crossed the red twice -- and its alpha its
//! cover; CSS stacks the layers top first.
//!
//! Each layer is its own image on its own torus, repeating without a seam
//! at its own period; the periods share no factor, so the stack repeats
//! only after their least common multiple, billions of pixels on.

use crate::light;

/// One layer of shards: its image, its period in CSS pixels (the image is
/// twice that, for double-density screens), and how it differs from the
/// others.
pub struct Layer {
    pub path: &'static str,
    pub period: usize,
    seed: u32,
    /// How much of the light it returns, deeper in the red.
    dim: f64,
    /// The widest margin round a shard, in spacings: the layer below shows
    /// through it.
    margin: f64,
    /// Its shards' spacing, in image pixels.
    spacing: f64,
}

/// The shards' mean spacing in the top layers, in image pixels.
const SPACING: f64 = 47.0;

/// Top first, as CSS stacks them.
pub const LAYERS: [Layer; 4] = [
    Layer {
        path: "assets/pearl-top-a.png",
        period: 331,
        seed: 61,
        dim: 1.0,
        margin: 0.55,
        spacing: SPACING,
    },
    Layer {
        path: "assets/pearl-top-b.png",
        period: 277,
        seed: 62,
        dim: 1.0,
        margin: 0.55,
        spacing: SPACING,
    },
    Layer {
        path: "assets/pearl-mid.png",
        period: 239,
        seed: 63,
        dim: 0.55,
        margin: 0.15,
        spacing: SPACING * 0.8,
    },
    Layer {
        path: "assets/pearl-deep.png",
        period: 197,
        seed: 64,
        dim: 0.3,
        margin: 0.0,
        spacing: SPACING * 0.6,
    },
];

/// How far a shard tilts (standard deviation, radians), and its second
/// facet from its first.
const TILT: f64 = 0.3;
const CREASE_TILT: f64 = 0.24;
/// The share of shards creased into two facets.
const CREASED: f64 = 0.6;
/// How much a shard bends: tilt per spacing.
const BEND: f64 = 0.25;
/// The flakes' shimmer in the tilt, and its grain in image pixels.
const SHIMMER: f64 = 0.03;
const SHIMMER_GRAIN: f64 = 4.0;
/// How much larger than the smallest a shard may grow, in spacings.
const GROWTH: f64 = 0.45;
/// The soft edge of a shard, in image pixels.
const EDGE: f64 = 1.2;
/// The light a shard returns: diffuse, and Ward's lobe (isotropic).
const DIFFUSE: f64 = 0.25;
const ROUGHNESS: f64 = 0.2;
const AMBIENT: f64 = 0.1;
const GAIN: f64 = 0.8;
/// A whole shard's opacity.
const OPACITY: f64 = 0.94;

/// Dark red, saturated red and the pearl's glint.
const DARK: [f64; 3] = [40.0, 2.0, 8.0];
const RED: [f64; 3] = [236.0, 38.0, 52.0];
const WHITE: [f64; 3] = [255.0, 214.0, 206.0];
/// The light where a shard is the saturated red, and where it is white.
const RED_AT: f64 = 0.6;
const WHITE_AT: f64 = 1.2;

fn hash(a: u32, b: u32, seed: u32) -> f64 {
    // A small integer hash: stable, so the images are the same on every build.
    let mut h =
        a.wrapping_mul(0x27d4_eb2d) ^ b.wrapping_mul(0x1656_67b1) ^ seed.wrapping_mul(0x9e37_79b9);
    h ^= h >> 15;
    h = h.wrapping_mul(0x85eb_ca6b);
    h ^= h >> 13;
    h = h.wrapping_mul(0xc2b2_ae35);
    h ^= h >> 16;
    f64::from(h) / f64::from(u32::MAX)
}

/// A standard normal deviate from two hashes (Box-Muller).
fn normal(a: u32, b: u32, seed: u32) -> f64 {
    let u = hash(a, b, seed).max(1.0e-12);
    let v = hash(a, b, seed.wrapping_add(1));
    (-2.0 * u.ln()).sqrt() * (std::f64::consts::TAU * v).cos()
}

/// Value noise on a torus `size` pixels round with `cells` cells across.
fn value_noise(x: f64, y: f64, size: usize, cells: usize, seed: u32) -> f64 {
    let cell = size as f64 / cells as f64;
    let (fx, fy) = (x / cell, y / cell);
    let (ix, iy) = (fx.floor(), fy.floor());
    let smooth = |t: f64| t * t * (3.0 - 2.0 * t);
    let (tx, ty) = (smooth(fx - ix), smooth(fy - iy));
    let wrap = |i: f64| (i as i64).rem_euclid(cells as i64) as u32;
    let (x0, y0) = (wrap(ix), wrap(iy));
    let (x1, y1) = ((x0 + 1) % cells as u32, (y0 + 1) % cells as u32);
    let top = hash(x0, y0, seed) * (1.0 - tx) + hash(x1, y0, seed) * tx;
    let bottom = hash(x0, y1, seed) * (1.0 - tx) + hash(x1, y1, seed) * tx;
    top * (1.0 - ty) + bottom * ty
}

/// The flakes' shimmer: two octaves of value noise, zero-mean, with the
/// standard deviation `SHIMMER` (value noise of this kind has a standard
/// deviation near 0.2).
fn shimmer(x: f64, y: f64, size: usize, seed: u32) -> f64 {
    let cells = ((size as f64 / SHIMMER_GRAIN).round() as usize).max(1);
    let noise = 0.7 * value_noise(x, y, size, cells, seed)
        + 0.3 * value_noise(x, y, size, (cells * 2).max(1), seed + 1);
    (noise - 0.5) * SHIMMER / 0.17
}

struct Shard {
    x: f64,
    y: f64,
    grow: f64,
    margin: f64,
    tilt: [f64; 2],
    crease_tilt: [f64; 2],
    crease: Option<(f64, f64)>,
    bend: [f64; 2],
}

/// The image's edge in pixels.
pub fn size(layer: &Layer) -> usize {
    2 * layer.period
}

/// The shortest offset from `b` to `a` round a torus `size` across.
fn wrapped(a: f64, b: f64, size: f64) -> f64 {
    (a - b + size / 2.0).rem_euclid(size) - size / 2.0
}

/// RGBA bytes of a layer's image, row by row.
pub fn layer_rgba(layer: &Layer) -> Vec<u8> {
    let size = size(layer);
    let edge = size as f64;
    let spacing = layer.spacing;
    let count = ((edge / spacing).powi(2).round() as usize).max(4);
    let seed = layer.seed;
    let shards: Vec<Shard> = (0..count as u32)
        .map(|i| Shard {
            x: hash(i, 0, seed) * edge,
            y: hash(i, 1, seed) * edge,
            grow: hash(i, 2, seed) * GROWTH * spacing,
            margin: hash(i, 3, seed).powi(2) * layer.margin * spacing,
            tilt: [TILT * normal(i, 4, seed), TILT * normal(i, 6, seed)],
            crease_tilt: [
                CREASE_TILT * normal(i, 8, seed),
                CREASE_TILT * normal(i, 10, seed),
            ],
            crease: (hash(i, 12, seed) < CREASED).then(|| {
                (
                    hash(i, 13, seed) * std::f64::consts::PI,
                    (hash(i, 14, seed) - 0.5) * 0.6 * spacing,
                )
            }),
            bend: [
                BEND / spacing * normal(i, 15, seed),
                BEND / spacing * normal(i, 17, seed),
            ],
        })
        .collect();
    // Buckets a spacing wide, so the nearest shards lie within two of a
    // pixel's own.
    let buckets = ((edge / spacing).floor() as usize).max(1);
    let bucket = edge / buckets as f64;
    let mut grid: Vec<Vec<usize>> = vec![Vec::new(); buckets * buckets];
    for (index, shard) in shards.iter().enumerate() {
        let bx = ((shard.x / bucket) as usize).min(buckets - 1);
        let by = ((shard.y / bucket) as usize).min(buckets - 1);
        grid[by * buckets + bx].push(index);
    }
    let reach = 2_i64.min(buckets as i64 / 2).max(1);
    let light = light::refracted_light_vector();
    let half = light::normalize([light[0], light[1], light[2] + 1.0]);
    let transmission = light::coat_transmission();
    let mut rgba = Vec::with_capacity(size * size * 4);
    for py in 0..size {
        for px in 0..size {
            let (x, y) = (px as f64 + 0.5, py as f64 + 0.5);
            let (bx, by) = ((x / bucket) as i64, (y / bucket) as i64);
            // The two nearest shards by weighted distance.
            let mut first = (f64::INFINITY, 0);
            let mut second = f64::INFINITY;
            let mut seen = Vec::with_capacity(32);
            for oy in -reach..=reach {
                for ox in -reach..=reach {
                    let cell = (by + oy).rem_euclid(buckets as i64) as usize * buckets
                        + (bx + ox).rem_euclid(buckets as i64) as usize;
                    for &index in &grid[cell] {
                        if seen.contains(&index) {
                            continue;
                        }
                        seen.push(index);
                        let shard = &shards[index];
                        let dx = wrapped(x, shard.x, edge);
                        let dy = wrapped(y, shard.y, edge);
                        let power = (dx * dx + dy * dy).sqrt() - shard.grow;
                        if power < first.0 {
                            second = first.0;
                            first = (power, index);
                        } else if power < second {
                            second = power;
                        }
                    }
                }
            }
            let shard = &shards[first.1];
            let cover = ((second - first.0 - shard.margin) / EDGE).clamp(0.0, 1.0);
            let ox = wrapped(x, shard.x, edge);
            let oy = wrapped(y, shard.y, edge);
            let facet = match shard.crease {
                Some((angle, offset)) if ox * angle.cos() + oy * angle.sin() > offset => [
                    shard.tilt[0] + shard.crease_tilt[0],
                    shard.tilt[1] + shard.crease_tilt[1],
                ],
                _ => shard.tilt,
            };
            let tx = facet[0] + shard.bend[0] * ox + shimmer(x, y, size, seed + 1000);
            let ty = facet[1] + shard.bend[1] * oy + shimmer(x, y, size, seed + 2000);
            let normal = light::normalize([tx, ty, 1.0]);
            let facing = light::dot(normal, light);
            let lit = if facing > 0.0 {
                DIFFUSE * facing
                    + light::ward_term(half, normal, [1.0, 0.0, 0.0], ROUGHNESS, ROUGHNESS)
            } else {
                0.0
            };
            let brightness =
                (transmission * layer.dim * (AMBIENT + lit) * GAIN).clamp(0.0, WHITE_AT + 0.2);
            let low = (brightness / RED_AT).clamp(0.0, 1.0);
            let high = ((brightness - RED_AT) / (WHITE_AT - RED_AT)).clamp(0.0, 1.0);
            let colour: [u8; 3] = core::array::from_fn(|c| {
                let base = DARK[c] + (RED[c] - DARK[c]) * low;
                (base + (WHITE[c] - base) * high).round() as u8
            });
            let alpha = (cover * OPACITY * 255.0).round() as u8;
            rgba.extend_from_slice(&[colour[0], colour[1], colour[2], alpha]);
        }
    }
    rgba
}

/// The stack as CSS background layers, top first, each at its period.
pub fn css_layers() -> String {
    LAYERS
        .iter()
        .map(|layer| {
            format!(
                "url(\"{}\") 0 0 / {p}px {p}px repeat",
                layer.path,
                p = layer.period
            )
        })
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 { a } else { gcd(b, a % b) }
    }

    /// No two periods share a factor, so the stack does not repeat on any
    /// screen.
    #[test]
    fn the_periods_share_no_factor() {
        for (i, a) in LAYERS.iter().enumerate() {
            for b in &LAYERS[i + 1..] {
                assert_eq!(gcd(a.period, b.period), 1, "{} {}", a.period, b.period);
            }
        }
        let repeat: f64 = LAYERS.iter().map(|layer| layer.period as f64).product();
        assert!(repeat > 1.0e9);
    }

    /// Each layer's image wraps without a seam: the pixels either side of
    /// its edge belong to shards that cross it.
    #[test]
    fn a_layer_wraps_without_a_seam() {
        let layer = &LAYERS[3];
        let size = size(layer);
        let rgba = layer_rgba(layer);
        let alpha = |x: usize, y: usize| f64::from(rgba[(y * size + x) * 4 + 3]);
        let across_edge: f64 = (0..size)
            .map(|y| (alpha(0, y) - alpha(size - 1, y)).abs())
            .sum();
        let inside: f64 = (0..size)
            .map(|y| (alpha(size / 2, y) - alpha(size / 2 - 1, y)).abs())
            .sum();
        assert!(across_edge < 2.0 * inside + 255.0, "{across_edge} {inside}");
    }

    /// The top layers leave room for the ones below; the deepest covers
    /// all but its seams (a soft edge of `EDGE` either side, about a tenth
    /// of its spacing); a few shards glint, most are red.
    #[test]
    fn the_shards_overlap_and_few_glint() {
        let share = |layer: &Layer, test: &dyn Fn(&[u8]) -> bool| {
            let rgba = layer_rgba(layer);
            let pixels = rgba.as_chunks::<4>().0;
            pixels.iter().filter(|pixel| test(&pixel[..])).count() as f64 / pixels.len() as f64
        };
        let covered = |pixel: &[u8]| pixel[3] > 200;
        let top = share(&LAYERS[0], &covered);
        let deep = share(&LAYERS[3], &covered);
        assert!(top > 0.4 && top < 0.85, "top {top}");
        assert!(deep > 0.85, "deep {deep}");
        let glint = share(&LAYERS[0], &|pixel: &[u8]| pixel[3] > 200 && pixel[1] > 120);
        assert!(glint > 0.002 && glint < 0.15, "glint {glint}");
    }

    /// The stylesheet stacks the layers this file makes, at their periods.
    #[test]
    fn the_stylesheet_stacks_these_layers() {
        let styles = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../package/web/styles.css"),
        )
        .unwrap();
        let declared: String = styles
            .split("--pearl-layers:")
            .nth(1)
            .and_then(|rest| rest.split(';').next())
            .unwrap()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        assert_eq!(declared, css_layers());
    }

    /// The shipped images are exactly what the generator makes; run with
    /// `RF_MUSETTE_WRITE_FINISHES=1` to write them after changing it.
    #[test]
    fn the_shipped_pearl_matches_its_generator() {
        let write = std::env::var_os("RF_MUSETTE_WRITE_FINISHES").is_some();
        for layer in &LAYERS {
            let file = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../package/web")
                .join(layer.path);
            let size = size(layer);
            let rgba = layer_rgba(layer);
            if write {
                std::fs::create_dir_all(file.parent().unwrap()).unwrap();
                let mut encoder = png::Encoder::new(
                    std::io::BufWriter::new(std::fs::File::create(&file).unwrap()),
                    size as u32,
                    size as u32,
                );
                encoder.set_color(png::ColorType::Rgba);
                encoder.set_depth(png::BitDepth::Eight);
                encoder.set_compression(png::Compression::Best);
                let mut writer = encoder.write_header().unwrap();
                writer.write_image_data(&rgba).unwrap();
                continue;
            }
            let decoder = png::Decoder::new(std::io::BufReader::new(
                std::fs::File::open(&file)
                    .unwrap_or_else(|error| panic!("{}: {error}", layer.path)),
            ));
            let mut reader = decoder.read_info().unwrap();
            let mut pixels = vec![0; reader.output_buffer_size()];
            let info = reader.next_frame(&mut pixels).unwrap();
            assert_eq!((info.width as usize, info.height as usize), (size, size));
            assert_eq!(info.color_type, png::ColorType::Rgba);
            assert!(
                pixels[..info.buffer_size()] == rgba[..],
                "{} differs from its generator",
                layer.path
            );
        }
    }
}
