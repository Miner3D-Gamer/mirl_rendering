// use crate::graphics::interpolate_color_rgb_u32_f32;
use mirl_buffer::traits::*;
#[allow(unused_imports)]
use mirl_extensions::*;

use super::{draw_pixel_safe, draw_pixel_unsafe};
#[inline]
/// Draw a simple line using Bresenham's line algorithm and destroy its performance by drawing circle at every point
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
pub fn draw_line<const SAFE: bool>(
    buffer: &mut (impl BufferPointers + BufferMetrics),
    start: (usize, usize),
    end: (usize, usize),
    color: u32,
    thickness: isize,
) {
    let mut start_x = start.0 as i16;
    let mut start_y = start.1 as i16;
    let end_x = end.0 as i16;
    let end_y = end.1 as i16;
    let difference_x: i16 = end_x - start_x;
    let difference_y: i16 = end_y - start_y;
    let sign_x = difference_x.sign();
    let sign_y = difference_y.sign();
    let abs_difference_x: i16 = difference_x.abs();
    let abs_difference_y: i16 = difference_y.abs();

    let radius = thickness as i16 / 2;
    let radius_sq = radius * radius;

    if abs_difference_x > abs_difference_y {
        let mut error: i16 = abs_difference_x / 2;
        while start_x != end_x {
            start_x += sign_x;
            error -= abs_difference_y;
            if error < 0 {
                start_y += sign_y;
                error += abs_difference_x;
            }
            // Draw a circle at each point
            for dy in -radius..=radius {
                for dx in -radius..=radius {
                    if dx * dx + dy * dy <= radius_sq {
                        let new_x = start_x + dx;
                        let new_y = start_y + dy;
                        if SAFE {
                            draw_pixel_safe(buffer, (new_x as usize, new_y as usize), color);
                        } else {
                            draw_pixel_unsafe(buffer, (new_x as usize, new_y as usize), color);
                        }
                    }
                }
            }
        }
    } else {
        let mut error: i16 = abs_difference_y / 2;
        while start_y != end_y {
            start_y += sign_y;
            error -= abs_difference_x;
            if error < 0 {
                start_x += sign_x;
                error += abs_difference_y;
            }
            // Draw a circle at each point
            for dy in -radius..=radius {
                for dx in -radius..=radius {
                    if dx * dx + dy * dy <= radius_sq {
                        let new_x = start_x + dx;
                        let new_y = start_y + dy;
                        if SAFE {
                            draw_pixel_safe(buffer, (new_x as usize, new_y as usize), color);
                        } else {
                            draw_pixel_unsafe(buffer, (new_x as usize, new_y as usize), color);
                        }
                    }
                }
            }
        }
    }
}
#[inline]
/// Draw a straight line
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
pub fn draw_line_straight<const SAFE: bool>(
    buffer: &mut (impl BufferPointers + BufferMetrics),
    start: (usize, usize),
    length: usize,
    vertical: bool,
    color: u32,
    thickness: isize,
) {
    let (start_x, start_y) = (start.0 as isize, start.1 as isize);
    let half_thickness = thickness / 2;

    if vertical {
        for y in start_y..start_y + length as isize {
            for offset_x in -half_thickness..=half_thickness {
                if SAFE {
                    draw_pixel_safe(buffer, ((start_x + offset_x) as usize, y as usize), color);
                } else {
                    draw_pixel_unsafe(buffer, ((start_x + offset_x) as usize, y as usize), color);
                }
            }
        }
    } else {
        for x in start_x..start_x + length as isize {
            for offset_y in -half_thickness..=half_thickness {
                if SAFE {
                    draw_pixel_safe(buffer, (x as usize, (start_y + offset_y) as usize), color);
                } else {
                    draw_pixel_unsafe(buffer, (x as usize, (start_y + offset_y) as usize), color);
                }
            }
        }
    }
}

// #[inline]
// /// Draw a simple antialiased line using Bresenham's line algorithm with basic antialiasing
// #[allow(clippy::cast_sign_loss)]
// #[allow(clippy::cast_precision_loss)]
// #[allow(clippy::cast_possible_truncation)]
// #[allow(clippy::cast_possible_wrap)]
// pub fn draw_line_antialiased(
//     buffer: &Buffer,
//     start: (usize, usize),
//     end: (usize, usize),
//     color: u32,
//     thickness: isize,
//     safe: bool,
// ) {
//     let draw_pixel: DrawPixelFunction = {
//         if safe {
//             draw_pixel_safe
//         } else {
//             draw_pixel_unsafe
//         }
//     };

//     let start_x = start.0 as f32;
//     let start_y = start.1 as f32;
//     let end_x = end.0 as f32;
//     let end_y = end.1 as f32;

//     let difference_x = end_x - start_x;
//     let difference_y = end_y - start_y;

//     let line_length =
//         difference_x.hypot(difference_y);
//     let steps = line_length.ceil() as usize;

//     if steps == 0 {
//         return;
//     }

//     let step_x = difference_x / steps as f32;
//     let step_y = difference_y / steps as f32;

//     // Calculate perpendicular vector for thickness
//     let p_x = -difference_y / line_length;
//     let p_y = difference_x / line_length;

//     for i in 0..=steps {
//         let x = step_x.mul_add(i as f32, start_x);
//         let y = step_y.mul_add(i as f32, start_y);

//         // Draw thickness around the main line
//         for t in
//             -(thickness as f32 / 2.0) as i16..=(thickness as f32 / 2.0) as i16
//         {
//             let thick_x = p_x.mul_add(t as f32, x);
//             let thick_y = p_y.mul_add(t as f32, y);

//             // Get the integer pixel coordinates
//             let pixel_x = thick_x as i16;
//             let pixel_y = thick_y as i16;

//             // Calculate fractional parts for antialiasing
//             let fract_x = thick_x - pixel_x as f32;
//             let fract_y = thick_y - pixel_y as f32;

//             // Draw the main pixel and its neighbors with appropriate alpha blending
//             let pixels_to_draw = [
//                 (pixel_x, pixel_y, (1.0 - fract_x) * (1.0 - fract_y)),
//                 (pixel_x + 1, pixel_y, fract_x * (1.0 - fract_y)),
//                 (pixel_x, pixel_y + 1, (1.0 - fract_x) * fract_y),
//                 (pixel_x + 1, pixel_y + 1, fract_x * fract_y),
//             ];

//             for (px, py, alpha) in pixels_to_draw {
//                 if px >= 0 && py >= 0 && alpha > 0.01 {
//                     let pixel_coord = (px as usize, py as usize);

//                     // Get the existing pixel color
//                     let existing_color = buffer.get_pixel(pixel_coord);

//                     // Interpolate between existing color and new color based on alpha
//                     let blended_color = interpolate_color_rgb_u32_f32(
//                         existing_color,
//                         color,
//                         alpha,
//                     );

//                     draw_pixel(
//                         buffer,
//                         pixel_coord.0,
//                         pixel_coord.1,
//                         blended_color,
//                     );
//                 }
//             }
//         }
//     }
// }
