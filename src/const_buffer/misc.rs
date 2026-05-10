// use super::ConstBuffer;
// #[cfg(not(feature = "std"))]
// #[allow(unused_imports)]
// use crate::extensions::*;
// use crate::graphics::ColorManipulation;

// impl<const WIDTH: usize, const HEIGHT: usize> ConstBuffer<WIDTH, HEIGHT>
// where
//     [(); WIDTH * HEIGHT]:,
// {
//     /// Apply a simple filter to every pixel of the buffer
//     pub fn apply_filter(&mut self, function: impl Fn(u32) -> u32) {
//         for x in 0..WIDTH {
//             for y in 0..HEIGHT {
//                 self.set_pixel_unchecked(
//                     (x, y),
//                     function(self.get_pixel_unchecked((x, y))),
//                 );
//             }
//         }
//     }
//     #[must_use]
//     #[inline(always)]
//     /// Get the current size in a tuple
//     pub const fn get_size(&self) -> (usize, usize) {
//         (WIDTH, HEIGHT)
//     }
//     /// A steepness of 15 and offset of 0.8 makes a nice looking icon (Rough estimates based on trail and error)
//     pub fn fade_out_edges(&mut self, steepness: f32, offset: f32) {
//         let cx = WIDTH as f32 / 2.0;
//         let cy = HEIGHT as f32 / 2.0;
//         let max_dist = cx.hypot(cy);

//         for y in 0..HEIGHT {
//             for x in 0..WIDTH {
//                 let dx = x as f32 - cx;
//                 let dy = y as f32 - cy;
//                 let dist = dx.hypot(dy) / max_dist;
//                 let fade = 1.0 - dist;
//                 let fade =
//                     1.0 - crate::math::smooth_0_to_1(fade, steepness, offset);
//                 unsafe {
//                     let color = *self.data.as_ptr().add(y * WIDTH + x);
//                     *self.data.as_mut_ptr().add(y * WIDTH + x) = color
//                         .with_alpha(crate::math::interpolate(
//                             0_f32, 255_f32, fade,
//                         ) as u32);
//                 };
//             }
//         }
//     }
// }
