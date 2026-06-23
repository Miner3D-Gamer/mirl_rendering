// I despise how much duplicate code is present in the text rendering functions

use std::clone::Share;

// use mirl::settings::SettingsMapType;
use mirl_extensions::{IntoPatch, MapLike};
use parking_lot::RwLock;
// /// Caches drawn text
// ///
// /// See [`GlyphCache`] for the composition of this type
// pub static GLYPH_CACHE: GlyphCache = std::sync::LazyLock::new(|| RwLock::new(MapType::new_map()));

/// A trait used by text drawing functions
pub trait GlyphCache {
    /// Clear the entire glyph cache
    fn clear(&mut self);
    /// Remove from glyph cache
    fn remove(&mut self, glyph: Glyph) -> Option<std::sync::Arc<GlyphData>>;
    /// Insert a glyph, replacing the old
    fn insert(&mut self, glyph: Glyph) -> std::sync::Arc<GlyphData>;
    /// Get a glyph from cache or add it if it doesn't exist
    fn get_or_insert(&mut self, glyph: Glyph) -> std::sync::Arc<GlyphData>;
    /// Get a glyph from the cache if it exists
    fn get(&self, glyph: Glyph) -> Option<std::sync::Arc<GlyphData>>;
    /// Get a reference to the internal font used
    fn get_font(&self) -> &fontdue::Font;
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
/// The size of the glyph
///
/// Internally a f32 wrapper that will always be a valid value
pub struct GlyphSize {
    /// The size of the glyph
    pub _size: f32,
}
impl Eq for GlyphSize {}
#[allow(clippy::derive_ord_xor_partial_ord)]
impl Ord for GlyphSize {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { self.partial_cmp(other).unwrap_unchecked() }
    }
}

impl std::hash::Hash for GlyphSize {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u32(u32::from_le_bytes(self._size.to_le_bytes()));
    }
}

impl GlyphSize {
    #[must_use]
    #[inline]
    /// Create a new [`GlyphSize`]
    ///
    /// Returns [`None`] if the given f32 is negative, Nan/Inf, or so big it looses notable precision
    pub fn new<T: IntoPatch<f32>>(size: T) -> Option<Self> {
        let size = size.into_value();
        if size < 0.0 {
            return None;
        }
        if !size.is_normal() {
            return None;
        }
        Some(Self { _size: size })
    }
    #[inline(always)]
    #[must_use]
    /// Get the inner value
    pub const fn inner(self) -> f32 {
        self._size
    }
    #[inline(always)]
    #[must_use]
    /// Get the value as T where f32 can turn into T
    pub fn get<T>(self) -> T
    where
        f32: IntoPatch<T>,
    {
        self._size.into_value()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A single glyph
pub struct Glyph {
    /// The internal char
    pub char: char,
    /// The size of the char
    pub size: GlyphSize,
    /// The color of the glyph
    pub color: u32,
}
impl Glyph {
    #[must_use]
    #[inline]
    /// Create a new glyph using a [`char`], it' [`GlyphSize`], and its color packed in a u32 (rgba)
    pub const fn new(char: char, size: GlyphSize, color: u32) -> Self {
        Self { char, size, color }
    }
}

/// The data required to draw a glyph
pub type GlyphData = (fontdue::Metrics, Vec<u8>);

#[derive(Debug)]
/// The default glyph cache
pub struct DefaultGlyphCache<HashMap: MapLike<Glyph, std::sync::Arc<GlyphData>>> {
    /// The cache containing
    pub _cache: parking_lot::RwLock<HashMap>,
    /// The font
    pub _font: fontdue::Font,
}
impl<HashMap: MapLike<Glyph, std::sync::Arc<GlyphData>> + PartialEq> PartialEq
    for DefaultGlyphCache<HashMap>
{
    fn eq(&self, other: &Self) -> bool {
        self._cache.read().eq(&self._cache.read())
            && are_fontdue_fonts_eq(&self._font, &other._font)
    }
}
#[must_use]
/// Checks if 2 fontdue font files are the same
///
/// Does not check `horizontal_kern`
pub fn are_fontdue_fonts_eq(f1: &fontdue::Font, f2: &fontdue::Font) -> bool {
    f1.chars().eq(f2.chars())
        && f1.name() == f2.name()
        && f1.units_per_em() == f2.units_per_em()
        && f1.glyph_count() == f2.glyph_count()
        && f1.horizontal_line_metrics(1.0) == f2.horizontal_line_metrics(1.0)
        && f1.vertical_line_metrics(1.0) == f2.vertical_line_metrics(1.0)
        && format!("{f1:?}") == format!("{f2:?}")
}

impl<HashMap: MapLike<Glyph, std::sync::Arc<GlyphData>> + Clone> Clone
    for DefaultGlyphCache<HashMap>
{
    fn clone(&self) -> Self {
        Self {
            _cache: RwLock::new(self._cache.read().clone()),
            _font: self._font.clone(),
        }
    }
}
impl<HashMap: MapLike<Glyph, std::sync::Arc<GlyphData>> + Default> DefaultGlyphCache<HashMap> {
    #[must_use]
    /// Create a new empty glyph cache
    pub fn new(font: fontdue::Font) -> Self {
        Self {
            _cache: RwLock::new(HashMap::default()),
            _font: font,
        }
    }
}
impl<HashMap: MapLike<Glyph, std::sync::Arc<GlyphData>> + Clone> GlyphCache
    for DefaultGlyphCache<HashMap>
{
    fn clear(&mut self) {
        let lock = self._cache.get_mut();
        for (key, _values) in lock.clone().iter() {
            lock.remove(key);
        }
    }

    fn remove(&mut self, glyph: Glyph) -> Option<std::sync::Arc<GlyphData>> {
        let lock = self._cache.get_mut();
        lock.remove(&glyph)
    }

    fn insert(&mut self, glyph: Glyph) -> std::sync::Arc<GlyphData> {
        let lock = self._cache.get_mut();
        let rasterized = self._font.rasterize(glyph.char, glyph.size._size);
        let arc = std::sync::Arc::new(rasterized);
        lock.insert(glyph, arc.share());
        arc
    }

    fn get_or_insert<'a>(&mut self, glyph: Glyph) -> std::sync::Arc<GlyphData> {
        self.get(glyph).unwrap_or_else(|| self.insert(glyph))
    }

    fn get<'a>(&self, glyph: Glyph) -> Option<std::sync::Arc<GlyphData>> {
        let lock = self._cache.read();
        lock.get(&glyph).map(std::clone::Share::share)
    }
    fn get_font(&self) -> &fontdue::Font {
        &self._font
    }
}

