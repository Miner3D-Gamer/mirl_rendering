// This file uses the old rendering system

// use crate::graphics::{image_rgba_to_u32, rgb_to_u32, u32_to_rgb};
// use crate::render::{uv_interpolate, vertex_3d_to_2d, Polygon};
// use std::mem;

// use image::{self, GenericImageView};
// pub fn draw_triangle(
//     buffer: &mut Vec<u32>,
//     width: usize,
//     height: usize,
//     point1: (u16, u16, f32, f32),
//     point2: (u16, u16, f32, f32),
//     point3: (u16, u16, f32, f32),
//     texture: &image::DynamicImage,
// ) {
//     // points: screen_x, screen_y, texture_x, texture_y
//     let x1 = point1.0;
//     let y1 = point1.1;
//     let u1 = point1.2;
//     let v1 = point1.3;

//     let x2 = point2.0;
//     let y2 = point2.1;
//     let u2 = point2.2;
//     let v2 = point2.3;

//     let x3 = point3.0;
//     let y3 = point3.1;
//     let u3 = point3.2;
//     let v3 = point3.3;
//     let mut x_start;
//     let mut x_end;
//     let mut u_start;
//     let mut u_end;
//     let mut v_start;
//     let mut v_end;

//     let mut temp;
//     let mut u;
//     let mut v;

//     let mut tex_x;
//     let mut tex_y;
//     let mut color;

//     for y in y1..y3 + 1 {
//         if y < y2 {
//             x_start = uv_interpolate(
//                 y as f32, y1 as f32, x1 as f32, y2 as f32, x2 as f32,
//             );
//             x_end = uv_interpolate(
//                 y as f32 as f32,
//                 y1 as f32,
//                 x1 as f32,
//                 y3 as f32,
//                 x3 as f32,
//             );

//             u_start = uv_interpolate(
//                 y as f32, y1 as f32, u1 as f32, y2 as f32, u2 as f32,
//             );
//             u_end = uv_interpolate(
//                 y as f32, y1 as f32, u1 as f32, y3 as f32, u3 as f32,
//             );

//             v_start = uv_interpolate(
//                 y as f32, y1 as f32, v1 as f32, y2 as f32, v2 as f32,
//             );
//             v_end = uv_interpolate(
//                 y as f32, y1 as f32, v1 as f32, y3 as f32, v3 as f32,
//             );
//         } else {
//             x_start = uv_interpolate(
//                 y as f32, y2 as f32, x2 as f32, y3 as f32, x3 as f32,
//             );
//             x_end = uv_interpolate(
//                 y as f32, y1 as f32, x1 as f32, y3 as f32, x3 as f32,
//             );

//             u_start = uv_interpolate(
//                 y as f32, y2 as f32, u2 as f32, y3 as f32, u3 as f32,
//             );
//             u_end = uv_interpolate(
//                 y as f32, y1 as f32, u1 as f32, y3 as f32, u3 as f32,
//             );

//             v_start = uv_interpolate(
//                 y as f32, y2 as f32, v2 as f32, y3 as f32, v3 as f32,
//             );
//             v_end = uv_interpolate(
//                 y as f32, y1 as f32, v1 as f32, y3 as f32, v3 as f32,
//             );
//         }
//         if x_start > x_end {
//             mem::swap(&mut x_start, &mut x_end);
//             mem::swap(&mut u_start, &mut u_end);
//             mem::swap(&mut v_start, &mut v_end);
//         }
//         for x in x_start.floor() as u16..(x_end.floor() + 1.0) as u16 {
//             if x_start != x_end {
//                 temp = (x as f32 - x_start) / (x_end - x_start)
//             } else {
//                 temp = 0.0
//             }

//             u = u_start + temp * (u_end - u_start);
//             v = v_start + temp * (v_end - v_start);

//             tex_x = top_clamp(
//                 (u * (texture.width() - 1) as f32) as u16,
//                 (texture.width() - 1) as u16,
//             );
//             tex_y = top_clamp(
//                 (v * (texture.height() - 1) as f32) as u16,
//                 (texture.width() - 1) as u16,
//             );
//             color = texture.get_pixel(tex_x as u32, tex_y as u32);
//             set_pixel(
//                 buffer,
//                 width,
//                 height,
//                 x as usize,
//                 y as usize,
//                 image_rgba_to_u32(color),
//             );
//         }
//     }
// }

// pub fn draw_polygon3d(
//     buffer: &mut Vec<u32>,
//     width: usize,
//     height: usize,
//     polygon: Polygon,
//     texture: &image::DynamicImage,
// ) {
//     let (point1_position_x, point1_position_y) =
//         vertex_3d_to_2d(&polygon.point1, width, height);
//     let (point2_position_x, point2_position_y) =
//         vertex_3d_to_2d(&polygon.point2, width, height);
//     let (point3_position_x, point3_position_y) =
//         vertex_3d_to_2d(&polygon.point3, width, height);
//     // print("Points1");
//     // print(point1_position_x.to_string().as_str());
//     // print(point1_position_y.to_string().as_str());
//     // print("Points2");
//     // print(point2_position_x.to_string().as_str());
//     // print(point2_position_y.to_string().as_str());
//     // print("Points3");
//     // print(point3_position_x.to_string().as_str());
//     // print(point3_position_y.to_string().as_str());

//     draw_triangle(
//         buffer,
//         width,
//         height,
//         (
//             point1_position_x,
//             point1_position_y,
//             polygon.point1.u,
//             polygon.point1.v,
//         ),
//         (
//             point2_position_x,
//             point2_position_y,
//             polygon.point2.u,
//             polygon.point2.v,
//         ),
//         (
//             point3_position_x,
//             point3_position_y,
//             polygon.point3.u,
//             polygon.point3.v,
//         ),
//         texture,
//     );
// }

// pub fn dbuffer(
//     buffer: &mut Vec<u32>,
//     width: usize,
//     height: usize,
//     texture_width: u16,
//     texture_height: u16,
//     texture_x: isize,
//     texture_y: isize,
//     texture: &image::DynamicImage,
// ) {
//     let safe_width = texture.width() as f32 - 1.0;
//     let safe_height = texture.height() as f32 - 1.0;

//     // Clamping drawing bounds to the screen
//     let start_x = texture_x.max(0) as u32;
//     let start_y = texture_y.max(0) as u32;
//     let end_x = (texture_x + texture_width as isize).min(width as isize) as u32;
//     let end_y =
//         (texture_y + texture_height as isize).min(height as isize) as u32;

//     for x in start_x..end_x {
//         let texture_uv_x = ((x as isize - texture_x) as f32
//             / texture_width as f32)
//             * safe_width;
//         for y in start_y..end_y {
//             let texture_uv_y = ((y as isize - texture_y) as f32
//                 / texture_height as f32)
//                 * safe_height;

//             let clamped_uv_x = texture_uv_x.min(safe_width).max(0.0);
//             let clamped_uv_y = texture_uv_y.min(safe_height).max(0.0);

//             let pixel =
//                 texture.get_pixel(clamped_uv_x as u32, clamped_uv_y as u32);

//             set_pixel(
//                 buffer,
//                 width,
//                 height,
//                 x as usize,
//                 y as usize,
//                 image_rgba_to_u32(pixel),
//             );
//         }
//     }
// }

// fn buffer_to_image
