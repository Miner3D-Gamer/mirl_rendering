use mirl_buffer::prelude::*;
use mirl_buffer_interpolation::InterpolationMode;
use mirl_extensions::{
    Interpolate0To1AsInterpolator, InterpolateAsInterpolator, IntoPatch, TryIntoPatch,
};
use mirl_graphics::{
    misc::advance_color,
    // prelude::*,
    u32_color_casting::{ColorManipulation, ColorManipulationWithoutInput},
};

// /// Create a collision -> [Pos2<Rectangle>>](crate::math::geometry::Pos2D) / [Rectangle](crate::math::geometry::d2::Rectangle) from the buffer metrics -> Automatically implemented for structs that implement [`BufferMetrics`]
// pub const trait BufferCollision {
//     // /// Create a collision instance for the current buffer
//     // fn to_rectangle<const CS: bool, T: Copy>(
//     //     &self,
//     // ) -> mirl_geometry::geometry::d2::Rectangle<T, CS>
//     // where
//     //     (usize, usize): [const] IntoPatch<(T, T)>;
//     // /// Create a collision instance for the current buffer
//     // fn try_to_rectangle<
//     //     const CS: bool,
//     //     T: core::ops::Add<Output = T>
//     //         + PartialOrd
//     //         + Copy
//     //         + core::ops::Div<Output = T>,
//     // >(
//     //     &self,
//     // ) -> Option<mirl_geometry::geometry::d2::Rectangle<T, CS>>
//     // where
//     //     usize: [const] TryIntoPatch<T>;
//     #[must_use]
//     /// Create a collision instance for the current buffer
//     fn to_collision<const CS: bool, T: Copy>(
//         &self,
//         pos: (T, T),
//     ) -> mirl_geometry::geometry::d2::Rectangle<T, CS>
//     where
//         usize: [const] IntoPatch<T>;
//     /// Create a collision instance for the current buffer
//     #[must_use]
//     #[allow(clippy::cast_possible_wrap)]
//     fn try_to_collision<
//         const CS: bool,
//         T: core::ops::Add<Output = T>
//             + PartialOrd
//             + Copy
//             + core::ops::Div<Output = T>,
//     >(
//         &self,
//         pos: (T, T),
//     ) -> Option<mirl_geometry::geometry::d2::Rectangle<T, CS>>
//     where
//         usize: [const] TryIntoPatch<T>;
// }

