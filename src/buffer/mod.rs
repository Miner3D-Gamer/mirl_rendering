// #[cfg(feature = "serde")]
// impl<'de> serde::Deserialize<'de> for Buffer {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         Err(D::)
//     }
// }

mod collision_support;
mod draw;
mod get_converted;
// mod get_pixel;
mod manipulate;
mod misc;
// mod new;
mod set_color;
mod set_pixel;
mod trim;

// impl Clone for Buffer {
//     #[allow(clippy::as_ptr_cast_mut)]
//     fn clone(&self) -> Self {
//         let data = self.data.clone();

//         Self {
//             data,
//             width: self.width,
//             height: self.height,
//             total_size: self.total_size,
//         }
//     }
// }
