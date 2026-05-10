use mirl_core::{
    // graphics::u32_to_rgba_u8,
    render::traits::{BufferData, BufferMetrics},
};

// TODO:
// Make sure that the buffer size is a valid cursor size
// Automatically create smaller cursor sizes when possible so windows can choose which one to use
// Add support for .ani files (Animated .cur files)
use crate::prelude::FlipBuffer;

/// Create a bitmap from the buffer
pub const trait BufferToBmp {
    /// Create a bitmap from the buffer
    fn create_bmp(self) -> Option<Vec<u8>>;
}
impl<S: BufferData + BufferMetrics + FlipBuffer> BufferToBmp for S {
    /// Convert the image the buffer is holding into a bmp
    fn create_bmp(self) -> Option<Vec<u8>> {
        mirl_core::render::formats::create_bmp(self.data(), self.get_size())
        // if self.width() > i32::MAX as usize || self.height() > i32::MAX as usize
        // {
        //     return None;
        // }
        // let mut bmp_buffer: Vec<u8> = Vec::new();

        // let width: u32 = self.width() as u32;
        // let height: u32 = self.height() as u32;

        // // BMP File Header (14 bytes)
        // bmp_buffer.extend(&[0x42, 0x4D]); // "BM" signature

        // let row_stride = (width * 32).div_ceil(32) * 4;
        // let pixel_array_size = row_stride * height;
        // let bmp_header_size = 40;
        // let file_header_size = 14;
        // let file_size = file_header_size + bmp_header_size + pixel_array_size;
        // let pixel_data_offset = file_header_size + bmp_header_size;

        // bmp_buffer.extend(&file_size.to_le_bytes()); // File size
        // bmp_buffer.extend(&[0x00, 0x00]); // Reserved
        // bmp_buffer.extend(&[0x00, 0x00]); // Reserved
        // bmp_buffer.extend(&pixel_data_offset.to_le_bytes()); // Pixel data offset

        // // BITMAPINFOHEADER (40 bytes)
        // bmp_buffer.extend(&40u32.to_le_bytes()); // Header size
        // bmp_buffer.extend(&(width as i32).to_le_bytes()); // Width
        // bmp_buffer.extend(&(height as i32).to_le_bytes()); // Height (no x2 for BMP)
        // bmp_buffer.extend(&1u16.to_le_bytes()); // Planes
        // bmp_buffer.extend(&32u16.to_le_bytes()); // Bit count
        // bmp_buffer.extend(&0u32.to_le_bytes()); // Compression
        // bmp_buffer.extend(&pixel_array_size.to_le_bytes()); // Image size
        // bmp_buffer.extend(&0u32.to_le_bytes()); // X pixels per meter
        // bmp_buffer.extend(&0u32.to_le_bytes()); // Y pixels per meter
        // bmp_buffer.extend(&0u32.to_le_bytes()); // Colors used
        // bmp_buffer.extend(&0u32.to_le_bytes()); // Important colors

        // let mut image = self;
        // image.flip_vertically();

        // // Pixel data (BGR + Alpha format)
        // for pixel in image.data() {
        //     let (r, g, b, a) = u32_to_rgba_u8(*pixel);
        //     #[allow(clippy::tuple_array_conversions)]
        //     bmp_buffer.extend(&[b, g, r, a]);
        // }

        // Some(bmp_buffer)
    }
}

/// Create a windows .ico file from the buffer
pub const trait BufferToIco {
    /// Create a bitmap from the buffer
    fn create_ico(self, hotspot: (u16, u16)) -> Option<Vec<u8>>;
}
impl<S: BufferData + BufferMetrics + FlipBuffer> BufferToIco for S {
    fn create_ico(self, hotspot: (u16, u16)) -> Option<Vec<u8>> {
        mirl_core::render::formats::create_cur_simple(
            self.data(),
            self.get_size(),
            hotspot,
        )
        // if self.width() > u8::MAX as usize || self.height() > u8::MAX as usize {
        //     return None;
        // }
        // let mut ico_buffer: Vec<u8> = Vec::new();

        // let width = self.width() as u8;
        // let height = self.height() as u8;

        // // ICONDIR (6 bytes)
        // ico_buffer.extend(&[0x00, 0x00]); // Reserved
        // ico_buffer.extend(&[0x01, 0x00]); // Image type (1 = icon, not 2)
        // ico_buffer.extend(&[0x01, 0x00]); // Number of images

        // // ICONDIRENTRY (16 bytes)
        // ico_buffer.push(width); // Width
        // ico_buffer.push(height); // Height
        // ico_buffer.push(0); // Color count
        // ico_buffer.push(0); // Reserved
        // ico_buffer.extend(&[0x00, 0x00]); // Color planes (0 for icons)
        // ico_buffer.extend(&[0x20, 0x00]); // Bits per pixel (32)

        // let image_data_offset = 6 + 16;
        // let row_stride = (u32::from(width) * 32).div_ceil(32) * 4;
        // let pixel_array_size = row_stride * u32::from(height);
        // let bmp_header_size = 40;
        // let and_mask_size =
        //     u32::from(height) * (u32::from(width).div_ceil(32) * 4);
        // let size_in_bytes = bmp_header_size + pixel_array_size + and_mask_size;

        // ico_buffer.extend(&size_in_bytes.to_le_bytes()); // Image size
        // ico_buffer.extend(&(image_data_offset as u32).to_le_bytes()); // Image offset

        // // BITMAPINFOHEADER (40 bytes)
        // let mut bmp_data: Vec<u8> = Vec::with_capacity(size_in_bytes as usize);
        // bmp_data.extend(&40u32.to_le_bytes()); // Header size
        // bmp_data.extend(&i32::from(width).to_le_bytes()); // Width
        // bmp_data.extend(&(2 * i32::from(height)).to_le_bytes()); // Height (x2 for AND mask)
        // bmp_data.extend(&1u16.to_le_bytes()); // Planes
        // bmp_data.extend(&32u16.to_le_bytes()); // Bit count
        // bmp_data.extend(&0u32.to_le_bytes()); // Compression
        // bmp_data.extend(&0u32.to_le_bytes()); // Image size
        // bmp_data.extend(&0u32.to_le_bytes()); // X pixels per meter
        // bmp_data.extend(&0u32.to_le_bytes()); // Y pixels per meter
        // bmp_data.extend(&0u32.to_le_bytes()); // Colors used
        // bmp_data.extend(&0u32.to_le_bytes()); // Important colors

        // let mut image = self;
        // image.flip_vertically();

        // // Pixel data
        // for pixel in image.data() {
        //     let (r, g, b, a) = u32_to_rgba_u8(*pixel);
        //     #[allow(clippy::tuple_array_conversions)]
        //     bmp_data.extend(&[b, g, r, a]);
        // }

        // // AND mask (all zero = fully visible)
        // bmp_data.extend(vec![0u8; and_mask_size as usize]);

        // // Combine all into buffer
        // ico_buffer.extend(bmp_data);

        // Some(ico_buffer)
    }
}