// impl<B: [const] BufferMetrics> const BufferCollision for B {
//     // /// Create a collision instance for the current buffer
//     // default fn to_rectangle<const CS: bool, T: Copy>(
//     //     &self,
//     // ) -> mirl_geometry::geometry::d2::Rectangle<T, CS>
//     // where
//     //     (usize, usize): [const] IntoPatch<(T, T)>,
//     // {
//     //     mirl_geometry::geometry::d2::Rectangle::new(self.total_size().into_value())
//     // }
//     // /// Create a collision instance for the current buffer using isize coordinates
//     // #[allow(clippy::cast_possible_wrap)]
//     // default fn try_to_rectangle<
//     //     const CS: bool,
//     //     T: core::ops::Add<Output = T>
//     //         + PartialOrd
//     //         + Copy
//     //         + core::ops::Div<Output = T>,
//     // >(
//     //     &self,
//     // ) -> Option<mirl_geometry::geometry::d2::Rectangle<T, CS>>
//     // where
//     //     usize: [const] TryIntoPatch<T>,
//     // {
//     //     Some(mirl_geometry::geometry::d2::Rectangle::new((
//     //         (self.width()).try_into_value()?,
//     //         (self.height()).try_into_value()?,
//     //     )))
//     // }
//     /// Create a collision instance for the current buffer
//     default fn to_collision<const CS: bool, T: Copy>(
//         &self,
//         pos: (T, T),
//     ) -> mirl_geometry::geometry::d2::Rectangle<T, CS>
//     where
//         usize: [const] IntoPatch<T>,
//     {
//         mirl_geometry::geometry::d2::Rectangle::new(
//             pos,
//             (self.width().into_value(), self.height().into_value()),
//         )
//     }
//     /// Create a collision instance for the current buffer using isize coordinates
//     #[allow(clippy::cast_possible_wrap)]
//     default fn try_to_collision<
//         const CS: bool,
//         T: core::ops::Add<Output = T>
//             + PartialOrd
//             + Copy
//             + core::ops::Div<Output = T>,
//     >(
//         &self,
//         pos: (T, T),
//     ) -> Option<mirl_geometry::geometry::d2::Rectangle<T, CS>>
//     where
//         usize: [const] TryIntoPatch<T>,
//     {
//         Some(mirl_geometry::geometry::d2::Rectangle::new(
//             pos,
//             (self.width().try_into_value()?, self.height().try_into_value()?),
//         ))
//     }
// }
/// Check if the given position is within the bounds of the buffer
pub const trait IsPixelPositionInBuffer {
    /// Safely check if the position is inside  the buffer
    fn is_pixel_position_in_buffer<T: [const] IntoPatch<usize> + [const] core::marker::Destruct>(
        &self,
        position: (T, T),
    ) -> bool;
    /// Safely check if the position is inside the buffer.
    /// Returning false when the value cannot be converted into usize (meaning it couldn't fit inside the buffer anyways)
    fn try_is_pixel_position_in_buffer<
        T: [const] TryIntoPatch<usize> + [const] core::marker::Destruct,
    >(
        &self,
        position: (T, T),
    ) -> bool;
}
const impl<S: [const] BufferMetrics> IsPixelPositionInBuffer for S {
    default fn is_pixel_position_in_buffer<
        T: [const] IntoPatch<usize> + [const] core::marker::Destruct,
    >(
        &self,
        position: (T, T),
    ) -> bool {
        position.0.into_value() < self.width() && position.1.into_value() < self.height()
    }
    default fn try_is_pixel_position_in_buffer<
        T: [const] TryIntoPatch<usize> + [const] core::marker::Destruct,
    >(
        &self,
        position: (T, T),
    ) -> bool {
        let Some(pos_x) = position.0.try_into_value() else {
            return false;
        };
        let Some(pos_y) = position.1.try_into_value() else {
            return false;
        };
        pos_x < self.width() && pos_y < self.height()
    }
}
/// Helper Functions that can be used inside the `execute_at`
pub const trait BufferExecuteAtFunctions {
    /// Inverts the color at the given coordinates
    fn invert_color_below<const SAFE: bool>(&mut self, xy: (usize, usize), color: u32);
    /// Inverts the color below if it matches the input number
    fn invert_color_if_same<const SAFE: bool>(&mut self, xy: (usize, usize), color: u32);
}
use mirl_extensions::InterpolateColorBetween;
impl<S: BufferGetPixel + BufferPointers + BufferMetrics> BufferExecuteAtFunctions for S {
    default fn invert_color_below<const SAFE: bool>(&mut self, xy: (usize, usize), color: u32) {
        let old = if SAFE {
            let Some(old) = self.get_pixel_option(xy) else {
                return;
            };
            old
        } else {
            unsafe { self.get_pixel_unchecked(xy) }
        };
        let inverted = old.invert_color();

        let new = inverted.interpolate_color_with(old, color.alpha() as f32 / 255.0);

        crate::draw_pixel_unsafe(self, xy, new);
    }
    /// A helper function to be used inside a `execute_at` render function
    ///
    /// Inverts the color below if it matches the input number
    default fn invert_color_if_same<const SAFE: bool>(&mut self, xy: (usize, usize), color: u32) {
        let old = if SAFE {
            let Some(old) = self.get_pixel_option(xy) else {
                return;
            };
            old
        } else {
            unsafe { self.get_pixel_unchecked(xy) }
        };
        if old == color {
            let inverted = old.invert_color();

            crate::draw_pixel_unsafe(self, xy, inverted);
        }
        crate::draw_pixel_unsafe(self, xy, color);
    }
}
/// Resized the contents of the Buffer
///
/// Warning: This trait will definitely be changed in some way
pub const trait ResizeBuffer {
    #[must_use]
    /// Creates a new buffer and copies the contents of the current buffer
    fn resize_content(&self, size: (usize, usize), resizing_method: InterpolationMode) -> Self;
}
#[cfg(feature = "std")]
impl ResizeBuffer for Buffer {
    default fn resize_content(
        &self,
        size: (usize, usize),
        resizing_method: InterpolationMode,
    ) -> Self {
        let mut new = Self::new_empty(size);
        let b = mirl_buffer_interpolation::resize_buffer(
            self,
            self.width,
            self.height,
            size.0,
            size.1,
            resizing_method,
        );
        new.data.copy_from_slice(&b);
        new
    }
}
/// Flip the buffer horizontally or vertically
pub const trait FlipBuffer {
    /// Flip the buffer vertically
    fn flip_vertically(&mut self);
    /// Flip the buffer horizontally
    fn flip_horizontally(&mut self);
}
impl FlipBuffer for Buffer {
    /// Flip the buffer vertically (top becomes bottom)
    default fn flip_vertically(&mut self) {
        let mut result = Self::new_empty((self.width, self.height));

        unsafe {
            for y in 0..self.height {
                let src_row = self.as_ptr().add(y * self.width);
                let dst_row = result
                    .data
                    .as_mut_ptr()
                    .add((self.height - 1 - y) * self.width);
                core::ptr::copy_nonoverlapping(src_row, dst_row, self.width);
            }
        }

        *self = result;
    }

