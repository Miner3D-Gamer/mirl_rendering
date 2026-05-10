#![allow(clippy::similar_names)]
#![allow(clippy::significant_drop_tightening)]
use super::get_character;
use crate::render::{
    BufferMetrics, BufferPointers, draw_pixel_safe, draw_pixel_unsafe,
};
/// Draw text in the specified font
pub fn draw_text<const SAFE: bool>(
    buffer: &mut (impl BufferPointers + BufferMetrics),
    text: &str,
    xy: (usize, usize),
    color: u32,
    size: f32,
    font: &fontdue::Font,
    alpha_cutoff: u8,
) {
    let mut pen_x = xy.0 as f32;
    let pen_y = xy.1;
    let ascent = font
        .horizontal_line_metrics(size)
        .map_or(0, |font_metrics| font_metrics.ascent as usize);

    for ch in text.chars() {
        // If not in cache, rasterize and insert
        let char = get_character(ch, size, font);
        let metrics = char.0;
        let bitmap = &char.1;

        let offset_y = ascent.saturating_sub(metrics.height);
        let w = metrics.width;
        let h = metrics.height;
        let advance_x = metrics.advance_width;

        for gy in 0..h {
            let py = ((pen_y + gy + offset_y) as i32 - metrics.ymin) as usize;
            if py >= buffer.height() {
                continue;
            }

            let row_start = gy * w;
            for gx in 0..w {
                let px = pen_x as usize + gx;
                if px >= buffer.width() {
                    continue;
                }
                let alpha = bitmap[gy * metrics.width + gx];
                if alpha < alpha_cutoff {
                    continue;
                }

                if bitmap[row_start + gx] > 0 {
                    if SAFE {
                        draw_pixel_safe(buffer, (px, py), color);
                    } else {
                        draw_pixel_unsafe(buffer, (px, py), color);
                    }
                }
            }
        }
        pen_x += advance_x;
    }
}

/// Draw text yet stretch the resulting characters
pub fn draw_text_stretched<const SAFE: bool>(
    buffer: &mut (impl BufferPointers + BufferMetrics),
    text: &str,
    xy: (usize, usize),
    color: u32,
    size: f32,
    font: &fontdue::Font,
    stretch: (f32, f32),
) {
    let mut pen_x = xy.0 as f32;
    let pen_y = xy.1;
    let ascent = font
        .horizontal_line_metrics(size)
        .map_or(0, |font_metrics| font_metrics.ascent as usize);

    for ch in text.chars() {
        // If not in cache, rasterize and insert
        let char = get_character(ch, size, font);
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

            let bitmap_row_start =
                ((gy as f32 / stretch.1) as usize) * metrics.width;
            for gx in 0..w.floor() as usize {
                let px = pen_x as usize + gx;
                if px >= buffer.width() {
                    continue;
                }

                if bitmap[bitmap_row_start + (gx as f32 / stretch.0) as usize]
                    > 0
                {
                    if SAFE {
                        draw_pixel_safe(buffer, (px, py), color);
                    } else {
                        draw_pixel_unsafe(buffer, (px, py), color);
                    }
                }
            }
        }
        pen_x += advance_x;
    }
}

/// Same as [`draw_text`] but uses isize for positioning allowing for partially out of bounds text (left and top)
pub fn draw_text_isize<const SAFE: bool>(
    buffer: &mut (impl BufferPointers + BufferMetrics),
    text: &str,
    xy: (isize, isize),
    color: u32,
    size: f32,
    font: &fontdue::Font,
    alpha_cutoff: u8,
) {
    let mut pen_x = xy.0 as f32;
    let pen_y = xy.1;
    let ascent = font
        .horizontal_line_metrics(size)
        .map_or(0, |font_metrics| font_metrics.ascent as isize);

    for ch in text.chars() {
        // If not in cache, rasterize and insert
        let char = get_character(ch, size, font);
        let metrics = char.0;
        let bitmap = &char.1;

        let offset_y = ascent.saturating_sub(metrics.height as isize);
        let w = metrics.width;
        let h = metrics.height;
        let advance_x = metrics.advance_width;

        for gy in 0..h {
            let py = pen_y + gy as isize + offset_y - metrics.ymin as isize;
            if py < 0 || py >= buffer.height() as isize {
                continue;
            }

            let row_start = gy * w;
            for gx in 0..w {
                let px = pen_x as isize + gx as isize;
                if px < 0 || px >= buffer.width() as isize {
                    continue;
                }
                let alpha = bitmap[gy * metrics.width + gx];
                if alpha < alpha_cutoff {
                    continue;
                }
                if bitmap[row_start + gx] > 0 {
                    let px_new = px as usize;
                    let py_new = py as usize;
                    let index = py_new * buffer.width() + px_new;

                    if SAFE && index >= buffer.width() * buffer.height() {
                        continue;
                    }

                    if SAFE {
                        draw_pixel_safe(buffer, (px_new, py_new), color);
                    } else {
                        draw_pixel_unsafe(buffer, (px_new, py_new), color);
                    }
                }
            }
        }
        pen_x += advance_x;
    }
}

/// Same as [`draw_text_stretched`] but uses isize for positioning allowing for partially out of bounds text (left and top)
pub fn draw_text_stretch_isize<const SAFE: bool>(
    buffer: &mut (impl BufferPointers + BufferMetrics),
    text: &str,
    xy: (isize, isize),
    color: u32,
    size: f32,
    font: &fontdue::Font,
    stretch: (f32, f32),
) {
    let mut pen_x = xy.0 as f32;
    let pen_y = xy.1;
    let ascent = font
        .horizontal_line_metrics(size)
        .map_or(0, |font_metrics| font_metrics.ascent as isize);

    for ch in text.chars() {
        // If not in cache, rasterize and insert
        let char = get_character(ch, size, font);
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

            let bitmap_row_start =
                ((gy as f32 / stretch.1) as usize) * metrics.width;
            for gx in 0..w.floor() as usize {
                let px = pen_x as isize + gx as isize;
                if px < 0 || px >= buffer.width() as isize {
                    continue;
                }

                if bitmap[bitmap_row_start + (gy as f32 / stretch.0) as usize]
                    > 0
                {
                    if SAFE {
                        draw_pixel_safe(
                            buffer,
                            (px as usize, py as usize),
                            color,
                        );
                    } else {
                        draw_pixel_unsafe(
                            buffer,
                            (px as usize, py as usize),
                            color,
                        );
                    }
                }
            }
        }
        pen_x += advance_x;
    }
}
