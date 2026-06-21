// I'm not even gonna pretend I know what any of this means
#[allow(unused_imports)]
#[cfg(not(feature = "std"))]
use mirl_extensions::*;

/// Rotate a [`Vertex3D`] around a [`Point3D`] on the x axis
pub fn rotate_x_vertex_3d(
    angle_degrees: f64,
    rotation_center: Point3D,
    vertex: &mut Vertex3D,
) {
    let angle_radians = angle_degrees.to_radians();
    let cos_a = angle_radians.cos();
    let sin_a = angle_radians.sin();
    let (cx, cy, cz) =
        (rotation_center.x, rotation_center.y, rotation_center.z);

    // Adjust the vertex relative to the rotation center
    let x = vertex.x - cx;
    let y = vertex.y - cy;
    let z = vertex.z - cz;

    // Apply the rotation
    let new_y = cos_a.mul_add(y, -(sin_a * (z)));
    let new_z = sin_a.mul_add(y, cos_a * (z));

    // Adjust the vertex back
    vertex.x = x + cx;
    vertex.y = new_y + cy;
    vertex.z = new_z + cz;
}
/// Rotate a [Polygon] around a [`Point3D`] on the x axis
pub fn rotate_x_polygon_3d(
    angle_degrees: f64,
    rotation_center: Point3D,
    polygon: &mut Polygon,
) {
    rotate_x_vertex_3d(angle_degrees, rotation_center, &mut polygon.point1);
    rotate_x_vertex_3d(angle_degrees, rotation_center, &mut polygon.point2);
    rotate_x_vertex_3d(angle_degrees, rotation_center, &mut polygon.point3);
}
/// Rotate a [Polygon] around a [`Point3D`] on the y axis
pub fn rotate_y_polygon_3d(
    angle_degrees: f64,
    rotation_center: Point3D,
    polygon: &mut Polygon,
) {
    rotate_y_vertex_3d(angle_degrees, rotation_center, &mut polygon.point1);
    rotate_y_vertex_3d(angle_degrees, rotation_center, &mut polygon.point2);
    rotate_y_vertex_3d(angle_degrees, rotation_center, &mut polygon.point3);
}
/// Rotate a [Polygon] around a [`Point3D`] on the z axis
pub fn rotate_z_polygon_3d(
    angle_degrees: f64,
    rotation_center: Point3D,
    polygon: &mut Polygon,
) {
    rotate_z_vertex_3d(angle_degrees, rotation_center, &mut polygon.point1);
    rotate_z_vertex_3d(angle_degrees, rotation_center, &mut polygon.point2);
    rotate_z_vertex_3d(angle_degrees, rotation_center, &mut polygon.point3);
}

/// Rotate a [`Vertex3D`] around a [`Point3D`] on the y axis
pub fn rotate_y_vertex_3d(
    angle_degrees: f64,
    rotation_center: Point3D,
    vertex: &mut Vertex3D,
) {
    let angle_radians = (angle_degrees).to_radians();
    let cos_a = angle_radians.cos();
    let sin_a = angle_radians.sin();
    let (cx, cy, cz) =
        (rotation_center.x, rotation_center.y, rotation_center.z);

    // Adjust the vertex relative to the rotation center
    let x = vertex.x - cx;
    let y = vertex.y - cy;
    let z = vertex.z - cz;

    // Apply the rotation
    let new_x = cos_a.mul_add(x, sin_a * (z));
    let new_z = (-sin_a).mul_add(x, cos_a * (z));

    // Adjust the vertex back
    vertex.x = new_x + cx;
    vertex.y = y + cy;
    vertex.z = new_z + cz;
}

/// Rotate a [`Vertex3D`] around a [`Point3D`] on the z axis
pub fn rotate_z_vertex_3d(
    angle_degrees: f64,
    rotation_center: Point3D,
    vertex: &mut Vertex3D,
) {
    let angle_radians = (angle_degrees).to_radians();
    let cos_a = angle_radians.cos();
    let sin_a = angle_radians.sin();
    let (cx, cy, cz) =
        (rotation_center.x, rotation_center.y, rotation_center.z);

    // Adjust the vertex relative to the rotation center
    let x = vertex.x - cx;
    let y = vertex.y - cy;
    let z = vertex.z - cz;

    // Apply the rotation
    let new_x = cos_a.mul_add(x, -(sin_a * (y)));
    let new_y = sin_a.mul_add(x, cos_a * (y));

    // Adjust the vertex back
    vertex.x = new_x + cx;
    vertex.y = new_y + cy;
    vertex.z = z + cz;
}
/// Set a pixel color at the specified coordinate while checking if the position is validly in the buffer range
#[inline(always)]
#[allow(clippy::inline_always)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn set_pixel_safe(
    buffer: *mut u32,
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    color: u32,
) {
    if x >= width || y >= height {
        return;
    }
    unsafe {
        *buffer.add(y * width + x) = color;
    }
}
/// Set a pixel color at the specified coordinate without checking if the position is validly in the buffer range
#[inline(always)]
#[allow(clippy::inline_always)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn set_pixel_unchecked(
    buffer: *mut u32,
    width: usize,
    x: usize,
    y: usize,
    color: u32,
) {
    unsafe {
        *buffer.add(y * width + x) = color;
    }
}
/// Interpolate between uv values
#[inline]
#[must_use]
#[allow(clippy::float_cmp)]
pub fn uv_interpolate(
    target_y: f32,
    start_y: f32,
    start_val: f32,
    end_y: f32,
    end_val: f32,
) -> f32 {
    if start_y == end_y {
        return start_val;
    }
    start_val + (end_val - start_val) * (target_y - start_y) / (end_y - start_y)
}
#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
/// A polygon - Created using 3 [`Vertex3D`]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Polygon {
    /// First [`Vertex3D`]
    pub point1: Vertex3D,
    /// Second [`Vertex3D`]
    pub point2: Vertex3D,
    /// Third [`Vertex3D`]
    pub point3: Vertex3D,
}
#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
/// A 3D point in space with uv coordinates
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vertex3D {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
    /// X coordinate
    pub z: f64,
    /// Texture x coordinate
    pub u: f32,
    /// Texture y coordinate
    pub v: f32,
}
#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
/// A 3D point in space without uv coordinates
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point3D {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
    /// X coordinate
    pub z: f64,
}

#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
/// A 2D point in space without uv coordinates
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point2D {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
}
#[must_use]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
/// Cast a 3d point into 2d space using a very simple algorithm
pub fn vertex_3d_to_2d(
    vertex: &Vertex3D,
    width: usize,
    height: usize,
) -> (isize, isize) {
    //return (vertex.x as u16, vertex.y as u16);
    let half_width = (width / 2) as f64;
    let half_height = (height / 2) as f64;
    let x = vertex.x;
    let y = vertex.y;
    let z = vertex.z;

    let screen_x = (x - half_width) / z + half_width;
    let screen_y = (y - half_height) / z + half_height;
    (screen_x as isize, screen_y as isize)
}
// fn useable(
//     p: &Vertex3D,
//     width: usize,
//     height: usize,
// ) -> (isize, isize, f32, f32) {
//     let position = vertex_3d_to_2d(p, width, height);

//     return (position.0, position.1, p.u, p.v);
// }
