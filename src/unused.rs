
#[must_use]
#[cfg(feature = "font_support")]
/// Create a buffer with some pre-placed text
///
/// # Errors
/// When the total visual text width or height exceeds [`usize::MAX`]
pub fn new_with_text(
    string: &str,
    size: usize,
    font: &fontdue::Font,
    text_color: u32,
    background_color: u32,
    antialiased: Option<u8>,
) -> Option<Self> {
    use crate::extensions::*;
    let height = crate::get_text_height(string, size as f32, font);
    let width = crate::get_text_width(string, size as f32, font);
    let mut buffer = Self::new_empty_with_color(
        (height, width).try_tuple_into()?,
        background_color,
    );
    crate::draw_text_switch::<false>(
        &mut buffer,
        string,
        (0, 0),
        text_color,
        size as f32,
        font,
        antialiased,
    );

    Some(buffer)
}
