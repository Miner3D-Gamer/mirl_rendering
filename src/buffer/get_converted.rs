// use mirl::graphics::interpolation::{InterpolationMode, resize_buffer};
// use mirl_core::graphics::u32_to_rgba_u8;
// impl Buffer {
//     #[must_use]
//     /// Converts the [`Vec<u32>`] to [`Vec<8>`] by unpacking the u32 into argb style
//     pub fn to_u8_argb(&self) -> Vec<u8> {
//         let mut return_list = Vec::new();
//         for i in &self.data {
//             let temp = u32_to_rgba_u8(*i);
//             return_list.push(temp.0);
//             return_list.push(temp.1);
//             return_list.push(temp.2);
//             return_list.push(temp.3);
//         }
//         return_list
//     }
//     #[must_use]
//     /// Converts the internal [`Box<[u32]>`](Box<u32>) to [`Vec<8>`] by unpacking the u32 into rgba style
//     pub fn to_u8_rgba(&self) -> Vec<u8> {
//         let mut return_list = Vec::new();
//         for i in &self.data {
//             let temp = u32_to_rgba_u8(*i);
//             return_list.push(temp.3);
//             return_list.push(temp.1);
//             return_list.push(temp.2);
//             return_list.push(temp.0);
//         }
//         return_list
//     }

//     // #[must_use]
//     // /// Creates a new buffer and copies the contents of the current buffer
//     // pub fn resize_content(
//     //     &self,
//     //     size: (usize, usize),
//     //     resizing_method: InterpolationMode,
//     // ) -> Self {
//     //     let mut new = Self::new_empty(size);
//     //     let b = resize_buffer(
//     //         self,
//     //         self.width,
//     //         self.height,
//     //         size.0,
//     //         size.1,
//     //         resizing_method,
//     //     );
//     //     new.data.copy_from_slice(&b);
//     //     new
//     // }
// }