// /// ### Key:
// ///
// /// `char`: Requested letter
// ///
// /// `(i32, i32)`: Dimensions
// ///
// /// `usize`: Hash (so multiple fonts can be used at the same time)
// ///
// ///
// /// ### Data:
// ///
// /// [`fontdue::Metrics`]: Positioning data
// ///
// /// `Vec<u8>`: Rasterized font data (alpha)
// pub type GlyphCache =
//     std::sync::LazyLock<RwLock<MapType<(char, u32, usize), (fontdue::Metrics, Vec<u8>)>>>;
// // #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// // struct GlyphKey(char, u32, usize);

// /// Get a glyph from the cache if it exists
// #[inline(always)]
// #[allow(clippy::inline_always)]
// #[must_use]
// pub fn _get_glyph_cache() -> &'static GlyphCache {
//     &GLYPH_CACHE
// }
// /// Reset the glyph cache
// pub fn _reset_glyph_cache() {
//     GLYPH_CACHE.write().clear();
// }
// /// Removes a selected glyph from the glyph cache
// pub fn _remove_glyph_from_glyph_cache(glyph: &(char, u32, usize)) {
//     GLYPH_CACHE.write().remove_thingy(glyph);
// }
// /// Manually add a glyph to the glyph cache
// pub fn _add_to_glyph_cache(key: (char, u32, usize), data: (fontdue::Metrics, Vec<u8>)) {
//     GLYPH_CACHE.write().insert(key, data);
// }
// const PRECISION_MULTIPLIER: f32 = 10000.0;
// #[inline(always)]
// #[must_use]
// /// Round the given text size in float format to be rendered
// pub fn _round_float_for_text_size(value: f32) -> u32 {
//     // Round to some precision first if needed
//     let rounded = (value * PRECISION_MULTIPLIER).round() / PRECISION_MULTIPLIER;
//     rounded.to_bits()
// }

