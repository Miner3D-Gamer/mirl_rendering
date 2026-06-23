#![allow(clippy::similar_names, clippy::cast_possible_wrap)]
#![allow(clippy::significant_drop_tightening)]

use mirl_buffer::{draw_pixel_safe, draw_pixel_unsafe, traits::*};

use crate::{Glyph, GlyphSize};

#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
/// Draw text in the specified font
pub fn draw_text_antialiased<const SAFE: bool, T: crate::GlyphCache>(
    buffer: &mut (impl BufferPointers + BufferMetrics),
    text: &str,
    xy: (usize, usize),
    color: u32,
    size: GlyphSize,
    glyph_cache: &mut T,
) {
    let mut pen_x = xy.0 as f32;
    let pen_y = xy.1;

    let ascent = glyph_cache
        .get_font()
        .horizontal_line_metrics(size.inner())
        .map_or(0, |font_metrics| font_metrics.ascent as usize);

    for ch in text.chars() {
        // Try to get the glyph from cache first
        let char = glyph_cache.get_or_insert(Glyph::new(ch, size, color));
        let metrics = char.0;
        let bitmap = &char.1;

        let offset_y = ascent.saturating_sub(metrics.height);
        // Draw each character into the buffer
        for gy in 0..metrics.height {
            for gx in 0..metrics.width {
                let px = pen_x as usize + gx;
                // Correcting for letter height
                let py = ((pen_y + gy + offset_y) as i32 - metrics.ymin) as usize;

                if px < buffer.width() && py < buffer.height() {
                    let index = py * buffer.width() + px;
                    let alpha = bitmap[gy * metrics.width + gx]; // Alpha (0-255)

                    if alpha > 0 {
                        unsafe {
                            let bg = *buffer.pointer().add(index);
                            // Extract RGBA
                            let (br, bg, bb, ba) = (
                                (bg >> 24) & 0xFF,
                                (bg >> 16) & 0xFF,
                                (bg >> 8) & 0xFF,
                                bg & 0xFF,
                            );
                            let (tr, tg, tb, ta) = (
                                (color >> 24) & 0xFF,
                                (color >> 16) & 0xFF,
                                (color >> 8) & 0xFF,
                                color & 0xFF,
                            );

                            // Alpha blending
                            let inv_alpha: u8 = 255 - alpha;
                            let nr = ((tr as u16 * u16::from(alpha)
                                + br as u16 * u16::from(inv_alpha))
                                / 255) as u8;
                            let ng = ((tg as u16 * u16::from(alpha)
                                + bg as u16 * u16::from(inv_alpha))
                                / 255) as u8;
                            let nb = ((tb as u16 * u16::from(alpha)
                                + bb as u16 * u16::from(inv_alpha))
                                / 255) as u8;
                            let na = ((ta as u16 * u16::from(alpha)
                                + ba as u16 * u16::from(inv_alpha))
                                / 255) as u8;

                            let new = u32::from(nr) << 24
                                | u32::from(ng) << 16
                                | u32::from(nb) << 8
                                | u32::from(na);

                            if SAFE {
                                draw_pixel_safe(buffer, (px, py), new);
                            } else {
                                draw_pixel_unsafe(buffer, (px, py), new);
                            }
                        }
                    }
                }
            }
        }

        // Advance the cursor position
        pen_x += metrics.advance_width;
    }
}

