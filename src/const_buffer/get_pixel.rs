// #![allow(clippy::inline_always)]
// use super::ConstBuffer;
// impl<const WIDTH: usize, const HEIGHT: usize> ConstBuffer<WIDTH, HEIGHT>
// where
//     [(); WIDTH * HEIGHT]:,
// {
//     /// Safely get the pixel color of the `ConstBuffer` at the specified x and y, returns 0 if the pixel is out of bounds
//     /// For a custom return number use [`get_pixel_fallback`](ConstBuffer::get_pixel_fallback)
//     /// For getting the pixel without bounds checking use [`get_pixel_unchecked`](ConstBuffer::get_pixel_unchecked)
//     #[inline(always)]
//     #[must_use]
//     pub const fn get_pixel(&self, xy: (usize, usize)) -> u32 {
//         if xy.0 >= WIDTH || xy.1 >= HEIGHT {
//             return 0;
//         }
//         let index = xy.1 * WIDTH + xy.0;
//         unsafe { *self.data.as_ptr().add(index) }
//     }
//     /// Safely get the pixel color of the `ConstBuffer` at the specified x and y, returns 0 if the pixel is out of bounds
//     /// For a custom return number use [`get_pixel_fallback`](ConstBuffer::get_pixel_fallback)
//     /// For getting the pixel without bounds checking use [`get_pixel_unchecked`](ConstBuffer::get_pixel_unchecked)
//     #[inline(always)]
//     #[must_use]
//     pub const fn get_pixel_option(&self, xy: (usize, usize)) -> Option<u32> {
//         if xy.0 >= WIDTH || xy.1 >= HEIGHT {
//             return None;
//         }
//         let index = xy.1 * WIDTH + xy.0;
//         Some(unsafe { *self.data.as_ptr().add(index) })
//     }
//     /// Safely get the pixel color of the `ConstBuffer` at the specified x and y, returns the fallback input if the pixel is out of bounds
//     #[inline(always)]
//     #[must_use]
//     pub const fn get_pixel_fallback(
//         &self,
//         xy: (usize, usize),
//         fallback: u32,
//     ) -> u32 {
//         if xy.0 >= WIDTH || xy.1 >= HEIGHT {
//             return fallback;
//         }
//         let index = xy.1 * WIDTH + xy.0;
//         unsafe { *self.data.as_ptr().add(index) }
//     }
//     /// Get the pixel color at a position in a `ConstBuffer` without checking if the pixel is on screen (which will crash the program if it isn't)
//     /// The function for getting a pixel safely is [`get_pixel`](ConstBuffer::get_pixel) or [`get_pixel_isize`](ConstBuffer::get_pixel_isize)
//     #[inline(always)]
//     #[must_use]
//     pub const fn get_pixel_unchecked(&self, xy: (usize, usize)) -> u32 {
//         let index = xy.1 * WIDTH + xy.0;
//         unsafe { *self.data.as_ptr().add(index) }
//     }
//     /// Get the pixel color at a position in a `ConstBuffer` yet before that check if it is in the range of the `ConstBuffer`
//     #[inline(always)]
//     #[must_use]
//     #[allow(clippy::cast_sign_loss)]
//     pub const fn get_pixel_isize(&self, xy: (isize, isize)) -> u32 {
//         if xy.0 < 0 || xy.1 < 0 {
//             return 0;
//         }
//         let y = xy.1 as usize;
//         let x = xy.0 as usize;
//         if x >= WIDTH || y >= HEIGHT {
//             return 0;
//         }
//         let index = y * WIDTH + x;
//         self.data[index]
//     }
//     /// Get the pixel color at a position in a `ConstBuffer` yet before that check if it is in the range of the `ConstBuffer`
//     #[inline(always)]
//     #[must_use]
//     #[allow(clippy::cast_sign_loss)]
//     pub const fn get_pixel_isize_option(
//         &self,
//         xy: (isize, isize),
//     ) -> Option<u32> {
//         if xy.0 < 0 || xy.1 < 0 {
//             return None;
//         }
//         let y = xy.1 as usize;
//         let x = xy.0 as usize;
//         if x >= WIDTH || y >= HEIGHT {
//             return None;
//         }
//         let index = y * WIDTH + x;
//         Some(self.data[index])
//     }
//     /// Get the pixel color at a position in a `ConstBuffer` yet before that check if it is in the range of the `ConstBuffer`
//     /// Instead of returning None if the result isn't in the `ConstBuffer`, it will return the specified fallback value
//     #[inline(always)]
//     #[must_use]
//     #[allow(clippy::cast_sign_loss)]
//     pub const fn get_pixel_isize_fallback(
//         &self,
//         xy: (isize, isize),
//         fallback: u32,
//     ) -> u32 {
//         if xy.0 < 0 || xy.1 < 0 {
//             return fallback;
//         }
//         let y = xy.1 as usize;
//         let x = xy.0 as usize;
//         if x >= WIDTH || y >= HEIGHT {
//             return fallback;
//         }
//         let index = y * WIDTH + x;
//         self.data[index]
//     }
// }
