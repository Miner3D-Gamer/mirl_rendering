// #![allow(clippy::inline_always)]
// use mirl_core::Buffer;

// impl Buffer {
//     #[must_use]
//     #[inline(always)]
//     /// Simple function checking if pixel a coordinate falls within the buffer metrics
//     pub const fn is_pixel_position_in_buffer(
//         &self,
//         x: usize,
//         y: usize,
//     ) -> bool {
//         x < self.width && y < self.height
//     }
//     #[must_use]
//     #[inline(always)]
//     #[allow(clippy::cast_sign_loss)]
//     /// Simple function checking if pixel a possibly negative coordinate falls within the buffer metrics
//     pub const fn is_pixel_position_in_buffer_isize(
//         &self,
//         x: isize,
//         y: isize,
//     ) -> bool {
//         // Really? Checking if the value is positive after casting to usize? Good thing there is no record of this code
//         x > 0
//             && y > 0
//             && (x as usize) < self.width
//             && (y as usize) < self.height
//     }
// }
