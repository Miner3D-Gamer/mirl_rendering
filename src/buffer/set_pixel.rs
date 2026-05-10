// #![allow(clippy::inline_always)]
// use super::Buffer;
// impl Buffer {
//     #[inline(always)]
//     /// Set the pixel at the specified position, it'll check if the pixel is in bounds for you
//     pub const fn set_pixel_safe(&mut self, xy: (usize, usize), color: u32) {
//         crate::draw_pixel_safe(self, xy, color);
//     }
//     #[inline(always)]
//     /// Set the pixel at the specified position without checking if it is within the allowed memory
//     pub const fn set_pixel_unchecked(&mut self, xy: (usize, usize), color: u32) {
//         crate::draw_pixel_unsafe(self, xy, color);
//     }
// }
