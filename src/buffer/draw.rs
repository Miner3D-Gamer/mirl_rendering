// use mirl::Buffer;
// use mirl_extensions::InterpolateColorBetween;

// use crate::BufferGetPixel;

// impl Buffer {
//     #[allow(clippy::cast_precision_loss)]
//     /// A helper function to be used inside a `execute_at` render function
//     ///
//     /// Inverts the color at the given coordinates
//     pub fn invert_color_below<const SAFE: bool>(
//         &mut self,
//         xy: (usize, usize),
//         color: u32,
//     ) {
//         let old = if SAFE {
//             let Some(old) = self.get_pixel_option(xy) else {
//                 return;
//             };
//             old
//         } else {
//             self.get_pixel_unchecked(xy)
//         };
//         let inverted = mirl_core::graphics::invert_color(old);

//         let new = inverted.interpolate_color_with(
//             old,
//             mirl_core::graphics::get_alpha_of_u32(color) as f32 / 255.0,
//         );

//         crate::draw_pixel_unsafe(self, xy, new);
//     }
//     /// A helper function to be used inside a `execute_at` render function
//     ///
//     /// Inverts the color below if it matches the input number
//     pub const fn invert_color_if_same<const SAFE: bool>(
//         &mut self,
//         xy: (usize, usize),
//         color: u32,
//     ) {
//         let old = if SAFE {
//             let Some(old) = self.get_pixel_option(xy) else {
//                 return;
//             };
//             old
//         } else {
//             self.get_pixel_unchecked(xy)
//         };
//         if old == color {
//             let inverted = crate::graphics::invert_color(old);

//             crate::draw_pixel_unsafe(self, xy, inverted);
//         }
//         crate::draw_pixel_unsafe(self, xy, color);
//     }
// }
