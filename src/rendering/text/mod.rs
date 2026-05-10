// I despise how much duplicate code is present in the text rendering functions

use parking_lot::RwLock;

/// Caches drawn text
///
/// See [`GlyphCache`] for the composition of this type
pub static GLYPH_CACHE: GlyphCache =
    std::sync::LazyLock::new(|| RwLock::new(MapType::new_map()));

/// ### Key:
///
/// `char`: Requested letter
///
/// `(i32, i32)`: Dimensions
///
/// `usize`: Hash (so multiple fonts can be used at the same time)
///
///
/// ### Data:
///
/// [`fontdue::Metrics`]: Positioning data
///
/// `Vec<u8>`: Rasterized font data (alpha)
pub type GlyphCache = std::sync::LazyLock<
    RwLock<MapType<(char, u32, usize), (fontdue::Metrics, Vec<u8>)>>,
>;
// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// struct GlyphKey(char, u32, usize);

/// Get a glyph from the cache if it exists
#[inline(always)]
#[allow(clippy::inline_always)]
#[must_use]
pub fn _get_glyph_cache() -> &'static GlyphCache {
    &GLYPH_CACHE
}
/// Reset the glyph cache
pub fn _reset_glyph_cache() {
    GLYPH_CACHE.write().clear();
}
/// Removes a selected glyph from the glyph cache
pub fn _remove_glyph_from_glyph_cache(glyph: &(char, u32, usize)) {
    GLYPH_CACHE.write().remove_thingy(glyph);
}
/// Manually add a glyph to the glyph cache
pub fn _add_to_glyph_cache(
    key: (char, u32, usize),
    data: (fontdue::Metrics, Vec<u8>),
) {
    GLYPH_CACHE.write().insert(key, data);
}
const PRECISION_MULTIPLIER: f32 = 10000.0;
#[inline(always)]
fn round_float_key(value: f32) -> u32 {
    // Round to some precision first if needed
    let rounded = (value * PRECISION_MULTIPLIER).round() / PRECISION_MULTIPLIER;
    rounded.to_bits()
}

mod aliased;
mod antialiased;
pub use aliased::*;
pub use antialiased::*;

use crate::{
    render::{BufferMetrics, BufferMisc, BufferPointers},
    settings::{MapType, SettingsMapType},
};

/// Switch between aliased and antialiased text rendering
///
/// When `antialiased` is [None], the drawn text respects alpha, otherwise it expects an alpha cutoff
pub fn draw_text_switch<const SAFE: bool>(
    buffer: &mut (impl BufferPointers + BufferMetrics + BufferMisc),
    text: &str,
    xy: (usize, usize),
    color: u32,
    size: f32,
    font: &fontdue::Font,
    antialiased: Option<u8>,
) {
    if let Some(val) = antialiased {
        draw_text::<SAFE>(buffer, text, xy, color, size, font, val);
    } else {
        draw_text_antialiased::<SAFE>(buffer, text, xy, color, size, font);
    }
}
/// Switch between aliased and antialiased text rendering in isize space
///
/// When `antialiased` is [None], the drawn text respects alpha, otherwise it expects an alpha cutoff
pub fn draw_text_switch_isize<const SAFE: bool>(
    buffer: &mut (impl BufferPointers + BufferMetrics + BufferMisc),
    text: &str,
    xy: (isize, isize),
    color: u32,
    size: f32,
    font: &fontdue::Font,
    antialiased: Option<u8>,
) {
    if let Some(val) = antialiased {
        draw_text_isize::<SAFE>(buffer, text, xy, color, size, font, val);
    } else {
        draw_text_antialiased_isize::<SAFE>(
            buffer, text, xy, color, size, font,
        );
    }
}

// #[inline]
// pub fn draw_text_angled_aliased(
//     buffer: &Buffer,
//     text: &str,
//     x: usize,
//     y: usize,
//     color: u32,
//     size: f32,
//     font: &fontdue::Font,
//     angle: f32,
// ) {
//     draw_text_angled_aliased_impl(
//         buffer,
//         text,
//         x,
//         y,
//         color,
//         size,
//         angle,
//         font,
//         draw_pixel_safe,
//     );
// }

// #[inline]
// pub fn draw_text_angled_aliased_unsafe(
//     buffer: &Buffer,
//     text: &str,
//     x: usize,
//     y: usize,
//     color: u32,
//     size: f32,
//     font: &fontdue::Font,
//     angle: f32,
// ) {
//     draw_text_angled_aliased_impl(
//         buffer,
//         text,
//         x,
//         y,
//         color,
//         size,
//         angle,
//         font,
//         draw_pixel_unsafe,
//     );
// }

/// Returns the metrics and bitmap of the character from cache (creating and rasterizing it if needed)
pub fn get_character(
    ch: char,
    size: f32,
    font: &fontdue::Font,
) -> parking_lot::MappedRwLockReadGuard<'static, (fontdue::Metrics, Vec<u8>)> {
    let rounded_size_key = round_float_key(size);
    let cache_key = (ch, rounded_size_key, font.file_hash());

    // Fast path: try to get from cache
    {
        let cache = GLYPH_CACHE.read();
        if core::intrinsics::likely(cache.contains_key(&cache_key)) {
            return parking_lot::RwLockReadGuard::map(cache, |c| unsafe {
                c.get(&cache_key).unwrap_unchecked()
            });
        }
    }

    // Slow path: rasterize and cache
    {
        let mut cache = GLYPH_CACHE.write();
        // Double-check in case another thread inserted while we were waiting
        if cache.get(&cache_key).is_some() {
            // Downgrade to read lock and return
            drop(cache);
            let read_cache = GLYPH_CACHE.read();
            return parking_lot::RwLockReadGuard::map(read_cache, |c| unsafe {
                c.get(&cache_key).unwrap_unchecked()
            });
        }

        // Actually rasterize
        let rasterized = font.rasterize(ch, size);
        cache.insert(cache_key, rasterized);
    }

    // Return the newly cached item
    let cache = GLYPH_CACHE.read();
    parking_lot::RwLockReadGuard::map(cache, |c| unsafe {
        c.get(&cache_key).unwrap_unchecked()
    })
}
/// Get the length of a string in a font if it was rendered out
#[must_use]
pub fn get_text_width(string: &str, size: f32, font: &fontdue::Font) -> f32 {
    let mut total_width = 0.0;

    for ch in string.chars() {
        let metrics = get_character(ch, size, font).0;

        total_width += metrics.advance_width;
    }

    total_width
}
/// Get the height of a string in a font if it was rendered out
#[must_use]
pub fn get_text_height(string: &str, size: f32, font: &fontdue::Font) -> f32 {
    let mut max_height = size;
    let mut min_height = 0.0;

    for ch in string.chars() {
        let metrics = get_character(ch, size, font).0;

        if metrics.height as f32 > max_height {
            max_height = metrics.height as f32;
        }
        if (metrics.ymin as f32) < min_height {
            min_height = metrics.ymin as f32;
        }
    }

    max_height - min_height
}
