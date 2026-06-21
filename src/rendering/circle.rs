use mirl_buffer::traits::*;
use mirl_extensions::*;

use super::{draw_pixel_safe, draw_pixel_unsafe};

/// Draw a filled circle by checking if every pixel is inside the radius
#[inline]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_possible_wrap)]
pub fn draw_circle<const SAFE: bool, const FIX_STRAY_PIXEL: bool>(
    buffer: &mut (impl BufferPointers + BufferMetrics),
    pos: (isize, isize),
    radius: isize,
    color: u32,
) {
    for y in -radius..=radius {
        let dy = y * y;
        let mut dx = (radius * radius - dy).abs().sqrt();
        if FIX_STRAY_PIXEL && (y == 0 || y == radius || y == -radius) {
            dx -= 1;
        }

        for x in -dx..=dx {
            let x_pos = pos.0 + x;
            let y_pos = pos.1 + y;

            if x_pos >= 0 && y_pos >= 0 {
                if SAFE {
                    draw_pixel_safe(
                        buffer,
                        (x_pos as usize, y_pos as usize),
                        color,
                    );
                } else {
                    draw_pixel_unsafe(
                        buffer,
                        (x_pos as usize, y_pos as usize),
                        color,
                    );
                }
            }
        }
    }
}