/// Execute a function at every pixel position
///
/// function: `fn(original_color: u32, color_under_pixel: u32) -> u32`
pub fn draw_text_antialiased_execute_at<const SAFE: bool, T: crate::GlyphCache>(
    buffer: &mut (impl BufferPointers + BufferMetrics + BufferGetPixel),
    text: &str,
    xy: (usize, usize),
    color: u32,
    size: GlyphSize,
    glyph_cache: &mut T,
    function: impl Fn(u32, u32) -> u32,
) {
    let mut pen_x = xy.0 as f32;
    let pen_y = xy.1;

    let ascent = glyph_cache
        .get_font()
        .horizontal_line_metrics(size.inner())
        .map_or(0, |font_metrics| font_metrics.ascent as usize);

    for ch in text.chars() {
        // Try to get the glyph from cache first
        let char = glyph_cache.get_or_insert(Glyph::new(ch, size, color));
        let metrics = char.0;
        let bitmap = &char.1;

        let offset_y = ascent.saturating_sub(metrics.height);
        // Draw each character into the buffer
        for gy in 0..metrics.height {
            for gx in 0..metrics.width {
                let px = pen_x as usize + gx;
                // Correcting for letter height
                let py = ((pen_y + gy + offset_y) as i32 - metrics.ymin) as usize;

                if px < buffer.width() && py < buffer.height() {
                    let index = py * buffer.width() + px;
                    let alpha = bitmap[gy * metrics.width + gx]; // Alpha (0-255)
                    let new_color = function(color, buffer.get_pixel((px, py)));

                    if alpha > 0 {
                        unsafe {
                            let bg = *buffer.pointer().add(index);
                            // Extract RGBA
                            let (br, bg, bb, ba) = (
                                (bg >> 24) & 0xFF,
                                (bg >> 16) & 0xFF,
                                (bg >> 8) & 0xFF,
                                bg & 0xFF,
                            );
                            let (tr, tg, tb, ta) = (
                                (new_color >> 24) & 0xFF,
                                (new_color >> 16) & 0xFF,
                                (new_color >> 8) & 0xFF,
                                new_color & 0xFF,
                            );

                            // Alpha blending
                            let inv_alpha: u8 = 255 - alpha;
                            let nr = ((tr as u16 * u16::from(alpha)
                                + br as u16 * u16::from(inv_alpha))
                                / 255) as u8;
                            let ng = ((tg as u16 * u16::from(alpha)
                                + bg as u16 * u16::from(inv_alpha))
                                / 255) as u8;
                            let nb = ((tb as u16 * u16::from(alpha)
                                + bb as u16 * u16::from(inv_alpha))
                                / 255) as u8;
                            let na = ((ta as u16 * u16::from(alpha)
                                + ba as u16 * u16::from(inv_alpha))
                                / 255) as u8;
                            let new = u32::from(nr) << 24
                                | u32::from(ng) << 16
                                | u32::from(nb) << 8
                                | u32::from(na);

                            if SAFE {
                                draw_pixel_safe(buffer, (px, py), new);
                            } else {
                                draw_pixel_unsafe(buffer, (px, py), new);
                            }
                        }
                    }
                }
            }
        }

        // Advance the cursor position
        pen_x += metrics.advance_width;
    }
}

