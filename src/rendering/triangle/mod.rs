use mirl_buffer::Buffer;

use crate::extra::uv_interpolate;
/// This sucks, it works but it sucks
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
pub fn draw_triangle(
    buffer: &mut Buffer,
    width: usize,
    height: usize,
    point1: (isize, isize, f32, f32),
    point2: (isize, isize, f32, f32),
    point3: (isize, isize, f32, f32),
    texture: &Buffer,
) {
    // Sort vertices by Y coordinate
    let mut vertices = [point1, point2, point3];
    vertices.sort_by_key(|a| a.1);

    let (x1, y1, u1, v1) = (vertices[0].0, vertices[0].1, vertices[0].2, vertices[0].3);
    let (x2, y2, u2, v2) = (vertices[1].0, vertices[1].1, vertices[1].2, vertices[1].3);
    let (x3, y3, u3, v3) = (vertices[2].0, vertices[2].1, vertices[2].2, vertices[2].3);

    let mut x_start;
    let mut x_end;
    let mut u_start;
    let mut u_end;
    let mut v_start;
    let mut v_end;

    for y in y1..=y3 {
        // Skip if y is out of bounds
        if y >= height as isize {
            break;
        }

        if y < y2 {
            // Top half of triangle
            x_start = uv_interpolate(y as f32, y1 as f32, x1 as f32, y2 as f32, x2 as f32);
            x_end = uv_interpolate(y as f32, y1 as f32, x1 as f32, y3 as f32, x3 as f32);

            u_start = uv_interpolate(y as f32, y1 as f32, u1, y2 as f32, u2);
            u_end = uv_interpolate(y as f32, y1 as f32, u1, y3 as f32, u3);

            v_start = uv_interpolate(y as f32, y1 as f32, v1, y2 as f32, v2);
        } else {
            // Bottom half of triangle
            x_start = uv_interpolate(y as f32, y2 as f32, x2 as f32, y3 as f32, x3 as f32);
            x_end = uv_interpolate(y as f32, y1 as f32, x1 as f32, y3 as f32, x3 as f32);

            u_start = uv_interpolate(y as f32, y2 as f32, u2, y3 as f32, u3);
            u_end = uv_interpolate(y as f32, y1 as f32, u1, y3 as f32, u3);

            v_start = uv_interpolate(y as f32, y2 as f32, v2, y3 as f32, v3);
        }
        v_end = uv_interpolate(y as f32, y1 as f32, v1, y3 as f32, v3);

        // Ensure correct ordering (left to right)
        if x_start > x_end {
            std::mem::swap(&mut x_start, &mut x_end);
            std::mem::swap(&mut u_start, &mut u_end);
            std::mem::swap(&mut v_start, &mut v_end);
        }

        let x_start_i = x_start.floor() as i32;
        let x_end_i = (x_end.floor() + 1.0) as i32;

        for x in x_start_i..x_end_i {
            // Bounds check
            if x < 0 || x >= width as i32 {
                continue;
            }
            let x = x as isize;

            // Interpolate texture coordinates
            let t = if (x_start - x_end).abs() < 0.000_000_000_1 {
                0.0
            } else {
                (x as f32 - x_start) / (x_end - x_start)
            };

            let u = t.mul_add(u_end - u_start, u_start);
            let v = t.mul_add(v_end - v_start, v_start);

            // Clamp texture coordinates to [0, 1]
            let u = u.clamp(0.0, 1.0);
            let v = v.clamp(0.0, 1.0);

            // Convert to texture pixel coordinates
            let tex_x = (u * (texture.width - 1) as f32) as usize;
            let tex_y = (v * (texture.height - 1) as f32) as usize;

            // Safe texture sampling
            let tex_index = tex_y * texture.width + tex_x;
            let index = y * width as isize + x;
            if index < 0 {
                continue;
            }
            if let Some(&color) = texture.data.get(tex_index) {
                unsafe {
                    *buffer.mut_pointer().add(index as usize) = color;
                }
            }
        }
    }
}