    /// Flip the buffer horizontally (left becomes right)
    default fn flip_horizontally(&mut self) {
        let mut result = Self::new_empty((self.width, self.height));

        unsafe {
            for y in 0..self.height {
                for x in 0..self.width {
                    let dst_idx = y * self.width + (self.width - 1 - x);
                    *result.mut_pointer().add(dst_idx) = *self.pointer().add(y * self.width + x);
                }
            }
        }

        *self = result;
    }
}

/// Rotate the buffer by 90°, 180°, or 270° (-90°)
pub const trait RotateBuffer {
    /// Rotate the buffer 90°
    fn rotate_90(&mut self);
    /// Rotate the buffer 180°
    fn rotate_180(&mut self);
    /// Rotate the buffer 270°
    fn rotate_270(&mut self);
}

impl RotateBuffer for Buffer {
    /// Rotate the buffer 90°
    default fn rotate_90(&mut self) {
        let mut result = Self::new_empty((self.height, self.width));

        unsafe {
            for y in 0..self.height {
                for x in 0..self.width {
                    let src_pixel = *self.pointer().add(y * self.width + x);
                    // For 90° clockwise: new_x = old_y, new_y = width - 1 - old_x
                    let new_x = y;
                    let new_y = self.width - 1 - x;
                    let dst_idx = new_y * self.height + new_x;
                    *result.mut_pointer().add(dst_idx) = src_pixel;
                }
            }
        }

        *self = result;
    }
    /// Rotate the buffer 180°
    default fn rotate_180(&mut self) {
        let mut result = Self::new_empty((self.width, self.height));

        unsafe {
            for i in 0..self.total_size {
                let src_pixel = *self.pointer().add(i);
                let dst_idx = self.total_size - 1 - i;
                *result.mut_pointer().add(dst_idx) = src_pixel;
            }
        }

        *self = result;
    }
    /// Rotate the buffer 270° (or -90°)
    default fn rotate_270(&mut self) {
        let mut result = Self::new_empty((self.height, self.width));

        unsafe {
            for y in 0..self.height {
                for x in 0..self.width {
                    let src_pixel = *self.pointer().add(y * self.width + x);
                    let new_x = self.height - 1 - y;
                    let new_y = x;
                    let dst_idx = new_y * self.height + new_x;
                    *result.mut_pointer().add(dst_idx) = src_pixel;
                }
            }
        }

        *self = result;
    }
}
impl<const WIDTH: usize, const HEIGHT: usize> FlipBuffer for ConstBuffer<WIDTH, HEIGHT>
where
    [(); WIDTH * HEIGHT]:,
{
    /// Flip the `ConstBuffer` vertically (top becomes bottom)
    default fn flip_vertically(&mut self) {
        let mut result = Self::new_empty();

        unsafe {
            for y in 0..HEIGHT {
                let src_row = self.as_ptr().add(y * WIDTH);
                let dst_row = result.data.as_mut_ptr().add((HEIGHT - 1 - y) * WIDTH);
                core::ptr::copy_nonoverlapping(src_row, dst_row, WIDTH);
            }
        }

        *self = result;
    }
    /// Flip the `ConstBuffer` horizontally (left becomes right)
    default fn flip_horizontally(&mut self) {
        let mut result = Self::new_empty();

        unsafe {
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let dst_idx = y * WIDTH + (WIDTH - 1 - x);
                    *result.data.as_mut_ptr().add(dst_idx) = *self.data.as_ptr().add(y * WIDTH + x);
                }
            }
        }

        *self = result;
    }
}
/// Apply a function to every pixel of the buffer
pub const trait ApplyFilter {
    /// Apply a function to every pixel of the buffer
    fn apply_filter(
        &mut self,
        function: impl [const] Fn(u32) -> u32 + [const] core::marker::Destruct,
    );
}
const impl<S: [const] BufferData> ApplyFilter for S {
    default fn apply_filter(
        &mut self,
        function: impl [const] Fn(u32) -> u32 + [const] core::marker::Destruct,
    ) {
        let data = self.data_mut();
        let mut i = 0;
        while i < data.len() {
            data[i] = function(data[i]);
            i += 1;
        }
    }
}
/// Fade out the edges of the buffer by setting their alpha
pub const trait FadeOutEdges {
    /// A steepness of 15 and offset of 0.8 makes a nice looking icon (Rough estimates based on trail and error)
    fn fade_out_edges(&mut self, steepness: f32, offset: f32);
}
impl<S: BufferMetrics + BufferPointers> FadeOutEdges for S {
    default fn fade_out_edges(&mut self, steepness: f32, offset: f32) {
        let cx = self.width() as f32 / 2.0;
        let cy = self.height() as f32 / 2.0;
        let max_dist = cx.hypot(cy);

        let mut y = 0;
        while y < self.height() {
            let mut x = 0;
            while x < self.width() {
                let dx = x as f32 - cx;
                let dy = y as f32 - cy;
                let dist = dx.hypot(dy) / max_dist;
                let fade = 1.0 - dist;
                let fade = 1.0 - fade.interpolate_smooth_0_to_1(steepness, offset);
                unsafe {
                    let color = *self.pointer().add(y * self.width() + x);
                    *self.mut_pointer().add(y * self.width() + x) =
                        color.with_alpha(fade.interpolate_values(0_f32, 255_f32) as u32);
                };
                x += 1;
            }
            y += 1;
        }
    }
}
/// Clear the buffer to nothing or a solid color
pub const trait ClearBuffer {
    /// Replaces all data with zeros - very fast
    fn clear(&mut self);
    /// Replaces all data with a flat color
    fn clear_buffer_with_color(&mut self, color: u32);
}
impl<S: BufferPointers + BufferMetrics + BufferData> ClearBuffer for S {
    default fn clear(&mut self) {
        unsafe {
            core::ptr::write_bytes(self.mut_pointer(), 0, self.width() * self.height());
        }
    }
    default fn clear_buffer_with_color(&mut self, color: u32) {
        self.data_mut().fill(color);
    }
}
/// Fill all transparent pixels of the buffer with a color
pub const trait FillTransparent {
    /// Replaces all pixels with a alpha of 0 with this color
    fn fill_fully_transparent(&mut self, color: u32);
    /// Fill all pixels with an alpha of != 255 with this color
    fn fill_transparent(&mut self, color: u32);
}
impl<S: BufferData> FillTransparent for S {
    default fn fill_fully_transparent(&mut self, color: u32) {
        use std::simd::{Select, prelude::SimdPartialEq, u32x8};

        let (prefix, middle, suffix) = self.data_mut().as_simd_mut::<8>();

        let color_vec = u32x8::splat(color);
        let alpha_mask = u32x8::splat(0xFF00_0000);
        let zero = u32x8::splat(0);

        for chunk in middle {
            let alphas = *chunk & alpha_mask;
            let is_transp = alphas.simd_eq(zero);
            *chunk = is_transp.select(color_vec, *chunk);
        }

        // handle unaligned prefix/suffix scalarly
        for pixel in prefix.iter_mut().chain(suffix.iter_mut()) {
            if (*pixel & 0xFF00_0000) == 0 {
                *pixel = color;
            }
        }
    }
    default fn fill_transparent(&mut self, color: u32) {
        use std::simd::{Select, prelude::SimdPartialEq, u32x8};

        let (prefix, middle, suffix) = self.data_mut().as_simd_mut::<8>();

        let color_vec = u32x8::splat(color);
        let alpha_mask = u32x8::splat(0xFF00_0000);
        // let zero = u32x8::splat(0);
        for chunk in middle {
            let alphas = *chunk & alpha_mask;
            let is_not_opaque = alphas.simd_ne(alpha_mask);
            *chunk = is_not_opaque.select(color_vec, *chunk);
        }

        for pixel in prefix.iter_mut().chain(suffix.iter_mut()) {
            if (*pixel & 0xFF00_0000) != 0xFF00_0000 {
                *pixel = color;
            }
        }
    }
}