/// Draw text yet stretch the resulting characters
pub fn draw_text_antialiased_stretched<const SAFE: bool, T: crate::GlyphCache>(
    buffer: &mut (impl BufferPointers + BufferMetrics + BufferGetPixel),
    text: &str,
    xy: (usize, usize),
    color: u32,
    size: GlyphSize,
    glyph_cache: &mut T,
    stretch: (f32, f32),
) {
    let mut pen_x = xy.0 as f32;
    let pen_y = xy.1;
    let ascent = glyph_cache
        .get_font()
        .horizontal_line_metrics(size.inner())
        .map_or(0, |font_metrics| font_metrics.ascent as usize);

    for ch in text.chars() {
        let char = glyph_cache.get_or_insert(Glyph::new(ch, size, color));
        let metrics = char.0;
        let bitmap = &char.1;

        let offset_y = ascent.saturating_sub(metrics.height);
        let w = metrics.width as f32 * stretch.0;
        let h = metrics.height as f32 * stretch.1;
        let advance_x = metrics.advance_width;

        for gy in 0..h.floor() as usize {
            let py = ((pen_y + gy + offset_y) as i32 - metrics.ymin) as usize;
            if py >= buffer.height() {
                continue;
            }

            let bitmap_row_start = ((gy as f32 / stretch.1) as usize) * metrics.width;
            for gx in 0..w.floor() as usize {
                let px = pen_x as usize + gx;
                if px >= buffer.width() {
                    continue;
                }

                let bitmap_index = bitmap_row_start + (gx as f32 / stretch.0) as usize;
                let alpha = bitmap[bitmap_index];

                if alpha > 0 {
                    let bg = buffer.get_pixel((px, py)); // Fixed: use px instead of gx
                    // Extract RGBA
                    let (br, bg_g, bb, ba) = (
                        (bg >> 24) & 0xFF,
                        (bg >> 16) & 0xFF,
                        (bg >> 8) & 0xFF,
                        bg & 0xFF,
                    );
                    let (tr, tg, tb, ta) = (
                        (color >> 24) & 0xFF,
                        (color >> 16) & 0xFF,
                        (color >> 8) & 0xFF,
                        color & 0xFF,
                    );

                    // Alpha blending
                    let inv_alpha: u8 = 255 - alpha;
                    let nr = ((tr as u16 * u16::from(alpha) + br as u16 * u16::from(inv_alpha))
                        / 255) as u8;
                    let ng = ((tg as u16 * u16::from(alpha) + bg_g as u16 * u16::from(inv_alpha))
                        / 255) as u8;
                    let nb = ((tb as u16 * u16::from(alpha) + bb as u16 * u16::from(inv_alpha))
                        / 255) as u8;
                    let na = ((ta as u16 * u16::from(alpha) + ba as u16 * u16::from(inv_alpha))
                        / 255) as u8;

                    let new = u32::from(nr) << 24
                        | u32::from(ng) << 16
                        | u32::from(nb) << 8
                        | u32::from(na);

                    if SAFE {
                        draw_pixel_safe(buffer, (px, py), new);
                    } else {
                        draw_pixel_unsafe(buffer, (px, py), new);
                    }
                }
            }
        }

        pen_x += advance_x;
    }
}

/// Same as [`draw_text_antialiased`] but uses isize for positioning allowing for partially out of bounds text (left and top)
pub fn draw_text_antialiased_isize<const SAFE: bool, T: crate::GlyphCache>(
    buffer: &mut (impl BufferPointers + BufferMetrics),
    text: &str,
    xy: (isize, isize),
    color: u32,
    size: GlyphSize,
    glyph_cache: &mut T,
) {
    let mut pen_x = xy.0 as f32;
    let pen_y = xy.1;

    let ascent = glyph_cache
        .get_font()
        .horizontal_line_metrics(size.inner())
        .map_or(0, |font_metrics| font_metrics.ascent as isize);

    for ch in text.chars() {
        // Try to get the glyph from cache first
        let char = glyph_cache.get_or_insert(Glyph::new(ch, size, color));
        let metrics = char.0;
        let bitmap = &char.1;

        let offset_y = ascent.saturating_sub(metrics.height as isize);
        // Draw each character into the buffer
        for gy in 0..metrics.height {
            for gx in 0..metrics.width {
                let px = pen_x as isize + gx as isize;
                // Correcting for letter height
                let py = pen_y + gy as isize + offset_y - metrics.ymin as isize;

                // Check if pixel is within buffer bounds
                if px >= 0
                    && py >= 0
                    && px < buffer.width() as isize
                    && py < buffer.height() as isize
                {
                    let pixel_x_new = px as usize;
                    let pixel_y_new = py as usize;
                    let index = pixel_y_new * buffer.width() + pixel_x_new;
                    let alpha = bitmap[gy * metrics.width + gx]; // Alpha (0-255)
                    if alpha > 0 {
                        if SAFE && index >= buffer.width() * buffer.height() {
                            continue;
                        }

                        unsafe {
                            let bg = *buffer.pointer().add(index);
                            // Extract RGBA
                            let (br, bg, bb, ba) = (
                                (bg >> 24) & 0xFF,
                                (bg >> 16) & 0xFF,
                                (bg >> 8) & 0xFF,
                                bg & 0xFF,
                            );
                            let (tr, tg, tb, ta) = (
                                (color >> 24) & 0xFF,
                                (color >> 16) & 0xFF,
                                (color >> 8) & 0xFF,
                                color & 0xFF,
                            );

                            // Alpha blending
                            let inv_alpha: u8 = 255 - alpha;
                            let nr = ((tr as u16 * u16::from(alpha)
                                + br as u16 * u16::from(inv_alpha))
                                / 255) as u8;
                            let ng = ((tg as u16 * u16::from(alpha)
                                + bg as u16 * u16::from(inv_alpha))
                                / 255) as u8;
                            let nb = ((tb as u16 * u16::from(alpha)
                                + bb as u16 * u16::from(inv_alpha))
                                / 255) as u8;
                            let na = ((ta as u16 * u16::from(alpha)
                                + ba as u16 * u16::from(inv_alpha))
                                / 255) as u8;
                            if SAFE {
                                draw_pixel_safe(
                                    buffer,
                                    (pixel_x_new, pixel_y_new),
                                    u32::from(nr) << 24
                                        | u32::from(ng) << 16
                                        | u32::from(nb) << 8
                                        | u32::from(na),
                                );
                            } else {
                                draw_pixel_unsafe(
                                    buffer,
                                    (pixel_x_new, pixel_y_new),
                                    u32::from(nr) << 24
                                        | u32::from(ng) << 16
                                        | u32::from(nb) << 8
                                        | u32::from(na),
                                );
                            }
                        }
                    }
                }
            }
        }

        // Advance the cursor position
        pen_x += metrics.advance_width;
    }
}

