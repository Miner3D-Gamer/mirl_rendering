use mirl_buffer::ConstBuffer;
/// A temporary solution to const buffer rotation
pub trait TempConstBufferRotate {
    /// Normally rotated
    type Normal;
    /// Flipped by 90° or -90°
    type Rotated;
    #[must_use]
    /// Rotate the `ConstBuffer` 90°
    fn rotated_90(&self) -> Self::Rotated;
    #[must_use]
    /// Rotate the `ConstBuffer` 180°
    fn rotated_180(&self) -> Self::Normal;
    #[must_use]
    /// Rotate the `ConstBuffer` 270° (or -90°)
    fn rotated_270(&self) -> Self::Rotated;
}

impl<const WIDTH: usize, const HEIGHT: usize> TempConstBufferRotate for ConstBuffer<WIDTH, HEIGHT>
where
    [(); WIDTH * HEIGHT]:,
    [(); HEIGHT * WIDTH]:,
{
    type Normal = Self;
    type Rotated = ConstBuffer<HEIGHT, WIDTH>;
    /// Rotate the `ConstBuffer` 90°
    fn rotated_90(&self) -> ConstBuffer<HEIGHT, WIDTH>
    where
        [(); WIDTH * HEIGHT]:,
        [(); HEIGHT * WIDTH]:,
    {
        let mut result: ConstBuffer<HEIGHT, WIDTH> = ConstBuffer::new_empty();

        unsafe {
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let src_pixel = *self.data.as_ptr().add(y * WIDTH + x);
                    // For 90° clockwise: new_x = old_y, new_y = width - 1 - old_x
                    let new_x = y;
                    let new_y = WIDTH - 1 - x;
                    let dst_idx = new_y * HEIGHT + new_x;
                    *result.data.as_mut_ptr().add(dst_idx) = src_pixel;
                }
            }
        }

        result
    }
    /// Rotate the `ConstBuffer` 180°
    fn rotated_180(&self) -> Self {
        let mut result = Self::new_empty();

        unsafe {
            for i in 0..Self::TOTAL_SIZE {
                let src_pixel = *self.data.as_ptr().add(i);
                let dst_idx = Self::TOTAL_SIZE - 1 - i;
                *result.data.as_mut_ptr().add(dst_idx) = src_pixel;
            }
        }

        result
    }
    /// Rotate the `ConstBuffer` 270° (or -90°)
    fn rotated_270(&self) -> ConstBuffer<HEIGHT, WIDTH>
    where
        [(); WIDTH * HEIGHT]:,
        [(); HEIGHT * WIDTH]:,
    {
        let mut result: ConstBuffer<HEIGHT, WIDTH> = ConstBuffer::new_empty();

        unsafe {
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let src_pixel = *self.data.as_ptr().add(y * WIDTH + x);
                    let new_x = HEIGHT - 1 - y;
                    let new_y = x;
                    let dst_idx = new_y * HEIGHT + new_x;
                    *result.data.as_mut_ptr().add(dst_idx) = src_pixel;
                }
            }
        }

        result
    }
}
