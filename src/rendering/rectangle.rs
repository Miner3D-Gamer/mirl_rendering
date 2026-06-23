#[cfg(feature = "std")]
use mirl_buffer::Buffer;
use mirl_buffer::traits::*;

use super::{draw_pixel_safe, draw_pixel_unsafe};

/// Draw a simple rectangle
#[inline]
#[allow(clippy::cast_sign_loss)]
pub fn draw_rectangle_old<const SAFE: bool>(
    buffer: &mut (impl BufferPointers + BufferMetrics),
    pos: (isize, isize),
    size: (isize, isize),
    color: u32,
) {
    for y in pos.1..pos.1 + size.1 {
        for x in pos.0..pos.0 + size.0 {
            if SAFE {
                if x > 0 && y > 0 {
                    draw_pixel_safe(buffer, (x as usize, y as usize), color);
                }
            } else {
                draw_pixel_unsafe(buffer, (x as usize, y as usize), color);
            }
        }
    }
}

#[inline]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_possible_wrap)]
/// Draw a simple rectangle
pub fn draw_rectangle<const SAFE: bool>(
    buffer: &mut (impl BufferPointers + BufferMetrics),
    pos: (isize, isize),
    size: (isize, isize),
    color: u32,
) {
    if size.0 <= 0 || size.1 <= 0 {
        return;
    }

    if SAFE {
        // Calculate actual drawable bounds
        let start_x = pos.0.max(0) as usize;
        let start_y = pos.1.max(0) as usize;
        let end_x =
            ((pos.0 + size.0).min(buffer.width() as isize)).max(0) as usize;
        let end_y =
            ((pos.1 + size.1).min(buffer.height() as isize)).max(0) as usize;

        if start_x >= buffer.width()
            || start_y >= buffer.height()
            || start_x >= end_x
            || start_y >= end_y
        {
            return;
        }

        let row_width = end_x - start_x;
        //println!("end {} - start {}", end_x, start_x);

        for y in start_y..end_y {
            let row_start = y * buffer.width() + start_x;
            unsafe {
                let ptr = buffer.mut_pointer().add(row_start);
                for i in 0..row_width {
                    //println!("Before {row_width}, {start_x}");
                    *ptr.add(i) = color;
                    //println!("After");
                }
            }
        }
    } else {
        let start_x = pos.0 as usize;
        let start_y = pos.1 as usize;
        let end_y = start_y + size.1 as usize;
        let row_width = size.0 as usize;

        for y in start_y..end_y {
            let row_start = y * buffer.width() + start_x;
            unsafe {
                let ptr = buffer.mut_pointer().add(row_start);
                if row_width <= 8 {
                    // Manual unrolling for small rectangles
                    match row_width {
                        1 => *ptr = color,
                        2 => {
                            *ptr = color;
                            *ptr.add(1) = color;
                        }
                        3 => {
                            *ptr = color;
                            *ptr.add(1) = color;
                            *ptr.add(2) = color;
                        }
                        4 => {
                            *ptr = color;
                            *ptr.add(1) = color;
                            *ptr.add(2) = color;
                            *ptr.add(3) = color;
                        }
                        _ => {
                            for i in 0..row_width {
                                *ptr.add(i) = color;
                            }
                        }
                    }
                } else {
                    for i in 0..row_width {
                        *ptr.add(i) = color;
                    }
                }
            }
        }
    }
}
#[cfg(feature = "std")]
/// Draw a simple rectangle
#[inline]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
pub fn execute_at_rectangle<const SAFE: bool>(
    buffer: &mut Buffer,
    pos: (isize, isize),
    size: (isize, isize),
    color: u32,
    function: impl Fn(&mut Buffer, (usize, usize), u32),
) {
    for y in pos.1..pos.1 + size.1 {
        for x in pos.0..pos.0 + size.0 {
            if SAFE {
                if x > 0 && y > 0 {
                    function(buffer, (x as usize, y as usize), color);
                }
            } else {
                function(buffer, (x as usize, y as usize), color);
            }
        }
    }
}

/// Draw a rotated rectangle
#[inline]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
pub fn draw_rectangle_angled<const SAFE: bool>(
    buffer: &mut (impl BufferPointers + BufferMetrics),
    pos: (usize, usize),
    size: (isize, isize),
    color: u32,
    anchor_pos: (usize, usize),
    rotation: f32,
) {
    let cos_theta = rotation.cos();
    let sin_theta = rotation.sin();

    let anchor_x_offset = anchor_pos.0 as f32 * size.0 as f32;
    let anchor_y_offset = anchor_pos.1 as f32 * size.1 as f32;

    for y in 0..size.1 as usize {
        for x in 0..size.0 as usize {
            let rel_x = x as f32 - anchor_x_offset;
            let rel_y = y as f32 - anchor_y_offset;

            let rotated_x = rel_x.mul_add(cos_theta, -(rel_y * sin_theta));
            let rotated_y = rel_x.mul_add(sin_theta, rel_y * cos_theta);

            let final_x =
                (pos.0 as f32 + rotated_x + anchor_x_offset).round() as isize;
            let final_y =
                (pos.1 as f32 + rotated_y + anchor_y_offset).round() as isize;

            if SAFE {
                if final_x >= 0 && final_y >= 0 {
                    draw_pixel_safe(
                        buffer,
                        (final_x as usize, final_y as usize),
                        color,
                    );
                }
            } else {
                draw_pixel_unsafe(
                    buffer,
                    (final_x as usize, final_y as usize),
                    color,
                );
            }
        }
    }
}
#[cfg(feature = "std")]
/// Tried to draw a rectangle 4 pixels at the time in the hopes of improving performance
#[inline]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
pub fn draw_rectangle_impl_simd(
    buffer: &mut Buffer,
    pos: (isize, isize),
    size: (isize, isize),
    color: u32,
    safe: bool,
) {
    if safe
        && (pos.0 < 0
            || pos.1 < 0
            || pos.0 + size.0 > buffer.width as isize
            || pos.1 + size.1 > buffer.height as isize)
    {
        return;
    }

    let start_x = pos.0 as usize;
    let start_y = pos.1 as usize;
    let rect_width = size.0 as usize;
    let rect_height = size.1 as usize;

    for y in 0..rect_height {
        let row_start = (start_y + y) * buffer.width + start_x;
        unsafe {
            let mut ptr = buffer.mut_pointer().add(row_start);
            let mut remaining = rect_width;

            // Process 4 pixels at a time
            while remaining >= 4 {
                *ptr = color;
                *ptr.add(1) = color;
                *ptr.add(2) = color;
                *ptr.add(3) = color;
                ptr = ptr.add(4);
                remaining -= 4;
            }

            // Handle remaining pixels
            while remaining > 0 {
                *ptr = color;
                ptr = ptr.add(1);
                remaining -= 1;
            }
        }
    }
}