/// Same as [`draw_text_antialiased_stretched`] but uses isize for positioning allowing for partially out of bounds text (left and top)
pub fn draw_text_antialiased_stretched_isize<const SAFE: bool, T: crate::GlyphCache>(
    buffer: &mut (impl BufferPointers + BufferMetrics + BufferGetPixel),
    text: &str,
    xy: (isize, isize),
    color: u32,
    size: GlyphSize,
    glyph_cache: &mut T,
    stretch: (f32, f32),
) {
    let mut pen_x = xy.0 as f32;
    let pen_y = xy.1;
    let ascent = glyph_cache
        .get_font()
        .horizontal_line_metrics(size.inner())
        .map_or(0, |font_metrics| font_metrics.ascent as isize);

    for ch in text.chars() {
        let char = glyph_cache.get_or_insert(Glyph::new(ch, size, color));
        let metrics = char.0;
        let bitmap = &char.1;

        let offset_y = ascent.saturating_sub(metrics.height as isize);
        let w = metrics.width as f32 * stretch.0;
        let h = metrics.height as f32 * stretch.1;
        let advance_x = metrics.advance_width;

        for gy in 0..h.floor() as usize {
            let py = pen_y + gy as isize + offset_y - metrics.ymin as isize;
            if py < 0 || py >= buffer.height() as isize {
                continue;
            }

            let bitmap_row_start = ((gy as f32 / stretch.1) as usize) * metrics.width;
            for gx in 0..w.floor() as usize {
                let px = pen_x as isize + gx as isize;
                if px < 0 || px >= buffer.width() as isize {
                    continue;
                }

                let bitmap_index = bitmap_row_start + (gx as f32 / stretch.0) as usize;
                let alpha = bitmap[bitmap_index];

                if alpha > 0 {
                    let px_new = px as usize;
                    let py_new = py as usize;
                    let index = py_new * buffer.width() + px_new;

                    if SAFE && index >= buffer.width() * buffer.height() {
                        continue;
                    }

                    let bg = buffer.get_pixel((px_new, py_new));
                    // Extract RGBA
                    let (br, bg_g, bb, ba) = (
                        (bg >> 24) & 0xFF,
                        (bg >> 16) & 0xFF,
                        (bg >> 8) & 0xFF,
                        bg & 0xFF,
                    );
                    let (tr, tg, tb, ta) = (
                        (color >> 24) & 0xFF,
                        (color >> 16) & 0xFF,
                        (color >> 8) & 0xFF,
                        color & 0xFF,
                    );

                    // Alpha blending
                    let inv_alpha: u8 = 255 - alpha;
                    let nr = ((tr as u16 * u16::from(alpha) + br as u16 * u16::from(inv_alpha))
                        / 255) as u8;
                    let ng = ((tg as u16 * u16::from(alpha) + bg_g as u16 * u16::from(inv_alpha))
                        / 255) as u8;
                    let nb = ((tb as u16 * u16::from(alpha) + bb as u16 * u16::from(inv_alpha))
                        / 255) as u8;
                    let na = ((ta as u16 * u16::from(alpha) + ba as u16 * u16::from(inv_alpha))
                        / 255) as u8;

                    let new = u32::from(nr) << 24
                        | u32::from(ng) << 16
                        | u32::from(nb) << 8
                        | u32::from(na);

                    if SAFE {
                        draw_pixel_safe(buffer, (px as usize, py as usize), new);
                    } else {
                        draw_pixel_unsafe(buffer, (px as usize, py as usize), new);
                    }
                }
            }
        }

        pen_x += advance_x;
    }
}

