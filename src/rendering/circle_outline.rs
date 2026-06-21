use mirl_buffer::traits::*;

use super::{draw_pixel_safe, draw_pixel_unsafe};

#[inline]
#[allow(clippy::cast_sign_loss)]
/// Draws a circle outline using the Midpoint Circle Algorithm
pub fn draw_circle_outline<const SAFE: bool>(
    buffer: &mut (impl BufferPointers + BufferMetrics),
    pos_x: usize,
    pos_y: usize,
    radius: usize,
    color: u32,
) {
    let mut x = 0;
    let mut y = radius as isize;
    let mut p = 1 - y;

    while x <= y {
        let tx = x as usize;
        let ty = y as usize;
        if SAFE {
            draw_pixel_safe(buffer, (pos_x + tx, pos_y + ty), color);
            draw_pixel_safe(buffer, (pos_x - tx, pos_y + ty), color);
            draw_pixel_safe(buffer, (pos_x + tx, pos_y - ty), color);
            draw_pixel_safe(buffer, (pos_x - tx, pos_y - ty), color);
            draw_pixel_safe(buffer, (pos_x + ty, pos_y + tx), color);
            draw_pixel_safe(buffer, (pos_x + ty, pos_y - tx), color);
            draw_pixel_safe(buffer, (pos_x - ty, pos_y + tx), color);
            draw_pixel_safe(buffer, (pos_x - ty, pos_y - tx), color);
        } else {
            draw_pixel_unsafe(buffer, (pos_x + tx, pos_y + ty), color);
            draw_pixel_unsafe(buffer, (pos_x - tx, pos_y + ty), color);
            draw_pixel_unsafe(buffer, (pos_x + tx, pos_y - ty), color);
            draw_pixel_unsafe(buffer, (pos_x - tx, pos_y - ty), color);
            draw_pixel_unsafe(buffer, (pos_x + ty, pos_y + tx), color);
            draw_pixel_unsafe(buffer, (pos_x + ty, pos_y - tx), color);
            draw_pixel_unsafe(buffer, (pos_x - ty, pos_y + tx), color);
            draw_pixel_unsafe(buffer, (pos_x - ty, pos_y - tx), color);
        }

        x += 1;
        if p < 0 {
            p += 2 * x + 1;
        } else {
            y -= 1;
            p += 2 * (x - y) + 1;
        }
    }
}
/// Draw a circle but instead of drawing it with a width of 1 you can define your own width
pub fn draw_circle_outline_with_thickness<const SAFE: bool>(
    buffer: &mut (impl BufferPointers + BufferMetrics),
    pos_x: usize,
    pos_y: usize,
    radius: usize,
    color: u32,
    width: usize,
) {
    let mut x = 0;
    let mut y = radius as isize;
    let mut p = 1 - y;

    while x <= y {
        for dx in -(width as isize) / 2..=(width as isize) / 2 {
            for dy in -(width as isize) / 2..=(width as isize) / 2 {
                if SAFE {
                    draw_pixel_safe(
                        buffer,
                        (
                            (pos_x as isize + x + dx) as usize,
                            (pos_y as isize + y + dy) as usize,
                        ),
                        color,
                    );
                    draw_pixel_safe(
                        buffer,
                        (
                            (pos_x as isize - x + dx) as usize,
                            (pos_y as isize + y + dy) as usize,
                        ),
                        color,
                    );
                    draw_pixel_safe(
                        buffer,
                        (
                            (pos_x as isize + x + dx) as usize,
                            (pos_y as isize - y + dy) as usize,
                        ),
                        color,
                    );
                    draw_pixel_safe(
                        buffer,
                        (
                            (pos_x as isize - x + dx) as usize,
                            (pos_y as isize - y + dy) as usize,
                        ),
                        color,
                    );

                    draw_pixel_safe(
                        buffer,
                        (
                            (pos_x as isize + y + dx) as usize,
                            (pos_y as isize + x + dy) as usize,
                        ),
                        color,
                    );
                    draw_pixel_safe(
                        buffer,
                        (
                            (pos_x as isize + y + dx) as usize,
                            (pos_y as isize - x + dy) as usize,
                        ),
                        color,
                    );
                    draw_pixel_safe(
                        buffer,
                        (
                            (pos_x as isize - y + dx) as usize,
                            (pos_y as isize + x + dy) as usize,
                        ),
                        color,
                    );
                    draw_pixel_safe(
                        buffer,
                        (
                            (pos_x as isize - y + dx) as usize,
                            (pos_y as isize - x + dy) as usize,
                        ),
                        color,
                    );
                } else {
                    draw_pixel_unsafe(
                        buffer,
                        (
                            (pos_x as isize + x + dx) as usize,
                            (pos_y as isize + y + dy) as usize,
                        ),
                        color,
                    );
                    draw_pixel_unsafe(
                        buffer,
                        (
                            (pos_x as isize - x + dx) as usize,
                            (pos_y as isize + y + dy) as usize,
                        ),
                        color,
                    );
                    draw_pixel_unsafe(
                        buffer,
                        (
                            (pos_x as isize + x + dx) as usize,
                            (pos_y as isize - y + dy) as usize,
                        ),
                        color,
                    );
                    draw_pixel_unsafe(
                        buffer,
                        (
                            (pos_x as isize - x + dx) as usize,
                            (pos_y as isize - y + dy) as usize,
                        ),
                        color,
                    );

                    draw_pixel_unsafe(
                        buffer,
                        (
                            (pos_x as isize + y + dx) as usize,
                            (pos_y as isize + x + dy) as usize,
                        ),
                        color,
                    );
                    draw_pixel_unsafe(
                        buffer,
                        (
                            (pos_x as isize + y + dx) as usize,
                            (pos_y as isize - x + dy) as usize,
                        ),
                        color,
                    );
                    draw_pixel_unsafe(
                        buffer,
                        (
                            (pos_x as isize - y + dx) as usize,
                            (pos_y as isize + x + dy) as usize,
                        ),
                        color,
                    );
                    draw_pixel_unsafe(
                        buffer,
                        (
                            (pos_x as isize - y + dx) as usize,
                            (pos_y as isize - x + dy) as usize,
                        ),
                        color,
                    );
                }
            }
        }

        x += 1;
        if p < 0 {
            p += 2 * x + 1;
        } else {
            y -= 1;
            p += 2 * (x - y) + 1;
        }
    }
}