/// Optimizes the image by removing empty space around the image
pub const trait TrimBuffer: TrimBufferHelper {
    #[must_use]
    /// Checks if the requested row only has fully transparent pixels
    fn is_row_transparent(&self, row: usize) -> bool;
    #[must_use]
    /// Checks if the requested column only has fully transparent pixels
    fn is_col_transparent(&self, row: usize) -> bool;
    /// Trims the image by the given restrictions
    fn apply_trim(&mut self, top: usize, bottom: usize, left: usize, right: usize);
}
impl<S: BufferMetrics + BufferData + SetBufferMetrics> TrimBuffer for S {
    default fn is_row_transparent(&self, row: usize) -> bool {
        let start = row * self.width();
        let end = start + self.width();
        self.data()[start..end]
            .iter()
            .all(|&pixel| pixel.alpha() == 0)
    }
    default fn is_col_transparent(&self, col: usize) -> bool {
        (0..self.height()).all(|row| self.data()[row * self.width() + col].alpha() == 0)
    }
    /// Trims the image by the given restrictions
    default fn apply_trim(&mut self, top: usize, bottom: usize, left: usize, right: usize) {
        let new_width = self.width() - left - right;
        let new_height = self.height() - top - bottom;
        let mut new_data = Vec::with_capacity(new_width * new_height);

        for row in top..(self.height() - bottom) {
            let row_start = row * self.width() + left;
            let row_end = row_start + new_width;
            new_data.extend_from_slice(&self.data()[row_start..row_end]);
        }

        self.set_data(&new_data);
        self.set_width(new_width);
        self.set_height(new_height);
    }
}
/// Helper functions that are automatically implemented for buffers what implement [`TrimBuffer`]
pub const trait TrimBufferHelper {
    /// Optimizes the image by removing empty space around the image
    fn remove_margins(&mut self);
    #[must_use]
    /// Calculates the empty space around the image
    fn calculate_trims(&self) -> (usize, usize, usize, usize);
}