mod aliased;
mod antialiased;
pub use aliased::*;
pub use antialiased::*;
// use crate::{
//     render::{BufferMetrics, BufferMisc, BufferPointers},
//     settings::{MapType, SettingsMapType},
// };

use mirl_buffer::traits::*;
/// Switch between aliased and antialiased text rendering
///
/// When `antialiased` is [None], the drawn text respects alpha, otherwise it expects an alpha cutoff
pub fn draw_text_switch<const SAFE: bool, T: crate::GlyphCache>(
    buffer: &mut (impl BufferPointers + BufferMetrics),
    text: &str,
    xy: (usize, usize),
    color: u32,
    size: GlyphSize,
    glyph_cache: &mut T,
    antialiased: Option<u8>,
) {
    if let Some(val) = antialiased {
        draw_text::<SAFE, T>(buffer, text, xy, color, size, glyph_cache, val);
    } else {
        draw_text_antialiased::<SAFE, T>(buffer, text, xy, color, size, glyph_cache);
    }
}
/// Switch between aliased and antialiased text rendering in isize space
///
/// When `antialiased` is [None], the drawn text respects alpha, otherwise it expects an alpha cutoff
pub fn draw_text_switch_isize<const SAFE: bool, T: crate::GlyphCache>(
    buffer: &mut (impl BufferPointers + BufferMetrics),
    text: &str,
    xy: (isize, isize),
    color: u32,
    size: GlyphSize,
    glyph_cache: &mut T,
    antialiased: Option<u8>,
) {
    if let Some(val) = antialiased {
        draw_text_isize::<SAFE, T>(buffer, text, xy, color, size, glyph_cache, val);
    } else {
        draw_text_antialiased_isize::<SAFE, T>(buffer, text, xy, color, size, glyph_cache);
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

// /// Returns the metrics and bitmap of the character from cache (creating and rasterizing it if needed)
// pub fn get_character(
//     ch: char,
//     size: f32,
//     font: &fontdue::Font,
// ) -> parking_lot::MappedRwLockReadGuard<'static, (fontdue::Metrics, Vec<u8>)> {
//     let rounded_size_key = _round_float_for_text_size(size);
//     let cache_key = (ch, rounded_size_key, font.file_hash());

//     // Fast path: try to get from cache
//     {
//         let cache = GLYPH_CACHE.read();
//         if core::intrinsics::likely(cache.contains_key(&cache_key)) {
//             return parking_lot::RwLockReadGuard::map(cache, |c| unsafe {
//                 c.get(&cache_key).unwrap_unchecked()
//             });
//         }
//     }

//     // Slow path: rasterize and cache
//     {
//         let mut cache = GLYPH_CACHE.write();
//         // Double-check in case another thread inserted while we were waiting
//         if cache.get(&cache_key).is_some() {
//             // Downgrade to read lock and return
//             drop(cache);
//             let read_cache = GLYPH_CACHE.read();
//             return parking_lot::RwLockReadGuard::map(read_cache, |c| unsafe {
//                 c.get(&cache_key).unwrap_unchecked()
//             });
//         }

//         // Actually rasterize
//         let rasterized = font.rasterize(ch, size);
//         cache.insert(cache_key, rasterized);
//     }

//     // Return the newly cached item
//     let cache = GLYPH_CACHE.read();
//     parking_lot::RwLockReadGuard::map(cache, |c| unsafe { c.get(&cache_key).unwrap_unchecked() })
// }
/// Get the length of a string in a font if it was rendered out
#[must_use]
pub fn get_text_width<T: GlyphCache>(string: &str, size: GlyphSize, cache: &mut T) -> f32 {
    let mut total_width = 0.0;

    for ch in string.chars() {
        let metrics = cache.get_or_insert(Glyph::new(ch, size, 0)).0;

        total_width += metrics.advance_width;
    }

    total_width
}
/// Get the height of a string in a font if it was rendered out
#[must_use]
pub fn get_text_height<T: GlyphCache>(string: &str, size: GlyphSize, cache: &mut T) -> f32 {
    let mut max_height = size.inner();
    let mut min_height = 0.0;

    for ch in string.chars() {
        let metrics = cache.get_or_insert(Glyph::new(ch, size, 0)).0;

        if metrics.height as f32 > max_height {
            max_height = metrics.height as f32;
        }
        if (metrics.ymin as f32) < min_height {
            min_height = metrics.ymin as f32;
        }
    }

    max_height - min_height
}
