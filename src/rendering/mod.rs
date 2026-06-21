pub use mirl_buffer::{draw_pixel_safe, draw_pixel_unsafe};

// type DrawPixelFunction =
//     fn(&mut (impl BufferPointers + BufferMetrics), (usize, usize), u32);

/// All buffer-type related traits
pub mod buffer_type;
// pub use buffer_type::*;
// #[cfg(feature = "std")]
// /// Helpers that should make using stuff like Arc easier
// pub mod buffer_compatibility;
// pub use buffer_compatibility::*;

// macro_rules! create_safe_and_unsafe {
//     (
//         fn $fn:ident($($arg:ident: $arg_ty:ty),*) $body:block
//     ) => {
//         // Safe version
//         pub fn $fn($($arg: $arg_ty),*) {
//             paste::paste! {
//                 [<$fn _impl>]($($arg),*, draw_pixel_safe);
//             }
//         }

//         // Unsafe (fast) version
//         paste::paste! {
//             pub fn [<$fn _unsafe>]($($arg: $arg_ty),*) {
//                 [<$fn _impl>]($($arg),*, draw_pixel_unsafe);
//             }
//         }

//         // Implementation function
//         paste::paste! {
//             fn [<$fn _impl>]($($arg: $arg_ty),*) {
//                 $body
//             }
//         }
//     };
// }

// #[derive(Copy, Clone)]
// #[cfg_attr(feature = "c_compatible", repr(C))] pub enum Safety {
//     Safe,
//     Unsafe,
// }

// #[derive(Copy, Clone)]
// #[cfg_attr(feature = "c_compatible", repr(C))] pub enum Quality {
//     Fast,
//     Pretty,
// }

mod circle_outline;
pub use circle_outline::*;
#[cfg(feature = "font_support")]
mod text;
#[cfg(feature = "font_support")]
pub use text::*;
mod line;
pub use line::*;
mod circle;
pub use circle::*;
mod rectangle;
pub use rectangle::*;
#[cfg(feature = "std")]
mod triangle;
#[cfg(feature = "std")]
pub use triangle::*;
mod texture;
pub use texture::*;