impl<S: BufferMetrics + TrimBuffer> TrimBufferHelper for S {
    default fn remove_margins(&mut self) {
        // Remove all margins in one pass to avoid multiple data copies
        let (top_trim, bottom_trim, left_trim, right_trim) = self.calculate_trims();

        if top_trim > 0 || bottom_trim > 0 || left_trim > 0 || right_trim > 0 {
            self.apply_trim(top_trim, bottom_trim, left_trim, right_trim);
        }
    }
    default fn calculate_trims(&self) -> (usize, usize, usize, usize) {
        let mut top_trim = 0;
        let mut bottom_trim = 0;
        let mut left_trim = 0;
        let mut right_trim = 0;

        // Calculate top trim
        for row in 0..self.height() {
            if self.is_row_transparent(row) {
                top_trim += 1;
            } else {
                break;
            }
        }

        // Calculate bottom trim
        for row in (0..self.height()).rev() {
            if self.is_row_transparent(row) {
                bottom_trim += 1;
            } else {
                break;
            }
        }

        // Calculate left trim
        for col in 0..self.width() {
            if self.is_col_transparent(col) {
                left_trim += 1;
            } else {
                break;
            }
        }

        // Calculate right trim
        for col in (0..self.width()).rev() {
            if self.is_col_transparent(col) {
                right_trim += 1;
            } else {
                break;
            }
        }

        (top_trim, bottom_trim, left_trim, right_trim)
    }
}
/// Try to find an unused color of a buffer
pub const trait GetUnusedColor {
    /// Try to find an unused color of a buffer, colors that are completely
    ///
    /// Warning, if the buffer contains all 16581375 (255**3) color/alpha combinations this may cause an infinite loop
    fn get_unused_color_of_buffer(&mut self, current_color: (u8, u8, u8)) -> (u8, u8, u8);
}
impl<S: BufferData> GetUnusedColor for S {
    default fn get_unused_color_of_buffer(&mut self, current_color: (u8, u8, u8)) -> (u8, u8, u8) {
        let mut current_color = current_color;
        let mut unique_colors = std::collections::HashSet::new();
        for color in self.data() {
            if color.alpha() != 0 {
                unique_colors.insert((color.red() as u8, color.green() as u8, color.blue() as u8));
            }
        }
        while unique_colors.contains(&current_color) {
            current_color = advance_color(current_color.0, current_color.1, current_color.2);
        }
        current_color
    }
}

// BufferMetrics
// BufferPointers
// BufferData
// BufferSetPixel
// BufferGetPixel
// fn test() {
//     let hi: f32 = 100u32.into_value();
//     let hi: &dyn mirl_extensions::InterpolateAsInterpolator<u32, u32, f32> = &100f32;
// }