// /// Draw text uniformly
// pub fn draw_text_antialiased_monospace<const SAFE: bool>(
//     buffer: &Buffer,
//     text: &str,
//     x: usize,
//     y: usize,
//     color: u32,
//     size: f32,
//     font: &fontdue::Font,
// ) {
//     let draw_pixel: DrawPixelFunction = {
//         if SAFE {
//             draw_pixel_safe
//         } else {
//             draw_pixel_unsafe
//         }
//     };
//     let mut pen_x = x;
//     let pen_y = xy.1

//     let font_metrics = font.horizontal_line_metrics(size).unwrap();
//     let ascent = font_metrics.ascent as usize;

//     for ch in text.chars() {
//         // Try to get the glyph from cache first
//         let char = get_character(ch, size, &font);
//         let metrics = char.0;
//         let bitmap = &char.1;

//         let offset_y = ascent.saturating_sub(metrics.height);
//         // Draw each character into the buffer
//         for gy in 0..metrics.height {
//             for gx in 0..metrics.width {
//                 let px = pen_x + gx;
//                 // Correcting for letter height
//                 let py =
//                     ((pen_y + gy + offset_y) as i32 - metrics.ymin) as usize;

//                 if px < buffer.width() && py < buffer.height() {
//                     let index = py * buffer.width() + px;
//                     let alpha = bitmap[gy * metrics.width + gx]; // Alpha (0-255)

//                     if alpha > 0 {
//                         unsafe {
//                             let bg = *buffer.pointer().add(index);
//                             // Extract RGBA
//                             let (br, bg, bb, ba) = (
//                                 (bg >> 24) & 0xFF,
//                                 (bg >> 16) & 0xFF,
//                                 (bg >> 8) & 0xFF,
//                                 bg & 0xFF,
//                             );
//                             let (tr, tg, tb, ta) = (
//                                 (color >> 24) & 0xFF,
//                                 (color >> 16) & 0xFF,
//                                 (color >> 8) & 0xFF,
//                                 color & 0xFF,
//                             );

//                             // Alpha blending
//                             let inv_alpha: u8 = 255 - alpha;
//                             let nr = ((tr as u16 * alpha as u16
//                                 + br as u16 * inv_alpha as u16)
//                                 / 255)
//                                 as u8;
//                             let ng = ((tg as u16 * alpha as u16
//                                 + bg as u16 * inv_alpha as u16)
//                                 / 255)
//                                 as u8;
//                             let nb = ((tb as u16 * alpha as u16
//                                 + bb as u16 * inv_alpha as u16)
//                                 / 255)
//                                 as u8;
//                             let na = ((ta as u16 * alpha as u16
//                                 + ba as u16 * inv_alpha as u16)
//                                 / 255)
//                                 as u8;

//                             draw_pixel(
//                                 buffer,
//                                 px,
//                                 py,
//                                 (nr as u32) << 24
//                                     | (ng as u32) << 16
//                                     | (nb as u32) << 8
//                                     | na as u32,
//                             );
//                         }
//                     }
//                 }
//             }
//         }

//         // Advance the cursor position
//         pen_x += size as usize;
//     }
// }
