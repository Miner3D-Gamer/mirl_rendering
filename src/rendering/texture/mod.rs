use mirl_buffer::prelude::*;
#[cfg(feature = "std")]
use mirl_buffer_interpolation::InterpolationMode;
use mirl_extensions::InterpolateColorBetween;
use mirl_graphics::u32_color_casting::ColorManipulationWithoutInput;

#[cfg(feature = "std")]
use crate::prelude::ResizeBuffer;
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
#[cfg(feature = "std")]
/// Draw a Buffer on screen while stretching it to the desired size
pub fn draw_buffer_on_buffer_stretched<
    const SAFE: bool,
    const TRANSPARENCY: bool,
    const TRANSPARENCY_INTERPOLATION: bool,
    const NICHE_TRANSPARENCY_CHECK: bool,
>(
    canvas: &mut Buffer,
    texture: &Buffer,
    position: (isize, isize),
    result_dimensions: (usize, usize),
    interpolation_mode: InterpolationMode,
) {
    draw_buffer_on_buffer::<SAFE, TRANSPARENCY, TRANSPARENCY_INTERPOLATION, NICHE_TRANSPARENCY_CHECK>(
        canvas,
        &texture.resize_content(result_dimensions, interpolation_mode),
        position,
    );
}

// #[cfg(feature = "std")]
// /// `Smartly` draw a buffer on another buffer by first checking the visibility of drawn buffer and as such eliminating the manual `::<SAFE>`
// ///
// /// Does not resize when requested dimensions and actual size are equivalent
// pub fn draw_buffer_on_buffer_stretched_smart<
//     const TRANSPARENCY: bool,
//     const TRANSPARENCY_INTERPOLATION: bool,
//     const NICHE_TRANSPARENCY_CHECK: bool,
// >(
//     canvas: &mut Buffer,
//     texture: &Buffer,
//     position: (isize, isize),
//     result_dimensions: (usize, usize),
//     interpolation_mode: InterpolationMode,
// ) {
//     if core::intrinsics::unlikely(result_dimensions == texture.get_size()) {
//         draw_buffer_on_buffer_smart::<
//             TRANSPARENCY,
//             TRANSPARENCY_INTERPOLATION,
//             NICHE_TRANSPARENCY_CHECK,
//         >(canvas, texture, position);
//     } else {
//         draw_buffer_on_buffer_smart::<
//             TRANSPARENCY,
//             TRANSPARENCY_INTERPOLATION,
//             NICHE_TRANSPARENCY_CHECK,
//         >(
//             canvas,
//             &texture.resize_content(result_dimensions, interpolation_mode),
//             position,
//         );
//     }
// }
// /// `Smartly` draw a buffer on another buffer by first checking the visibility of drawn buffer and as such eliminating the manual `::<SAFE>`
// pub fn draw_buffer_on_buffer_smart<
//     const TRANSPARENCY: bool,
//     const TRANSPARENCY_INTERPOLATION: bool,
//     const NICHE_TRANSPARENCY_CHECK: bool,
// >(
//     canvas: &mut (
//              impl BufferMetrics + BufferGetPixel + BufferSetPixel + BufferCollision
//          ),
//     texture: &(impl BufferMetrics + BufferGetPixel),
//     position: (isize, isize),
// ) {
//     let Some(collision_big) = canvas.try_to_collision::<false, isize>(position)
//     else {
//         return;
//     };
//     let Some(collision_small) =
//         texture.try_to_collision::<false, isize>(position)
//     else {
//         return;
//     };
//     if core::intrinsics::likely(
//         collision_big.do_areas_intersect_strict(&collision_small),
//     ) {
//         if collision_big.does_area_fully_include_other_area(&collision_small) {
//             draw_buffer_on_buffer::<
//                 false,
//                 TRANSPARENCY,
//                 TRANSPARENCY_INTERPOLATION,
//                 NICHE_TRANSPARENCY_CHECK,
//             >(canvas, texture, position);
//         } else {
//             draw_buffer_on_buffer::<
//                 true,
//                 TRANSPARENCY,
//                 TRANSPARENCY_INTERPOLATION,
//                 NICHE_TRANSPARENCY_CHECK,
//             >(canvas, texture, position);
//         }
//     }
// }

/// Draw a buffer on another buffer 1 to 1 instead of scaling it
#[allow(clippy::cast_possible_wrap)]
#[allow(clippy::cast_sign_loss)]
pub fn draw_buffer_on_buffer<
    const SAFE: bool,
    const TRANSPARENCY: bool,
    const TRANSPARENCY_INTERPOLATION: bool,
    const NICHE_TRANSPARENCY_CHECK: bool,
>(
    canvas: &mut (impl BufferMetrics + BufferGetPixel + BufferSetPixel),
    texture: &(impl BufferMetrics + BufferGetPixel),
    position: (isize, isize),
) {
    // Calculate the visible rectangle in canvas coordinates
    let canvas_start_x = if SAFE { position.0.max(0) } else { position.0 };
    let canvas_start_y = if SAFE { position.1.max(0) } else { position.1 };
    let canvas_end_x = if SAFE {
        (position.0 + texture.width() as isize).min(canvas.width() as isize)
    } else {
        position.0 + texture.width() as isize
    };
    let canvas_end_y = if SAFE {
        (position.1 + texture.height() as isize).min(canvas.height() as isize)
    } else {
        position.1 + texture.height() as isize
    };

    // Early exit if no visible area
    if SAFE && (canvas_start_x >= canvas_end_x || canvas_start_y >= canvas_end_y) {
        return;
    }

    // Calculate texture offsets for the visible area
    let texture_start_x = (canvas_start_x - position.0) as usize;
    let texture_start_y = (canvas_start_y - position.1) as usize;

    // Iterate only over the visible area
    for canvas_y in canvas_start_y..canvas_end_y {
        for canvas_x in canvas_start_x..canvas_end_x {
            let texture_x = texture_start_x + (canvas_x - canvas_start_x) as usize;
            let texture_y = texture_start_y + (canvas_y - canvas_start_y) as usize;

            let pixel = unsafe { texture.get_pixel_unchecked((texture_x, texture_y)) };

            if TRANSPARENCY {
                let trans = pixel.alpha();
                if NICHE_TRANSPARENCY_CHECK && trans == 0 {
                    continue;
                }

                let color = if TRANSPARENCY_INTERPOLATION {
                    if NICHE_TRANSPARENCY_CHECK && trans == 255 {
                        pixel
                    } else if NICHE_TRANSPARENCY_CHECK && trans == 0 {
                        continue;
                    } else {
                        unsafe {
                            canvas
                                .get_pixel_unchecked((canvas_x as usize, canvas_y as usize))
                                .interpolate_color_with(pixel, trans as f32 / 255.0)
                        }
                    }
                } else if trans != 0 {
                    pixel
                } else {
                    unsafe { canvas.get_pixel_unchecked((canvas_x as usize, canvas_y as usize)) }
                };
                unsafe {
                    canvas.set_pixel_unchecked((canvas_x as usize, canvas_y as usize), color);
                }
            } else {
                unsafe {
                    canvas.set_pixel_unchecked((canvas_x as usize, canvas_y as usize), pixel);
                }
            }
        }
    }
}
