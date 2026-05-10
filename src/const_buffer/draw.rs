// use super::ConstBuffer;
// impl<const WIDTH: usize, const HEIGHT: usize> ConstBuffer<WIDTH, HEIGHT>
// where
//     [(); WIDTH * HEIGHT]:,
// {
//     #[allow(clippy::cast_precision_loss)]
//     /// A helper function to be used inside a `execute_at` render function
//     ///
//     /// Inverts the color at the given coordinates
//     pub const fn invert_color_below<const SAFE: bool>(
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
//         let inverted = crate::graphics::invert_color(old);
//         let new = crate::graphics::interpolate_color_rgb_u32_f32(
//             inverted,
//             old,
//             crate::graphics::get_alpha_of_u32(color) as f32 / 255.0,
//         );
//         unsafe {
//             *self.data.as_mut_ptr().add(xy.1 * WIDTH + xy.0) = new;
//         }
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

//             unsafe {
//                 *self.data.as_mut_ptr().add(xy.1 * WIDTH + xy.0) = inverted;
//             }
//         }
//         unsafe {
//             *self.data.as_mut_ptr().add(xy.1 * WIDTH + xy.0) = color;
//         }
//     }
// }
