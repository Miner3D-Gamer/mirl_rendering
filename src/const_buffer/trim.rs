// use super::ConstBuffer;
// use crate::graphics::get_alpha_of_u32;
// impl<const WIDTH: usize, const HEIGHT: usize> ConstBuffer<WIDTH, HEIGHT>
// where
//     [(); WIDTH * HEIGHT]:,
// {
//     // /// Optimizes the image by removing empty space around the image
//     // pub fn remove_margins(&mut self) {
//     //     // Remove all margins in one pass to avoid multiple data copies
//     //     let (top_trim, bottom_trim, left_trim, right_trim) =
//     //         self.calculate_trims();

//     //     if top_trim > 0 || bottom_trim > 0 || left_trim > 0 || right_trim > 0 {
//     //         self.apply_trim(top_trim, bottom_trim, left_trim, right_trim);
//     //     }
//     // }
//     #[must_use]
//     /// Calculates the empty space around the image
//     pub fn calculate_trims(&self) -> (usize, usize, usize, usize) {
//         let mut top_trim = 0;
//         let mut bottom_trim = 0;
//         let mut left_trim = 0;
//         let mut right_trim = 0;

//         // Calculate top trim
//         for row in 0..HEIGHT {
//             if self.is_row_transparent(row) {
//                 top_trim += 1;
//             } else {
//                 break;
//             }
//         }

//         // Calculate bottom trim
//         for row in (0..HEIGHT).rev() {
//             if self.is_row_transparent(row) {
//                 bottom_trim += 1;
//             } else {
//                 break;
//             }
//         }

//         // Calculate left trim
//         for col in 0..WIDTH {
//             if self.is_col_transparent(col) {
//                 left_trim += 1;
//             } else {
//                 break;
//             }
//         }

//         // Calculate right trim
//         for col in (0..WIDTH).rev() {
//             if self.is_col_transparent(col) {
//                 right_trim += 1;
//             } else {
//                 break;
//             }
//         }

//         (top_trim, bottom_trim, left_trim, right_trim)
//     }
//     #[must_use]
//     /// Checks if the requested row only has fully transparent pixels
//     pub fn is_row_transparent(&self, row: usize) -> bool {
//         let start = row * WIDTH;
//         let end = start + WIDTH;
//         self.data[start..end].iter().all(|&pixel| get_alpha_of_u32(pixel) == 0)
//     }
//     #[must_use]
//     /// Checks if the requested column only has fully transparent pixels
//     pub fn is_col_transparent(&self, col: usize) -> bool {
//         (0..HEIGHT)
//             .all(|row| get_alpha_of_u32(self.data[row * WIDTH + col]) == 0)
//     }
//     // /// Trims the image by the given restrictions
//     // pub fn apply_trim(
//     //     &mut self,
//     //     top: usize,
//     //     bottom: usize,
//     //     left: usize,
//     //     right: usize,
//     // ) {
//     //     let new_width = WIDTH - left - right;
//     //     let new_height = HEIGHT - top - bottom;
//     //     let mut new_data = Vec::with_capacity(new_width * new_height);

//     //     for row in top..(HEIGHT - bottom) {
//     //         let row_start = row * WIDTH + left;
//     //         let row_end = row_start + new_width;
//     //         new_data.extend_from_slice(&self.data[row_start..row_end]);
//     //     }

//     //     self.data = new_data;
//     //     WIDTH = new_width;
//     //     HEIGHT = new_height;
//     // }
// }
