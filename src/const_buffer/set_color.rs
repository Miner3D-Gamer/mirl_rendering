// use super::ConstBuffer;
// use crate::graphics::get_alpha_of_u32_in_u8;
// impl<const WIDTH: usize, const HEIGHT: usize> ConstBuffer<WIDTH, HEIGHT>
// where
//     [(); WIDTH * HEIGHT]:,
// {
//     /// Replaces all data with zeros - very fast
//     pub const fn clear(&mut self) {
//         unsafe {
//             core::ptr::write_bytes(self.data.as_mut_ptr(), 0, Self::TOTAL_SIZE);
//         }
//     }
//     /// Replaces all data with a flat color
//     /// An alternative function would be [`Self::clear_buffer_with_color_sliced`] with an approximate, yet not guaranteed, 10% speed increase
//     pub fn clear_buffer_with_color(&mut self, color: u32) {
//         for idx in 0..Self::TOTAL_SIZE {
//             unsafe {
//                 *self.data.as_mut_ptr().add(idx) = color;
//             }
//         }
//     }
//     #[cfg(feature = "std")]
//     /// Replaces all data with a flat color
//     /// Not yet been properly tested yet roughly 10% faster than [`Self::clear_buffer_with_color`]
//     pub fn clear_buffer_with_color_sliced(&mut self, color: u32) {
//         let slice = unsafe {
//             std::slice::from_raw_parts_mut(
//                 self.data.as_mut_ptr(),
//                 Self::TOTAL_SIZE,
//             )
//         };
//         slice.fill(color);
//     }
//     /// Replaces all color with alpha 0 to the given color
//     /// An alternative function would be [`Self::replace_transparent_with_color_chunked`] with an approximate, yet not guaranteed, 30% speed increase
//     pub fn replace_transparent_with_color(&mut self, color: u32) {
//         for idx in 0..Self::TOTAL_SIZE {
//             unsafe {
//                 if get_alpha_of_u32_in_u8(*self.as_ptr().add(idx)) == 0 {
//                     *self.data.as_mut_ptr().add(idx) = color;
//                 }
//             }
//         }
//     }
//     /// Replaces all color with alpha 0 to the given color
//     /// Not yet been properly tested yet roughly 0-33% faster than [`Self::replace_transparent_with_color`]
//     pub fn replace_transparent_with_color_chunked(&mut self, color: u32) {
//         const CHUNK_SIZE: usize = 1024; // Tune based on cache size

//         for chunk in self.data.chunks_mut(CHUNK_SIZE) {
//             for pixel in chunk {
//                 if (*pixel & 0xFF00_0000) == 0 {
//                     *pixel = color;
//                 }
//             }
//         }
//     }
// }
